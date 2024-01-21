from enum import Enum

from fast_aug.text import TextAction


def test_text_action() -> None:
    # assert isinstance(TextAction, Enum), f"TextAction is not an Enum, got {type(TextAction)}"

    assert TextAction.Delete and 'Delete' in TextAction.__dict__
    assert TextAction.Insert and 'Insert' in TextAction.__dict__
    assert TextAction.Swap and 'Swap' in TextAction.__dict__
    assert TextAction.Substitute and 'Substitute' in TextAction.__dict__

