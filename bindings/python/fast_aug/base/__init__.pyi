# Generated content DO NOT EDIT
from __future__ import annotations

from typing import Any

class BaseAugmenter:
    """
    Abstract Base Class for Augmentation
    """

    def __init__(self) -> None:
        pass
    def augment(self, data: Any):
        """
        Augment data
        :param data: Data to augment - single data point
        :returns: Augmented data
        """
        pass
    def augment_batch(self, data: list[Any]):
        """
        Augment data given a batch of data
        :param data: Data to augment - vector of data points
        :returns: Augmented data
        """
        pass
