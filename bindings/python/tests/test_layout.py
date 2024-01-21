import fast_aug


def test_module_layout() -> None:
    assert sorted(fast_aug.__all__) == sorted(["__version__", "BaseAugmenter", "BaseTextAugmenter", "base", "text", "flow", "models"])
    assert fast_aug.__doc__ and len(fast_aug.__doc__) > 0, "module docstring is empty"


def test_base_module_layout() -> None:
    assert sorted(fast_aug.base.__all__) == sorted(["BaseAugmenter"])
    assert fast_aug.base.__doc__ and len(fast_aug.base.__doc__) > 0, "base module docstring is empty"


def test_text_module_layout() -> None:
    assert sorted(fast_aug.text.__all__) == sorted([
        "BaseTextAugmenter", "TextAction", "RandomCharsAugmenter", "RandomWordsAugmenter"
    ])
    assert fast_aug.text.__doc__ and len(fast_aug.text.__doc__) > 0, "text module docstring is empty"


def test_flow_module_layout() -> None:
    assert sorted(fast_aug.flow.__all__) == sorted([
        "ChanceAugmenter", "SelectorAugmenter", "SequentialAugmenter"
    ])
    assert fast_aug.flow.__doc__ and len(fast_aug.flow.__doc__) > 0, "flow module docstring is empty"
