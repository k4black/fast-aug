# Generated content DO NOT EDIT
from __future__ import annotations

from ..base import BaseAugmenter

class BaseTextAugmenter(BaseAugmenter):
    """
    Abstract Base Class for Text Augmentation
    """

    def __init__(self) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class CharsRandomDeleteAugmenter(BaseTextAugmenter):
    """
    Randomly delete chars in the random words

    :param word_params: The parameters for the word augmentation
     - probability or (probability, min_elements, max_elements)
    :param char_params: The parameters for the char augmentation
     - probability or (probability, min_elements, max_elements)
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self,
        word_params: float | tuple[float, int | None, int | None],
        char_params: float | tuple[float, int | None, int | None],
        stopwords: set[str] | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class CharsRandomInsertAugmenter(BaseTextAugmenter):
    """
    Randomly augment chars in the random words

    :param word_params: The parameters for the word augmentation
        - probability or (probability, min_elements, max_elements)
    :param char_params: The parameters for the char augmentation
        - probability or (probability, min_elements, max_elements)
    :param locale: The locale string to use for alphabet
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self,
        word_params: float | tuple[float, int | None, int | None],
        char_params: float | tuple[float, int | None, int | None],
        locale: str,
        stopwords: set[str] | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class CharsRandomSubstituteAugmenter(BaseTextAugmenter):
    """
    Randomly substitute chars in the random words

    :param word_params: The parameters for the word augmentation
       - probability or (probability, min_elements, max_elements)
    :param char_params: The parameters for the char augmentation
      - probability or (probability, min_elements, max_elements)
    :param locale: The locale string to use for alphabet
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self,
        word_params: float | tuple[float, int | None, int | None],
        char_params: float | tuple[float, int | None, int | None],
        locale: str,
        stopwords: set[str] | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class CharsRandomSwapAugmenter(BaseTextAugmenter):
    """
    Randomly swap chars in the random words

    :param word_params: The parameters for the word augmentation
      - probability or (probability, min_elements, max_elements)
    :param char_params: The parameters for the char augmentation
     - probability or (probability, min_elements, max_elements)
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self,
        word_params: float | tuple[float, int | None, int | None],
        char_params: float | tuple[float, int | None, int | None],
        stopwords: set[str] | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class WordsRandomDeleteAugmenter(BaseTextAugmenter):
    """
    Randomly delete words

    :param word_params: The parameters for the word augmentation
      - probability or (probability, min_elements, max_elements)
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self, word_params: float | tuple[float, int | None, int | None], stopwords: set[str] | None = None
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class WordsRandomInsertAugmenter(BaseTextAugmenter):
    """
    Randomly insert words from the given vocabulary

    :param word_params: The parameters for the word augmentation
        - probability or (probability, min_elements, max_elements)
    :param vocabulary: The vocabulary to use for insertion
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self,
        word_params: float | tuple[float, int | None, int | None],
        vocabulary: list[str],
        stopwords: set[str] | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class WordsRandomSubstituteAugmenter(BaseTextAugmenter):
    """
    Randomly substitute words from the given vocabulary

    :param word_params: The parameters for the word augmentation
       - probability or (probability, min_elements, max_elements)
    :param vocabulary: The vocabulary to use for substitution
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self,
        word_params: float | tuple[float, int | None, int | None],
        vocabulary: list[str],
        stopwords: set[str] | None = None,
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass

class WordsRandomSwapAugmenter(BaseTextAugmenter):
    """
    Randomly swap words

    :param word_params: The parameters for the word augmentation
       - probability or (probability, min_elements, max_elements)
    :param stopwords: Optional set of stopwords to ignore
    """

    def __init__(
        self, word_params: float | tuple[float, int | None, int | None], stopwords: set[str] | None = None
    ) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: A String to augment
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[str]):
        """
        Augment data given a batch of data
        :param data: Vector of strings to augment
        :returns: Augmented data
        """
        pass
