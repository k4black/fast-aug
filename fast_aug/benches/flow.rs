use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use fast_aug::text::*;
use fast_aug::flow::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Arc;
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


fn benchmark_flow(c: &mut Criterion) {
    let mut rng = rand::rngs::SmallRng::from_entropy();

    // Load dataset
    let text_data = load_txt_to_string_vector(BENCHMARK_DATASET_PATH).expect("Unable to load dataset");
    let text_data = text_data[0..text_data.len()/2].to_vec();

    // Benchmark pipelines/flow
    let mut group = c.benchmark_group("Flow");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("SequentialAugmenter", |b| {
        b.iter(|| {
            let aug_1 = RandomWordsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            let aug_2 = RandomCharsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, Some(2), Some(5)),
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            let aug_3 = RandomWordsAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            let aug_4 = RandomCharsAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::new(0.5, None, None),
                TextAugmentParameters::new(0.5, Some(1), Some(2)),
                None,
            );
            let pipeline = SequentialAugmenter::new(vec![
                Arc::new(aug_1),
                Arc::new(aug_2),
                Arc::new(aug_3),
                Arc::new(aug_4),
            ]);
            for text in text_data.iter() {
                black_box(pipeline.augment(text.clone(), &mut rng));
            }
        });
    });
    group.bench_function("SelectorAugmenter", |b| {
        b.iter(|| {
            let aug_1 = RandomWordsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            let aug_2 = RandomCharsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, Some(2), Some(5)),
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            let aug_3 = RandomWordsAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            let aug_4 = RandomCharsAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::new(0.5, None, None),
                TextAugmentParameters::new(0.5, Some(1), Some(2)),
                None,
            );
            let pipeline = SelectorAugmenter::new(vec![
                Arc::new(aug_1),
                Arc::new(aug_2),
                Arc::new(aug_3),
                Arc::new(aug_4),
            ], None);
            for text in text_data.iter() {
                black_box(pipeline.augment(text.clone(), &mut rng));
            }
        });
    });
    group.bench_function("ChanceAugmenter", |b| {
        b.iter(|| {
            let aug = RandomWordsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            let pipeline = ChanceAugmenter::new(Arc::new(aug), 0.5);
            for text in text_data.iter() {
                black_box(pipeline.augment(text.clone(), &mut rng));
            }
        });
    });
}

// Define the groups using the functions
criterion_group!{
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .measurement_time(std::time::Duration::from_secs(30))
        .significance_level(0.01);
    targets = benchmark_flow
}
criterion_main!(benches);
