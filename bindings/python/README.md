# fast-aug - python bindings

[![Test Rust library](https://github.com/k4black/fast-aug/actions/workflows/test-rust.yml/badge.svg?branch=main&event=push)](https://github.com/k4black/fast-aug/actions/workflows/test-rust.yml)
[![Test Python bindings](https://github.com/k4black/fast-aug/actions/workflows/test-python.yml/badge.svg?branch=main&event=push)](https://github.com/k4black/fast-aug/actions/workflows/test-python.yml)

---

The `fast-aug` library **x25** times faster than `nlpaug`!


## Performance Comparison

Comparison of the `fast-aug` library with the other NLP augmentation libraries.  
All libs compared on [tweeteval dataset](https://github.com/cardiffnlp/tweeteval) - sentiment test set - 12k samples.

![comparison time](./comparison-python-text-time.svg)
![comparison memory](./comparison-python-text-memory.svg)

[//]: # (* for `augmenty` spacy model loading time is included, as we measure end-to-end time and mem &#40;`spacy.lang.en.English` model was used&#41;)
* `fastnlpaug` - is re-write of a couple of `nlpaug` augmenters in rust (not really developed)
* `augly` not included as ["Our text augmentations use nlpaug as their backbone"](https://github.com/facebookresearch/AugLy/tree/main/augly/text)
* Try to compare `augmenty`, but it way too slow, so we exclude it from the comparison (2 times slower than `nlpaug`)

See [./benchmarks/compare_text.py](./benchmarks/compare_text.py) for details.


## Development

Please see the GitHub repository README for mo info [fast-aug](https://github.com/k4black/fast-aug).

For building and profiling see `Makefile` in the project root.

TBA
