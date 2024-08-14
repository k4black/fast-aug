from pathlib import Path

import pytest
from pytest_examples import CodeExample, EvalExample, find_examples


PYTHON_SOURCE_FOLDER = Path(__file__).parent.parent


@pytest.mark.parametrize("example", find_examples(PYTHON_SOURCE_FOLDER / "README.md"), ids=str)
def test_python_readme(example: CodeExample, eval_example: EvalExample):
    # run and check the example
    # eval_example.lint(example)  # ruff is not working as called from the subprocess
    eval_example.run(example)
    eval_example.run_print_check(example)


@pytest.mark.parametrize("example", find_examples(PYTHON_SOURCE_FOLDER / "fast_aug"), ids=str)
def test_pyi_files(example: CodeExample, eval_example: EvalExample):
    # run and check the example
    # eval_example.lint(example)  # ruff is not working as called from the subprocess
    eval_example.run(example)
    eval_example.run_print_check(example)
