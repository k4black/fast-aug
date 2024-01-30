import pytest

from fast_aug.text import CharsRandomAugmenter, TextAction


@pytest.mark.parametrize(
    "action",
    [
        "DELETE",
        "INSERT",
        "SWAP",
        "SUBSTITUTE",
        TextAction.DELETE,
        TextAction.INSERT,
        TextAction.SWAP,
        TextAction.SUBSTITUTE,
    ],
)
def test_init_action(action: str | TextAction) -> None:
    CharsRandomAugmenter(action, locale="en")


def test_init_action_error() -> None:
    with pytest.raises(Exception):
        CharsRandomAugmenter("NOT_EXISTING_ACTION")
    with pytest.raises(Exception):
        CharsRandomAugmenter()
    with pytest.raises(Exception):
        CharsRandomAugmenter(None)


@pytest.mark.parametrize(
    "words_params",
    [
        None,
        0.3,
        0.2,
        (0.3, None, None),
        (0.4, None, 10),
        (0.1, 10, None),
        (0.2, 10, 10),
    ],
)
def test_init_words_params(words_params: float | tuple[float, int | None, int | None] | None) -> None:
    CharsRandomAugmenter(TextAction.DELETE, words_params)
    CharsRandomAugmenter(TextAction.DELETE, words_params, None)
    CharsRandomAugmenter(TextAction.DELETE, word_params=words_params)


@pytest.mark.parametrize(
    "chars_params",
    [
        None,
        0.3,
        0.2,
        (0.3, None, None),
        (0.4, None, 10),
        (0.1, 10, None),
        (0.2, 10, 10),
    ],
)
def test_init_chars_params(chars_params: float | tuple[float, int | None, int | None] | None) -> None:
    CharsRandomAugmenter(TextAction.DELETE, None, chars_params)
    CharsRandomAugmenter(TextAction.DELETE, None, char_params=chars_params)
    CharsRandomAugmenter(TextAction.DELETE, char_params=chars_params)


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
    CharsRandomAugmenter(TextAction.DELETE, None, None, stopwords)
    CharsRandomAugmenter(TextAction.DELETE, None, None, stopwords=stopwords)
    CharsRandomAugmenter(TextAction.DELETE, stopwords=stopwords)


@pytest.mark.parametrize("locale", [None, "en", "sr-Cyrl-ME", "ru_RU"])
def test_init_locale(locale: str) -> None:
    CharsRandomAugmenter(TextAction.DELETE, None, None, None, locale)
    CharsRandomAugmenter(TextAction.DELETE, None, None, None, locale=locale)
    CharsRandomAugmenter(TextAction.DELETE, locale=locale)


def test_init_locale_error() -> None:
    with pytest.raises(Exception):
        CharsRandomAugmenter(TextAction.DELETE, None, None, None, "NOT_EXISTING_LOCALE")
    with pytest.raises(Exception):
        CharsRandomAugmenter(TextAction.INSERT, None, None, None, "en")
    with pytest.raises(Exception):
        CharsRandomAugmenter(TextAction.SUBSTITUTE, None, None, None, "en")


@pytest.mark.parametrize(
    "text",
    [
        "word",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ],
)
def test_input_changes(text: str) -> None:
    augmenter = CharsRandomAugmenter(TextAction.DELETE, 0.3, 0.3)
    output = augmenter.augment(text)
    assert text != output
    assert len(text) >= len(output)


def test_input_changes_batch() -> None:
    augmenter = CharsRandomAugmenter(TextAction.DELETE, 0.3, 0.3)
    texts = [
        "word",
        "Some sentence",
        "Some sentence with 5 words!",
        "This is 2 sentences. This is the second sentence.",
    ]
    output = augmenter.augment_batch(texts)
    assert texts != output
    assert len(texts) == len(output)
    for text, out in zip(texts, output):
        assert len(text) >= len(out)
