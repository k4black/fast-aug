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

Models and utils
- Models lazy loading
  - [ ] At creation time
  - [ ] At first use
  - [ ] Background after creation
- [ ] [candle](https://github.com/huggingface/candle) support for DL models loading
  - HF loading
  - ONNX loading
  - Optimizations (fp16/int8/int4/layers/etc)
  - GPU support
- [ ] TF-IDF model
  - [ ] json file loading
  - [ ] sklearn model loading
- [x] Alphabet model
- [ ] Embeddings model
  - [ ] fasttext model loading
  - [ ] word2vec model loading
- WordNet model
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
- [ ] Bindings with 
  - [ ] Base pyo3 bindings
  - [x] [maturin](https://github.com/PyO3/maturin) auto build from pyproject.toml
  - [ ] Auto stubs (.pyi) files generation
- [ ] [flamegraph profiling](https://ohadravid.github.io/posts/2023-03-rusty-python/)
- [ ] Optimizations - see [this](https://ohadravid.github.io/posts/2023-03-rusty-python/)
- [ ] Integration tests
- [x] CI build and tests
- [ ] CI publish to pypi

