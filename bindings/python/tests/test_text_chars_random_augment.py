from __future__ import annotations

import pytest

from fast_aug.text import (
    BaseTextAugmenter,
    CharsRandomDeleteAugmenter,
    CharsRandomInsertAugmenter,
    CharsRandomSubstituteAugmenter,
    CharsRandomSwapAugmenter,
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
@pytest.mark.parametrize(
    "char_params",
    [
        0.3,
        0.2,
        (0.3, None, None),
        (0.4, None, 10),
        (0.1, 10, None),
        (0.2, 10, 10),
    ],
)
def test_init_word_char_params(
    word_params: float | tuple[float, int | None, int | None], char_params: float | tuple[float, int | None, int | None]
) -> None:
    CharsRandomInsertAugmenter(word_params, char_params, "en")
    CharsRandomSubstituteAugmenter(word_params, char_params, "en")
    CharsRandomDeleteAugmenter(word_params, char_params)
    CharsRandomSwapAugmenter(word_params, char_params)


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
    CharsRandomInsertAugmenter(0.3, 0.3, "en", stopwords=stopwords)
    CharsRandomSubstituteAugmenter(0.3, 0.3, "en", stopwords=stopwords)
    CharsRandomDeleteAugmenter(0.3, 0.3, stopwords=stopwords)
    CharsRandomSwapAugmenter(0.3, 0.3, stopwords=stopwords)


@pytest.mark.parametrize("locale", ["en", "ru_RU", "sr-Cyrl-ME"])
def test_init_locale(locale: str) -> None:
    CharsRandomInsertAugmenter(0.3, 0.3, locale)
    CharsRandomSubstituteAugmenter(0.3, 0.3, locale)


@pytest.mark.parametrize(
    "locale",
    [
        None,
        # "not_existing_locale",  # TODO: fix, error not raised
        "en_US_US",
    ],
)
def test_init_locale_error(locale: str | None) -> None:
    with pytest.raises(Exception):
        CharsRandomInsertAugmenter(0.3, 0.3, locale)
    with pytest.raises(Exception):
        CharsRandomSubstituteAugmenter(0.3, 0.3, locale)


@pytest.mark.parametrize(
    "augmenter",
    [
        CharsRandomInsertAugmenter(0.3, 0.3, "en"),
        CharsRandomSubstituteAugmenter(0.3, 0.3, "en"),
        CharsRandomDeleteAugmenter(0.3, 0.3),
        CharsRandomSwapAugmenter(0.3, 0.3),
    ],
)
def test_input_changes(augmenter: BaseTextAugmenter) -> None:
    texts = [
        "word some test",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ]

    for text in texts:
        output = augmenter.augment(text)
        assert text != output


@pytest.mark.parametrize(
    "augmenter",
    [
        CharsRandomInsertAugmenter(0.3, 0.3, "en"),
        CharsRandomSubstituteAugmenter(0.3, 0.3, "en"),
        CharsRandomDeleteAugmenter(0.3, 0.3),
        CharsRandomSwapAugmenter(0.3, 0.3),
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
