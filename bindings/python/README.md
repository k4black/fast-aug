# fast-aug - python bindings

[![Python Test Workflow Status](https://img.shields.io/github/actions/workflow/status/k4black/fast-aug/test-python.yml?branch=main&event=push&label=python%20tests)](https://github.com/k4black/fast-aug/actions/workflows/test-python.yml)
[![PyPI - Version](https://img.shields.io/pypi/v/fast-aug)](https://pypi.org/project/fast-aug/)
[![GitHub License](https://img.shields.io/github/license/k4black/fast-aug)](https://github.com/k4black/fast-aug/blob/main/LICENSE)


`fast-aug` is a library for fast text augmentation, available for both Rust and Python as `fast-aug`.  
It is designed with focus on performance and real-time usage (e.g. during training), while providing a wide range of text augmentation methods.

Note: **x25** times faster than `nlpaug`!

---


## Installation

`fast-aug` is available on [PyPI](https://pypi.org/project/fast-aug).

```shell
pip install fast-aug
```

## Usage

```python
from fast_aug import CharsRandomSwapAugmenter

text_data = "Some text!"
augmenter = CharsRandomSwapAugmenter(
    0.5,  # probability of words selection
    0.5,  # probability of characters selection
    None,  # stopwords
)
assert augmenter.augment(text_data) != text_data
assert augmenter.augment([text_data]) != [text_data]
```

TBA

## Performance Comparison

Comparison of the `fast-aug` library with the other NLP augmentation libraries.

* `fast-aug` - this, Fast Augmentation library written in Rust, with Python bindings
* `nlpaug` - [nlpaug](https://github.com/makcedward/nlpaug) - The most popular NLP augmentation library
* `fasttextaug` - [fasttextaug](https://github.com/Tzinch21/fasttextaug) - re-write of some `nlpaug`'s augmenters in Rust with Python bindings
* `augly` not included as ["Our text augmentations use nlpaug as their backbone"](https://github.com/facebookresearch/AugLy/tree/main/augly/text)
* `augmenty` not included as it is too slow (2-8 times slower than `nlpaug`)

[//]: # (* for `augmenty` spacy model loading time is included, as we measure end-to-end time and mem &#40;`spacy.lang.en.English` model was used&#41;)

It is end-to-end comparison, including dataset loading, classes initialization and augmentation of all samples (one-by-one or provided as a list).  
See [./benchmarks/compare_text.py](./benchmarks/compare_text.py) for details of the comparison.


![comparison time](https://raw.githubusercontent.com/k4black/fast-aug/main/bindings/python/comparison-python-text-time.svg)
![comparison memory](https://raw.githubusercontent.com/k4black/fast-aug/main/bindings/python/comparison-python-text-memory.svg)


All libs compared on [tweeteval dataset](https://github.com/cardiffnlp/tweeteval) - sentiment test set - 12k samples.  
Note: dataset text file size is 1.1Mb, it is included in the memory usage.


## Contributing and Development

Any contribution is warmly welcomed!  
Please see the GitHub repository README at [fast-aug](https://github.com/k4black/fast-aug).
