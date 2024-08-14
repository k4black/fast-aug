"""
Adapted from huggingface/tokenizers library:
https://github.com/huggingface/tokenizers/blob/main/bindings/python/stub.py
"""

from __future__ import annotations

import argparse
import inspect
from collections.abc import Callable
from modulefinder import Module
from pathlib import Path
from types import GetSetDescriptorType, ModuleType
from typing import Any

import black
import isort
import isort.exceptions


INDENT = " " * 4
GENERATED_COMMENT_LINE = "# Generated content DO NOT EDIT\n"
PYTHON_SOURCE_FOLDER = Path(__file__).parent / "fast_aug"


def do_indent(text: str, indent: str) -> str:
    return text.replace("\n", f"\n{indent}")


def function(
    obj: GetSetDescriptorType | Callable[..., Any],
    indent: str,
    text_signature: str | None = None,
) -> str:
    if text_signature is None:
        text_signature = obj.__text_signature__  # type: ignore
    string = ""
    string += f"{indent}def {obj.__name__}{text_signature}:\n"
    indent += INDENT
    string += f'{indent}"""\n'
    string += f"{indent}{do_indent(obj.__doc__ or '', indent)}\n"
    string += f'{indent}"""\n'
    string += f"{indent}pass\n"
    string += "\n"
    string += "\n"
    return string


def member_sort(member: type) -> int:
    if inspect.isclass(member):
        value = 10 + len(inspect.getmro(member))
    else:
        value = 1
    return value


def fn_predicate(obj: Callable[..., Any]) -> bool:
    if inspect.ismethoddescriptor(obj) or inspect.isbuiltin(obj):
        return bool(obj.__doc__ and obj.__text_signature__ and not obj.__name__.startswith("_"))  # type: ignore
    if inspect.isgetsetdescriptor(obj):
        return bool(obj.__doc__ and not obj.__name__.startswith("_"))
    return False


def get_module_members(module: object) -> list[ModuleType]:
    members = [
        member
        for name, member in inspect.getmembers(module)
        if not name.startswith("_") and not inspect.ismodule(member)
    ]
    members.sort(key=member_sort)
    return members


def pyi_file(obj: ModuleType | Module | Callable[..., Any] | object, indent: str = "") -> str:  # noqa: C901
    string = ""
    if inspect.ismodule(obj):
        string += GENERATED_COMMENT_LINE

        #  pre-read content to check if we need imports
        content = ""
        members = get_module_members(obj)
        for member in members:
            content += pyi_file(member, indent=indent)

        # Add imports
        string += "from __future__ import annotations\n\n"
        if "Any" in content:
            string += "from typing import Any\n\n"
        if "BaseAugmenter" in content and obj.__name__ != "base" and obj.__name__ != "fast_aug":
            string += "from ..base import BaseAugmenter\n\n"

        # Add content
        string += content

    elif inspect.isclass(obj):
        indent += INDENT
        mro = inspect.getmro(obj)
        if len(mro) > 2:
            inherit = f"({mro[1].__name__})"
        else:
            inherit = ""
        string += f"class {obj.__name__}{inherit}:\n"

        body = ""
        if obj.__doc__:
            body += f'{indent}"""\n{indent}{do_indent(obj.__doc__, indent)}\n{indent}"""\n'

        fns = inspect.getmembers(obj, fn_predicate)

        # Init and add docs from new
        if obj.__text_signature__:
            body += f"{indent}def __init__{obj.__text_signature__} -> None:\n"
            body += f"{indent+INDENT}pass\n"
            body += "\n"

        for name, fn in fns:
            body += pyi_file(fn, indent=indent)

        if not body:
            body += f"{indent}pass\n"

        string += body
        string += "\n"

    elif inspect.isbuiltin(obj):
        string += f"{indent}@staticmethod\n"
        string += function(obj, indent)

    elif inspect.ismethoddescriptor(obj):
        string += function(obj, indent)

    elif inspect.isgetsetdescriptor(obj):
        # TODO it would be interesting to add the setter maybe ?
        string += f"{indent}@property\n"
        string += function(obj, indent, text_signature="(self)")

    else:
        raise Exception(f"Object {obj} is not supported")

    return string


