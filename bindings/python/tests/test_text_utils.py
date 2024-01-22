from enum import Enum

from fast_aug.text import TextAction


def test_text_action() -> None:
    # assert isinstance(TextAction, Enum), f"TextAction is not an Enum, got {type(TextAction)}"

    assert TextAction.DELETE and 'DELETE' in TextAction.__dict__
    assert TextAction.INSERT and 'INSERT' in TextAction.__dict__
    assert TextAction.SWAP and 'SWAP' in TextAction.__dict__
    assert TextAction.SUBSTITUTE and 'SUBSTITUTE' in TextAction.__dict__
