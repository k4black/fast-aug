# Generated content DO NOT EDIT
from __future__ import annotations

from typing import Any

from ..base import BaseAugmenter

class TextAction:
    pass

class BaseTextAugmenter(BaseAugmenter):
    """
    Abstract Base Class for Text Augmentation
    """

    def __init__(self) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: The single String to augment
        :return: The augmented data
        """
        pass
    def augment_batch(self, data: list[Any]):
        """
        Augment data given a batch of data
        :param data: Vector of Strings to augment
        :returns: Augmented data
        """
        pass

class CharsRandomAugmenter(BaseTextAugmenter):
    """
    Randomly augment chars in the words
    :param action: The action to perform - insert, substitute, swap, delete
    :param word_params: The parameters for the word augmentation
        - probability or (probability, min_elements, max_elements)
    :param char_params: The parameters for the char augmentation
        - probability or (probability, min_elements, max_elements)
    :param stopwords: The set of stopwords to ignore
    :param locale: The locale string to use for alphabet, optional. Required for insert and substitute
    """

    def __init__(
        self,
        action: str | TextAction,
        word_params: float | tuple[float, int | None, int | None] | None = None,
        char_params: float | tuple[float, int | None, int | None] | None = None,
        stopwords: set[str] | None = None,
        locale: str | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :return: The augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment the data given a batch
        :param data: The list of Strings to augment
        :return: The augmented data
        """
        pass

class WordsRandomAugmenter(BaseTextAugmenter):
    """
    Randomly augment the words
    :param action: The action to perform - insert, substitute, swap, delete
    :param word_params: The parameters for the word augmentation
        - probability or (probability, min_elements, max_elements)
    :param stopwords: The set of stopwords to ignore
    :param vocab: The optional vocabulary to use for insert and substitute
    """

    def __init__(
        self,
        action: str | TextAction,
        word_params: float | tuple[float, int | None, int | None] | None = None,
        stopwords: set[str] | None = None,
        vocabulary: list[str] | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :return: The augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment the data given a batch
        :param data: The list of Strings to augment
        :return: The augmented data
        """
        pass
