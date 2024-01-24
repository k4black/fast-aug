use rand::prelude::SliceRandom;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
// use criterion::black_box;
use fast_aug::text::{RandomCharsAugmenter, TextAction, TextAugmentParameters};

use fast_aug::BaseAugmenter;
use rand::SeedableRng;

const BENCHMARK_DATASET_PATH: &str = "../test_data/tweet_eval_sentiment_train_text.txt";

// Function to load txt file and return a vector of strings
fn load_txt_to_string_vector<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect(); // Collects the lines into a Result<Vec<String>, Error>
    lines
}

fn main() {
    let _rng = rand::rngs::SmallRng::from_entropy();

    // Load dataset
    let mut text_data = load_txt_to_string_vector(BENCHMARK_DATASET_PATH).expect("Unable to load dataset");
    text_data = text_data[0..text_data.len() / 4].to_vec();
    // Shuffle the dataset
    let mut rng = rand::thread_rng();
    text_data.shuffle(&mut rng);

    let start = std::time::Instant::now();
    for _ in 0..10 {
        let aug = RandomCharsAugmenter::new(
            TextAction::Delete,
            TextAugmentParameters::new(0.5, Some(1), Some(10)),
            TextAugmentParameters::new(0.5, Some(1), Some(10)),
            None,
        );
        for text in text_data.iter() {
            aug.augment(text.clone(), &mut rng);
            // black_box(aug.augment(text.clone(), &mut rng));
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration / 10);
}
