# fast-aug - rust library

![Rust Test Workflow Status](https://img.shields.io/github/actions/workflow/status/k4black/fast-aug/test-rust.yml?branch=main&event=push&label=rust%20tests)
![Crates.io Version](https://img.shields.io/crates/v/fast-aug)
![Rust docs](https://img.shields.io/docsrs/fast-aug)
![GitHub License](https://img.shields.io/github/license/k4black/fast-aug)

`fast-aug` is a library for fast text augmentation, available for both Rust and Python as `fast-aug`.  
It is designed with focus on performance and real-time usage (e.g. during training), while providing a wide range of text augmentation methods.

---


## Installation

`fast-aug` is available on [crates.io](https://crates.io/crates/fast-aug).

```shell
cargo install fast-aug
```


## Usage

```rust
use fast_aug::base::BaseAugmenter;
use fast_aug::text::{CharsRandomSwapAugmenter, TextAugmentParameters};

let rng = &mut rand::thread_rng();
let augmenter = CharsRandomSwapAugmenter::new(
    TextAugmentParameters::new(0.5, None, None),
    TextAugmentParameters::new(0.5, None, None),
    None,
);
augmenter.augment("Some text!".to_string(), rng);
augmenter.augment_batch(vec!["Some text!".to_string()], rng);
```

Please refer to [rustdoc](https://docs.rs/fast-aug) for details.

TBA


## Contributing and Development

Any contribution is warmly welcomed!  
Please see the GitHub repository README at [fast-aug](https://github.com/k4black/fast-aug).

