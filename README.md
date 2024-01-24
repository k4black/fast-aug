# fast-aug

TBA


* See [fast_aug/README.md](fast_aug/README.md) for rust library details.
* See [bindings/python/README.md](bindings/python/README.md) for python library details.


## Features and TODO

Flow
- [x] ChanceAugmenter
- [x] SelectorAugmenter
- [x] SequentialAugmenter

Text
- [ ] RandomWordsAugmenter
  - [x] Base - swaps/deletions
  - [ ] Insertions/Substitutions (from alphabet)
- [ ] RandomCharsAugmenter
    - [x] Base - swaps/deletions
    - [ ] Insertions/Substitutions (from provided list)
- [ ] RandomSpellingAugmenter
- [ ] RandomKeyboardAugmenter
- [ ] RandomEmbeddingsAugmenter
- [ ] RandomTfIdfAugmenter
- [ ] RandomPosAugmenter
- [ ] [EmojiNormalizer](https://github.com/unicode-org/cldr-json/blob/858baad63c1d51e1d576ef99dccc229d92cedda4/cldr-json/cldr-annotations-full/annotations/en-AU/annotations.json#L1498)
- [ ] Keep labels (e.g. POS tags) unchanged

Models and utils
- [ ] Models lazy loading
  - [ ] At creation time
  - [ ] At first use
  - [ ] Background after creation
- [ ] [candle](https://github.com/huggingface/candle) support for DL models loading
  - [ ] HF loading
  - [ ] ONNX loading
  - [ ] Optimizations (fp16/int8/int4/layers/etc)
  - [ ] GPU support
- [ ] TF-IDF model
  - [ ] json file loading
  - [ ] sklearn model loading
- [x] Alphabet model
- [ ] Embeddings model
  - [ ] fasttext model loading
  - [ ] word2vec model loading
- [ ] WordNet model
  - [ ] English
  - [ ] German
  - [ ] More?

Rust
- [x] Formatting
  - [x] rustfmt
  - [x] clippy
- [ ] [rust flamegraph profiling](https://www.jibbow.com/posts/criterion-flamegraphs/)
- [x] Unit tests
- [ ] Integration tests
- [x] CI build and tests
- [ ] CI publish to crates.io


Python 
- [ ] Custom Python Augmenter class
- [ ] Bindings with 
  - [x] Base pyo3 bindings
  - [x] [maturin](https://github.com/PyO3/maturin) auto build from pyproject.toml
  - [x] Stubs (.pyi) files generation
  - [ ] Auto generate stubs on maturing build
  - [x] Text
  - [x] Flow
- [ ] [flamegraph profiling](https://ohadravid.github.io/posts/2023-03-rusty-python/)
- [ ] Optimizations - see [this](https://ohadravid.github.io/posts/2023-03-rusty-python/)
- [ ] Integration tests
- [x] CI build and tests
- [ ] CI publish to pypi


## Development

### Prerequisites

For rust library development:
* [rustup](https://rustup.rs/)
* [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

For python bindings development:
* All rust library prerequisites
* `cd python/bindings && python -m venv .venv`


### Make

The `Makefile` contains all the commands needed for development.
```shell
make help
```

- `*-rust` - all targets related to rust library (`fast_aug/` folder)
- `*-python` - all targets related to python bindings (`bindings/python/` folder)


### Benchmarks

All text benchmarks are run on the [tweet_eval dataset](https://github.com/cardiffnlp/tweeteval/) - sentiment task, test set, 12k rows.
```shell
cat test_data/tweet_eval_sentiment_test_text.txt | wc
12284  182576 1156877
```

