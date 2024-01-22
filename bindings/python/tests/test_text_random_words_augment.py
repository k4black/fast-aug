import pytest

from fast_aug.text import TextAction
from fast_aug.text import RandomWordsAugmenter


@pytest.mark.parametrize("action", [
    "DELETE", "INSERT", "SWAP", "SUBSTITUTE",
    TextAction.DELETE, TextAction.INSERT, TextAction.SWAP, TextAction.SUBSTITUTE
])
def test_init_action(action: str | TextAction) -> None:
    RandomWordsAugmenter(action)


def test_init_action_error() -> None:
    with pytest.raises(Exception):
        RandomWordsAugmenter("NOT_EXISTING_ACTION")
    with pytest.raises(Exception):
        RandomWordsAugmenter()
    with pytest.raises(Exception):
        RandomWordsAugmenter(None)


@pytest.mark.parametrize("words_params", [
    None, 0.3, 0.2, (0.3, None, None), (0.4, None, 10), (0.1, 10, None), (0.2, 10, 10)
])
def test_init_words_params(words_params: float | tuple[float, int | None, int | None] | None) -> None:
    RandomWordsAugmenter(TextAction.DELETE, words_params)
    RandomWordsAugmenter(TextAction.DELETE, words_params, None)
    RandomWordsAugmenter(TextAction.DELETE, aug_params_word=words_params)


@pytest.mark.parametrize("stopwords", [
    # None, list(), set(), ["a", "b", "c"], {"a", "b", "c"}
    None, set(), {"a", "b", "c"}
])
def test_init_stopwords(stopwords: list[str] | set[str] | None) -> None:
    RandomWordsAugmenter(TextAction.DELETE, None, stopwords)
    RandomWordsAugmenter(TextAction.DELETE, None, stopwords=stopwords)
    RandomWordsAugmenter(TextAction.DELETE, stopwords=stopwords)


@pytest.mark.parametrize("text", ["word", "Some sentence", "Some sentence with 5 words!", "This is 2 sentences. This is the second sentence."])
def test_input_changes(text: str) -> None:
    augmenter = RandomWordsAugmenter(TextAction.DELETE, 0.3)
    output = augmenter.augment(text)
    assert text != output
    assert len(text) >= len(output)
