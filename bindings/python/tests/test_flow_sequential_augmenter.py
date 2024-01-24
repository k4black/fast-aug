from typing import Any

import pytest

from fast_aug.base import BaseAugmenter
from fast_aug.flow import SequentialAugmenter
from fast_aug.text import RandomCharsAugmenter, RandomWordsAugmenter


@pytest.mark.parametrize(
    "augmenters",
    [
        [RandomWordsAugmenter("DELETE", 0.3), RandomCharsAugmenter("DELETE", 0.3)],
        [
            RandomWordsAugmenter("DELETE"),
            RandomCharsAugmenter("DELETE"),
            RandomCharsAugmenter("DELETE"),
        ],
    ],
)
def test_init_sequential_augmenter(augmenters: list[BaseAugmenter]) -> None:
    SequentialAugmenter(augmenters)


@pytest.mark.parametrize(
    "augmenters",
    [
        "string",
        None,
        object,
        [1, 2, 3],
        [RandomWordsAugmenter("DELETE", 0.3), "not an augmenter"],
        [],
    ],
)
def test_init_wrong_sequential_augmenter(augmenters: Any) -> None:
    with pytest.raises(Exception):
        SequentialAugmenter(augmenters)


@pytest.mark.parametrize(
    "text",
    [
        "word",
        "Some sentence",
        "A sentence with 5 words!",
        "Two sentences here. This is the second one.",
    ],
)
def test_input_changes(text: str) -> None:
    augmenters = [
        RandomWordsAugmenter("DELETE", 0.3),
        RandomCharsAugmenter("DELETE", 0.3),
    ]
    sequential_augmenter = SequentialAugmenter(augmenters)

    assert sequential_augmenter.augment(text) != text
