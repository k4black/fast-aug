from . import fast_aug

text = fast_aug.text
flow = fast_aug.flow
base = fast_aug.base
models = fast_aug.models

BaseAugmenter = fast_aug.base.BaseAugmenter
BaseTextAugmenter = fast_aug.text.BaseTextAugmenter

__all__ = ["text", "flow", "base", "models", "BaseAugmenter", "BaseTextAugmenter"]
__doc__ = fast_aug.__doc__
__version__ = fast_aug.__version__
