# Generated content DO NOT EDIT
from __future__ import annotations

from ..base import BaseAugmenter

class ChanceAugmenter(BaseAugmenter):
    """
    Given other augmenter apply it with a given probability
    :param augmenter: The augmenter to apply with a given probability
    :param probability: The probability of applying the augmenter
    """

    def __init__(self, augmenter: BaseAugmenter, probability: float) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: The String data to augment
        :return: The augmented data
        """
        pass

class SelectorAugmenter(BaseAugmenter):
    """
    Given a list of augmenters, apply one of them randomly
    :param augmenters: The list of augmenters to choose from
    :param weights: Optional weights for each augmenter
    """

    def __init__(self, augmenters: list[BaseAugmenter], weights: list[float] | None = None) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: The String data to augment
        :return: The augmented data
        """
        pass

class SequentialAugmenter(BaseAugmenter):
    """
    Given a list of augmenters, apply them sequentially
    :param augmenters: The list of augmenters to apply sequentially
    """

    def __init__(self, augmenters: list[BaseAugmenter]) -> None:
        pass
    def augment(self, data: str):
        """
        Augment the data
        :param data: The String data to augment
        :return: The augmented data
        """
        pass
