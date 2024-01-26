import pytest

from fast_aug.text import RandomCharsAugmenter, TextAction


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
    RandomCharsAugmenter(action)


def test_init_action_error() -> None:
    with pytest.raises(Exception):
        RandomCharsAugmenter("NOT_EXISTING_ACTION")
    with pytest.raises(Exception):
        RandomCharsAugmenter()
    with pytest.raises(Exception):
        RandomCharsAugmenter(None)


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
    RandomCharsAugmenter(TextAction.DELETE, words_params)
    RandomCharsAugmenter(TextAction.DELETE, words_params, None)
    RandomCharsAugmenter(TextAction.DELETE, aug_params_word=words_params)


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
    RandomCharsAugmenter(TextAction.DELETE, None, chars_params)
    RandomCharsAugmenter(TextAction.DELETE, None, aug_params_char=chars_params)
    RandomCharsAugmenter(TextAction.DELETE, aug_params_char=chars_params)


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
    RandomCharsAugmenter(TextAction.DELETE, None, None, stopwords)
    RandomCharsAugmenter(TextAction.DELETE, None, None, stopwords=stopwords)
    RandomCharsAugmenter(TextAction.DELETE, stopwords=stopwords)


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
    augmenter = RandomCharsAugmenter(TextAction.DELETE, 0.3, 0.3)
    output = augmenter.augment(text)
    assert text != output
    assert len(text) >= len(output)


def test_input_changes_batch() -> None:
    augmenter = RandomCharsAugmenter(TextAction.DELETE, 0.3, 0.3)
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
