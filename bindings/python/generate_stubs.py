"""
Adapted from huggingface/tokenizers library:
https://github.com/huggingface/tokenizers/blob/main/bindings/python/stub.py
"""
import argparse
import inspect
from pathlib import Path


INDENT = " " * 4
GENERATED_COMMENT = "# Generated content DO NOT EDIT\n\n"
PYTHON_SOURCE_FOLDER = Path(__file__).parent / "fast_aug"

def do_indent(text: str, indent: str):
    return text.replace("\n", f"\n{indent}")


def function(obj, indent, text_signature=None):
    if text_signature is None:
        text_signature = obj.__text_signature__
    string = ""
    string += f"{indent}def {obj.__name__}{text_signature}:\n"
    indent += INDENT
    string += f'{indent}"""\n'
    string += f"{indent}{do_indent(obj.__doc__, indent)}\n"
    string += f'{indent}"""\n'
    string += f"{indent}pass\n"
    string += "\n"
    string += "\n"
    return string


def member_sort(member):
    if inspect.isclass(member):
        value = 10 + len(inspect.getmro(member))
    else:
        value = 1
    return value


def fn_predicate(obj):
    value = inspect.ismethoddescriptor(obj) or inspect.isbuiltin(obj)
    if value:
        return obj.__doc__ and obj.__text_signature__ and not obj.__name__.startswith("_")
    if inspect.isgetsetdescriptor(obj):
        return obj.__doc__ and not obj.__name__.startswith("_")
    return False


def get_module_members(module):
    members = [
        member
        for name, member in inspect.getmembers(module)
        if not name.startswith("_") and not inspect.ismodule(member)
    ]
    members.sort(key=member_sort)
    return members


def pyi_file(obj, indent=""):
    string = ""
    if inspect.ismodule(obj):
        string += GENERATED_COMMENT
        members = get_module_members(obj)
        for member in members:
            string += pyi_file(member, indent)

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

        # Init
        if obj.__text_signature__:
            body += f"{indent}def __init__{obj.__text_signature__}:\n"
            body += f"{indent+INDENT}pass\n"
            body += "\n"

        for (name, fn) in fns:
            body += pyi_file(fn, indent=indent)

        if not body:
            body += f"{indent}pass\n"

        string += body
        string += "\n\n"

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


def py_file(module, origin):
    members = get_module_members(module)

    string = GENERATED_COMMENT
    string += f"from .. import {origin}\n"
    string += "\n"
    for member in members:
        name = member.__name__
        string += f"{name} = {origin}.{name}\n"
    return string


def generate_stubs_and_imports(
        module: object,
        folder: Path,
        module_name: str,
        check: bool = False,
) -> None:
    folder.mkdir(parents=True, exist_ok=True)

    print("  -", module)

    submodules = [(name, member) for name, member in inspect.getmembers(module) if inspect.ismodule(member)]

    stub_filename = folder / "__init__.pyi"
    relative_stub_filename = stub_filename.relative_to(PYTHON_SOURCE_FOLDER)
    pyi_content = pyi_file(module)
    if check:
        with open(stub_filename, "r") as f:
            data = f.read()
            assert data == pyi_content, \
                f"The content of {relative_stub_filename} seems outdated, please run `python stub.py`"
        print(f"    --> File {relative_stub_filename} is up to date")
    else:
        with open(stub_filename, "w") as f:
            f.write(pyi_content)
        print(f"    --> File {relative_stub_filename} has been regenerated")

    init_filename = folder / "__init__.py"
    relative_init_filename = init_filename.relative_to(PYTHON_SOURCE_FOLDER)
    py_content = py_file(module, module_name)

    is_auto = False
    if not init_filename.exists():
        is_auto = True
    else:
        with open(init_filename, "r") as f:
            line = f.readline()
            if line == GENERATED_COMMENT:
                is_auto = True

    if is_auto:
        if check:
            with (open(init_filename, "r") as f):
                data = f.read()
                assert data == py_content, \
                    f"The content of {relative_init_filename} seems outdated, please run `python generate_stubs.py`"
            print(f"    --> File {relative_init_filename} is up to date")
        else:
            with open(init_filename, "w") as f:
                f.write(py_content)
            print(f"    --> File {relative_init_filename} has been regenerated")
    else:
        print(f"    --> File {relative_init_filename} already exists and is not autogenerated, skipping")

    for name, submodule in submodules:
        generate_stubs_and_imports(submodule, folder / name, str(name), check=check)



if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")

    args = parser.parse_args()

    if not args.check:
        print("Cleaning up old autogenerated files:")
        for path in PYTHON_SOURCE_FOLDER.glob("**/*.py*"):
            try:
                file_content = path.read_text()
            except UnicodeDecodeError:
                file_content = ""
            if file_content.startswith(GENERATED_COMMENT):
                print("  -", path)
                path.unlink()

    import fast_aug

    print("Processing fast_aug:")
    generate_stubs_and_imports(fast_aug.fast_aug, PYTHON_SOURCE_FOLDER, "fast_aug", check=args.check)
