import fast_aug


def test_module_layout() -> None:
    assert sorted(fast_aug.__all__) == sorted(["__version__", "BaseAugmenter", "BaseTextAugmenter", "base", "text", "flow", "models"])


def test_base_module_layout() -> None:
    assert sorted(fast_aug.base.__all__) == sorted(["BaseAugmenter"])


def test_text_module_layout() -> None:
    assert sorted(fast_aug.text.__all__) == sorted([
        "BaseTextAugmenter", "TextAction", "RandomCharsAugmenter", "RandomWordsAugmenter"
    ])


def test_flow_module_layout() -> None:
    assert sorted(fast_aug.flow.__all__) == sorted([
        "ChanceAugmenter", "SelectorAugmenter", "SequentialAugmenter"
    ])
