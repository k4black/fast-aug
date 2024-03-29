from typing import Any

import pytest

from fast_aug.base import BaseAugmenter
from fast_aug.flow import ChanceAugmenter
from fast_aug.text import CharsRandomDeleteAugmenter, WordsRandomDeleteAugmenter


@pytest.mark.parametrize("probability", [0.0, 0, 0.3, 0.5, 0.7, 1.0])
def test_init_probability(probability: float) -> None:
    augmenter = WordsRandomDeleteAugmenter(0.3)

    ChanceAugmenter(augmenter, probability)
    ChanceAugmenter(augmenter, probability=probability)


@pytest.mark.parametrize(
    "probability",
    [
        -0.1,
        -0.5,
        -1.0,
        1.1,
        1.5,
        2.0,
        2.5,
        3.0,
        10.0,
        None,
        "0.3",
        "0.5",
        "0.7",
        "1.0",
        "1.5",
        "2.0",
        "2.5",
        "3.0",
        "10.0",
    ],
)
def test_init_wrong_probability(probability: Any) -> None:
    augmenter = WordsRandomDeleteAugmenter(0.3)

    with pytest.raises(Exception):
        ChanceAugmenter(augmenter, probability)


@pytest.mark.parametrize(
    "augmenter",
    [
        WordsRandomDeleteAugmenter(0.3),
        CharsRandomDeleteAugmenter(0.3, 0.3),
    ],
)
def test_init_augmenter(augmenter: BaseAugmenter) -> None:
    ChanceAugmenter(augmenter, 0.3)
    ChanceAugmenter(augmenter=augmenter, probability=0.3)


@pytest.mark.parametrize(
    "augmenter",
    [
        "string",
        None,
        object,
        list(),
    ],
)
def test_init_wrong_augmenter(augmenter: Any) -> None:
    with pytest.raises(Exception):
        ChanceAugmenter(augmenter, 0.3)


@pytest.mark.parametrize(
    "text",
    [
        "word some test",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ],
)
def test_input_changes(text: str) -> None:
    augmenter = WordsRandomDeleteAugmenter(0.3)
    chance_augmenter = ChanceAugmenter(augmenter, 0.5)

    num_of_changes = 0
    for _ in range(100):
        num_of_changes += chance_augmenter.augment(text) != text
    assert 0 < num_of_changes < 100


def test_input_changes_batch() -> None:
    augmenter = WordsRandomDeleteAugmenter(0.3)
    chance_augmenter = ChanceAugmenter(augmenter, 0.5)
    texts = [
        "word some test",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ]

    num_of_changes = 0
    for _ in range(100):
        num_of_changes += chance_augmenter.augment_batch(texts) != texts
    assert 0 < num_of_changes < 100
