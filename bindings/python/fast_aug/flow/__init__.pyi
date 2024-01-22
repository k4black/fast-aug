# Generated content DO NOT EDIT
from __future__ import annotations

from typings import Any

class ChanceAugmenter(BaseAugmenter):
    """
    Given other augmenter apply it with a given probability
    :param augmenter: The augmenter to apply with a given probability
    :param probability: The probability of applying the augmenter
    """
    def __init__(augmenter: BaseAugmenter, probability: float):
        pass


class SelectorAugmenter(BaseAugmenter):
    """
    Given a list of augmenters, apply one of them randomly
    :param augmenters: The list of augmenters to choose from
    :param weights: Optional weights for each augmenter
    """
    def __init__(self, augmenters: list[BaseAugmenter], weights: Optional[list[float]] = None):
        pass


class SequentialAugmenter(BaseAugmenter):
    """
    Given a list of augmenters, apply them sequentially
    :param augmenters: The list of augmenters to apply sequentially
    """
    def __init__(self, augmenters: list[BaseAugmenter]):
        pass


