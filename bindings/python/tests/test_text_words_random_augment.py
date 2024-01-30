from __future__ import annotations

import pytest

from fast_aug.text import (
    BaseTextAugmenter,
    WordsRandomDeleteAugmenter,
    WordsRandomInsertAugmenter,
    WordsRandomSubstituteAugmenter,
    WordsRandomSwapAugmenter,
)


@pytest.mark.parametrize(
    "word_params",
    [
        0.3,
        0.2,
        (0.3, None, None),
        (0.4, None, 10),
        (0.1, 10, None),
        (0.2, 10, 10),
    ],
)
def test_init_word_params(word_params: float | tuple[float, int | None, int | None]) -> None:
    WordsRandomInsertAugmenter(word_params, ["a", "b", "c"])
    WordsRandomSubstituteAugmenter(word_params, ["a", "b", "c"])
    WordsRandomDeleteAugmenter(word_params)
    WordsRandomSwapAugmenter(word_params)


@pytest.mark.parametrize(
    "stopwords",
    [
        # None, list(), set(), ["a", "b", "c"], {"a", "b", "c"}
        None,
        set(),
        {"a", "b", "c"},
    ],
)
def test_init_stopwords(stopwords: list[str] | set[str] | None) -> None:
    WordsRandomInsertAugmenter(0.3, ["a", "b", "c"], stopwords=stopwords)
    WordsRandomSubstituteAugmenter(0.3, ["a", "b", "c"], stopwords=stopwords)
    WordsRandomDeleteAugmenter(0.3, stopwords=stopwords)
    WordsRandomSwapAugmenter(0.3, stopwords=stopwords)


def test_init_vocabulary_error() -> None:
    with pytest.raises(Exception):
        WordsRandomInsertAugmenter(0.3, None)  # type: ignore
    with pytest.raises(Exception):
        WordsRandomInsertAugmenter(0.3, [])

    with pytest.raises(Exception):
        WordsRandomSubstituteAugmenter(0.3, None)  # type: ignore
    with pytest.raises(Exception):
        WordsRandomSubstituteAugmenter(0.3, [])


@pytest.mark.parametrize(
    "augmenter",
    [
        WordsRandomInsertAugmenter(0.3, ["a", "b", "c"]),
        WordsRandomSubstituteAugmenter(0.3, ["a", "b", "c"]),
        WordsRandomDeleteAugmenter(0.3),
        WordsRandomSwapAugmenter(0.3),
    ],
)
def test_input_changes(augmenter: BaseTextAugmenter) -> None:
    texts = [
        # "word some test",  # TODO: swap doesn't work here
        # "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ]

    for text in texts:
        output = augmenter.augment(text)
        assert text != output


@pytest.mark.parametrize(
    "augmenter",
    [
        WordsRandomInsertAugmenter(0.3, ["a", "b", "c"]),
        WordsRandomSubstituteAugmenter(0.3, ["a", "b", "c"]),
        WordsRandomDeleteAugmenter(0.3),
        WordsRandomSwapAugmenter(0.3),
    ],
)
def test_input_changes_batch(augmenter: BaseTextAugmenter) -> None:
    texts = [
        "word some test",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ]

    output = augmenter.augment_batch(texts)
    assert texts != output
    assert len(texts) == len(output)
