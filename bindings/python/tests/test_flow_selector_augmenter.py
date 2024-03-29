from __future__ import annotations

from typing import Any

import pytest

from fast_aug.base import BaseAugmenter
from fast_aug.flow import SelectorAugmenter
from fast_aug.text import CharsRandomDeleteAugmenter, WordsRandomDeleteAugmenter


@pytest.mark.parametrize(
    "augmenters",
    [
        [WordsRandomDeleteAugmenter(0.3), CharsRandomDeleteAugmenter(0.3, 0.3)],
        [
            WordsRandomDeleteAugmenter(0.3),
            CharsRandomDeleteAugmenter(0.3, 0.3),
            CharsRandomDeleteAugmenter(0.3, 0.3),
        ],
    ],
)
def test_init_augmenters(augmenters: list[BaseAugmenter]) -> None:
    SelectorAugmenter(augmenters)


@pytest.mark.parametrize(
    "augmenters",
    [
        [],
        [WordsRandomDeleteAugmenter(0.3)],
        None,
        [object, object],
        [WordsRandomDeleteAugmenter(0.3), "invalid_type"],
    ],
)
def test_init_wrong_augmenters(augmenters: Any) -> None:
    with pytest.raises(Exception):
        SelectorAugmenter(augmenters)


@pytest.mark.parametrize(
    "weights",
    [
        None,
        [0.7, 0.3],
        [0.5, 0.5],
        [0.1, 1.0],
        [1, 1000],
        [-0.1, 1.1],
    ],
)
def test_init_weights(weights: list[float] | None) -> None:
    augmenters = [WordsRandomDeleteAugmenter(0.3), CharsRandomDeleteAugmenter(0.3, 0.3)]
    SelectorAugmenter(augmenters, weights)


@pytest.mark.parametrize(
    "weights",
    [
        [],
        [0.5],
        [0.5, 0.5, 0.5],
        [0.4, object],
        [0.5, "invalid_type"],
        [None, 0.3],
    ],
)
def test_init_wrong_selector(weights: Any) -> None:
    augmenters = [WordsRandomDeleteAugmenter(0.3), CharsRandomDeleteAugmenter(0.3, 0.3)]
    with pytest.raises(Exception):
        SelectorAugmenter(augmenters, weights)


@pytest.mark.parametrize("text", ["word", "Some sentence", "A longer sentence with more words!"])
def test_input_changes(text: str) -> None:
    augmenters = [
        WordsRandomDeleteAugmenter(0.3),
        CharsRandomDeleteAugmenter(0.3, 0.3),
    ]
    selector_augmenter = SelectorAugmenter(augmenters)

    assert selector_augmenter.augment(text) != text


def test_input_changes_batch() -> None:
    augmenters = [
        WordsRandomDeleteAugmenter(0.3),
        CharsRandomDeleteAugmenter(0.3, 0.3),
    ]
    selector_augmenter = SelectorAugmenter(augmenters)
    texts = [
        "word some test",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ]
    output = selector_augmenter.augment_batch(texts)
    assert texts != output
    assert len(texts) == len(output)
    for text, out in zip(texts, output):
        assert len(text) >= len(out)
