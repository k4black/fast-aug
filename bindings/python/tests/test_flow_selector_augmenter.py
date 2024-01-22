from typing import List, Optional, Any
import pytest
from fast_aug.base import BaseAugmenter
from fast_aug.text import RandomWordsAugmenter, RandomCharsAugmenter
from fast_aug.flow import SelectorAugmenter


@pytest.mark.parametrize("augmenters", [
    [RandomWordsAugmenter('DELETE'), RandomCharsAugmenter('DELETE')],
    [RandomWordsAugmenter('DELETE'), RandomCharsAugmenter('DELETE'), RandomCharsAugmenter('DELETE')],
])
def test_init_augmenters(augmenters: List[BaseAugmenter]) -> None:
    SelectorAugmenter(augmenters)


@pytest.mark.parametrize("augmenters", [
    [],
    [RandomWordsAugmenter('DELETE')],
    None,
    [object, object],
    [RandomWordsAugmenter('DELETE'), "invalid_type"],
])
def test_init_wrong_augmenters(augmenters: Any) -> None:
    with pytest.raises(Exception):
        SelectorAugmenter(augmenters)


@pytest.mark.parametrize("weights", [
    None,
    [0.7, 0.3],
    [0.5, 0.5],
    [0.1, 1.0],
    [1, 1000],
    [-0.1, 1.1],
])
def test_init_weights(weights: Optional[List[float]]) -> None:
    augmenters = [RandomWordsAugmenter('DELETE'), RandomCharsAugmenter('DELETE')]
    SelectorAugmenter(augmenters, weights)

@pytest.mark.parametrize("weights", [
    [],
    [0.5],
    [0.5, 0.5, 0.5],
    [0.4, object],
    [0.5, "invalid_type"],
    [None, 0.3],
])
def test_init_wrong_selector(weights: Any) -> None:
    augmenters = [RandomWordsAugmenter('DELETE'), RandomCharsAugmenter('DELETE')]
    with pytest.raises(Exception):
        SelectorAugmenter(augmenters, weights)


@pytest.mark.parametrize("text", ["word", "Some sentence", "A longer sentence with more words!"])
def test_input_changes(text: str) -> None:
    augmenters = [RandomWordsAugmenter('DELETE', 0.3), RandomCharsAugmenter('DELETE', 0.3)]
    selector_augmenter = SelectorAugmenter(augmenters)

    assert selector_augmenter.augment(text) != text
