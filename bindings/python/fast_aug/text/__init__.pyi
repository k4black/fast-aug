# Generated content DO NOT EDIT
from __future__ import annotations

from typings import Any

class TextAction:
    pass

class BaseTextAugmenter(BaseAugmenter):
    """
    Abstract Base Class for Text Augmentation
    """
    def __init__(self):
        pass


class RandomCharsAugmenter(BaseTextAugmenter):
    """
    Randomly augment chars in the words
    :param action: The action to perform - insert, substitute, swap, delete
    :param aug_params_word: The parameters for the word augmentation - probability or (probability, min_elements, max_elements)
    :param aug_params_char: The parameters for the char augmentation - probability or (probability, min_elements, max_elements)
    :param stopwords: The set of stopwords to ignore
    """
    def __init__(self, action: str | TextAction, aug_params_word: float | tuple[float, int | None, int | None] | None = None, aug_params_char: float | tuple[float, int | None, int | None] | None = None, stopwords: set[str] | None = None):
        pass


class RandomWordsAugmenter(BaseTextAugmenter):
    """
    Randomly augment the words
    :param action: The action to perform - insert, substitute, swap, delete
    :param aug_params_word: The parameters for the word augmentation - probability or (probability, min_elements, max_elements)
    :param stopwords: The set of stopwords to ignore
    """
    def __init__(self, action: str | TextAction, aug_params_word: float | tuple[float, int | None, int | None] | None = None, stopwords: set[str] | None = None):
        pass


