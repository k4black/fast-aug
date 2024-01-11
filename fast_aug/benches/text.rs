use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use fast_aug::text::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::SeedableRng;
use fast_aug::base::BaseAugmenter;


const BENCHMARK_DATASET_PATH: &str = "data/tweet_eval_sentiment_train_text.txt";


// Function to load txt file and return a vector of strings
fn load_txt_to_string_vector<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect(); // Collects the lines into a Result<Vec<String>, Error>
    lines
}


fn benchmark_text(c: &mut Criterion) {
    // let mut rng = rand::thread_rng();
    let mut rng = rand::rngs::SmallRng::from_entropy();

    // Load dataset
    let text_data = load_txt_to_string_vector(BENCHMARK_DATASET_PATH).expect("Unable to load dataset");
    let text_data = text_data[0..text_data.len()/4].to_vec();

    // Benchmark for RandomWordsAugmenter
    let mut group = c.benchmark_group("RandomWordsAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("swap", |b| {
        b.iter(|| {
            let aug = RandomWordsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            for text in text_data.iter() {
                black_box(aug.augment(text.clone(), &mut rng));
            }
        });
    });
    group.bench_function("delete", |b| {
        b.iter(|| {
            let aug = RandomWordsAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            for text in text_data.iter() {
                black_box(aug.augment(text.clone(), &mut rng));
            }
        });
    });
    group.finish();

    // Benchmark for RandomCharsAugmenter
    let mut group = c.benchmark_group("RandomCharsAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("swap", |b| {
        b.iter(|| {
            let aug = RandomCharsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, None, None),
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            for text in text_data.iter() {
                black_box(aug.augment(text.clone(), &mut rng));
            }
        });
    });
    group.bench_function("delete", |b| {
        b.iter(|| {
            let aug = RandomCharsAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::new(0.5, None, None),
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            for text in text_data.iter() {
                black_box(aug.augment(text.clone(), &mut rng));
            }
        });
    });
    group.finish();

    // Add similar sections here for RandomCharsAugmenter, etc.
}


// Define the groups using the functions
criterion_group!{
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .measurement_time(std::time::Duration::from_secs(30))
        .significance_level(0.01);
    targets = benchmark_text
}
criterion_main!(benches);
