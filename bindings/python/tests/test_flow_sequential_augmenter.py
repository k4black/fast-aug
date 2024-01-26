from typing import Any

import pytest

from fast_aug.base import BaseAugmenter
from fast_aug.flow import SequentialAugmenter
from fast_aug.text import CharsRandomAugmenter, WordsRandomAugmenter


@pytest.mark.parametrize(
    "augmenters",
    [
        [WordsRandomAugmenter("DELETE", 0.3), CharsRandomAugmenter("DELETE", 0.3)],
        [
            WordsRandomAugmenter("DELETE"),
            CharsRandomAugmenter("DELETE"),
            CharsRandomAugmenter("DELETE"),
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
        [WordsRandomAugmenter("DELETE", 0.3), "not an augmenter"],
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
        WordsRandomAugmenter("DELETE", 0.3),
        CharsRandomAugmenter("DELETE", 0.3),
    ]
    sequential_augmenter = SequentialAugmenter(augmenters)

    assert sequential_augmenter.augment(text) != text


def test_input_changes_batch() -> None:
    augmenters = [
        WordsRandomAugmenter("DELETE", 0.3),
        CharsRandomAugmenter("DELETE", 0.3),
    ]
    sequential_augmenter = SequentialAugmenter(augmenters)
    texts = [
        "word",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ]
    output = sequential_augmenter.augment_batch(texts)
    assert texts != output
    assert len(texts) == len(output)
    for text, out in zip(texts, output):
        assert len(text) >= len(out)
