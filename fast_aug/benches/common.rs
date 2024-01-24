use criterion::measurement::WallTime;
use criterion::{black_box, BenchmarkGroup, Criterion};
use fast_aug::base::BaseAugmenter;
use fast_aug::text::Doc;
use rand::SeedableRng;
use std::fs::File;
use std::io;
use std::io::BufRead;

const BENCHMARK_DATASET_PATH: &str = "../test_data/tweet_eval_sentiment_test_text.txt";

pub fn get_config() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .measurement_time(std::time::Duration::from_secs(30))
        .significance_level(0.01)
}

// Benchmark function for a single text augmenter
pub fn bench_text_augmenter(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    augmenter: &dyn BaseAugmenter<String, Doc>,
) {
    let mut rng = rand::rngs::SmallRng::from_seed([0; 32]);

    // Load dataset
    let file = File::open(BENCHMARK_DATASET_PATH).expect("Unable to load dataset file");
    let reader = io::BufReader::new(file);
    let text_data: Vec<String> = reader.lines().collect::<Result<_, _>>().expect("Unable to read lines");

    // Benchmark for current augmenter
    group.bench_function(name, |b| {
        // Semi-create augmenter
        // let augmenter = augmenter.clone();
        // Benchmark
        b.iter(|| {
            for text in text_data.iter() {
                black_box(augmenter.augment(text.clone(), &mut rng));
            }
        });
    });
}
