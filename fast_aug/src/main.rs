use fast_aug::text::{TextAction};
use fast_aug::base::BaseAugmenter;
use fast_aug::text::RandomCharsAugmenter;
use fast_aug::text::TextAugmentParameters;
use std::fs::File;
use std::io::BufReader;

use finalfusion::prelude::*;


fn main() {

    // Start time measurement
    let start = std::time::Instant::now();

    let mut reader = BufReader::new(File::open("test_data/cc.en.24.bin").unwrap());

    // Read the embeddings.
    let embeddings = Embeddings::read_fasttext(&mut reader)
        .unwrap();

    // Look up an embedding.
    let embedding = embeddings.embedding("try");
    println!("Embedding for 'try': {:?}", embedding);

    // End time measurement
    let end = std::time::Instant::now();
    println!("Time elapsed: {:?}", end.duration_since(start));
}