def py_file(module: ModuleType, origin: str) -> str:
    members = get_module_members(module)

    string = GENERATED_COMMENT_LINE
    string += f"from .. import {origin}\n"
    string += "\n"
    for member in members:
        name = member.__name__
        string += f"{name} = {origin}.{name}\n"

    # if have __all__ attribute, use it, otherwise generate it
    string += "\n"
    if module.__dict__.get("__all__"):
        string += f"__all__ = {origin}.__all__\n"
    else:
        all_content = ", ".join([f'"{m.__name__}"' for m in members])
        string += f"__all__ = [{all_content}]\n"

    # add docs
    string += f"__doc__ = {origin}.__doc__\n"

    return string


def do_isort(content: str) -> str:
    try:
        return isort.code(
            content,
            config=isort.Config(profile="black", line_length=120, lines_after_imports=2),
        )
    except isort.exceptions.FileSkipComment:
        return content


def do_black(content: str, is_pyi: bool) -> str:
    mode = black.Mode(
        target_versions={black.TargetVersion.PY312},
        line_length=120,
        is_pyi=is_pyi,
        string_normalization=True,
    )
    try:
        return black.format_file_contents(content, fast=True, mode=mode)
    except black.NothingChanged:
        return content


def save_file_if_generated_and_different(
    root: Path, filename: Path, content: str, raise_if_different: bool = False
) -> None:
    relative_filename = filename.relative_to(root)

    # read file content
    if filename.exists():
        is_file_exists = True
        existed_content = filename.read_text()
    else:
        is_file_exists = False
        existed_content = ""

    # check if file is autogenerated
    if is_file_exists and not existed_content.startswith(GENERATED_COMMENT_LINE):
        print(f"    --> File {relative_filename} already exists and is not autogenerated, skipping")
        return

    # check if file is up to date
    if existed_content == content:
        print(f"    --> File {relative_filename} is up to date")
        return

    # raise if different - used for check
    if raise_if_different:
        raise Exception(f"The content of {relative_filename} seems outdated, please run `python generate_stubs.py`")

    # write file
    filename.touch(exist_ok=True)
    filename.write_text(content)
    print(f"    --> File {relative_filename} has been re-generated")


def generate_stubs_and_imports(
    module: ModuleType,
    folder: Path,
    module_name: str,
    check: bool = False,
) -> None:
    folder.mkdir(parents=True, exist_ok=True)

    print("  -", module)

    submodules = [(name, member) for name, member in inspect.getmembers(module) if inspect.ismodule(member)]

    stub_filename = folder / "__init__.pyi"
    pyi_content = pyi_file(module)
    pyi_content = do_isort(pyi_content)
    pyi_content = do_black(pyi_content, is_pyi=True)
    save_file_if_generated_and_different(PYTHON_SOURCE_FOLDER, stub_filename, pyi_content, raise_if_different=check)

    init_filename = folder / "__init__.py"
    py_content = py_file(module, module_name)
    py_content = do_isort(py_content)
    py_content = do_black(py_content, is_pyi=False)
    save_file_if_generated_and_different(PYTHON_SOURCE_FOLDER, init_filename, py_content, raise_if_different=check)

    for name, submodule in submodules:
        generate_stubs_and_imports(submodule, folder / name, str(name), check=check)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")

    args = parser.parse_args()

    # if not args.check:
    #     print("Cleaning up old autogenerated files:")
    #     for path in PYTHON_SOURCE_FOLDER.glob("**/*.py*"):
    #         try:
    #             file_content = path.read_text()
    #         except UnicodeDecodeError:
    #             file_content = ""
    #         if file_content.startswith(GENERATED_COMMENT):
    #             print("  -", path)
    #             path.unlink()

    import fast_aug

    print("Processing fast_aug:")
    generate_stubs_and_imports(
        fast_aug.fast_aug,  # type: ignore
        PYTHON_SOURCE_FOLDER,
        "fast_aug",
        check=args.check,
    )
