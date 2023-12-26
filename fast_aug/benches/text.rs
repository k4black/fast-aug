use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_aug::text::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


const BENCHMARK_DATASET_PATH: &str = "data/tweet_eval_sentiment_train_text.txt";


// Function to load txt file and return a vector of strings
fn load_txt_to_string_vector<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect(); // Collects the lines into a Result<Vec<String>, Error>
    lines
}


fn benchmark_individual_augmenters(c: &mut Criterion) {
    // Load dataset
    let text_data = load_txt_to_string_vector(BENCHMARK_DATASET_PATH).expect("Unable to load dataset");

    // Benchmark for RandomWordsAugmenter
    let mut group = c.benchmark_group("RandomWordsAugmenter");
    group.bench_function("swap", |b| {
        b.iter(|| {
            let aug = RandomWordsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            for text in text_data.iter() {
                black_box(aug.augment(text.clone()));
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
                black_box(aug.augment(text.clone()));
            }
        });
    });
    group.finish();

    // Benchmark for RandomCharsAugmenter
    let mut group = c.benchmark_group("RandomCharsAugmenter");
    group.bench_function("swap", |b| {
        b.iter(|| {
            let aug = RandomCharsAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::new(0.5, None, None),
                TextAugmentParameters::new(0.5, None, None),
                None,
            );
            for text in text_data.iter() {
                black_box(aug.augment(text.clone()));
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
                black_box(aug.augment(text.clone()));
            }
        });
    });
    group.finish();

    // Add similar sections here for RandomCharsAugmenter, etc.
}

// fn benchmark_pipeline(c: &mut Criterion) {
//     let input_text = "Some string".to_string();
//
//     c.bench_function("TextAugmentationPipeline", |b| {
//         b.iter(|| {
//             // Construct your pipeline here
//             // For example, assume you have a pipeline that takes augmenters:
//             let pipeline = MyPipeline::new() // Hypothetical pipeline construct
//                 .add(RandomWordsAugmenter::new(
//                     TextAction::Swap,
//                     TextAugmentParameters::new(0.5, None, None),
//                     None,
//                 ))
//                 // ... add other augmenters as needed ...
//                 ;
//
//             black_box(pipeline.augment(input_text.clone())); // Apply the pipeline
//         });
//     });
// }

// Define the groups using the functions
criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = benchmark_individual_augmenters //, benchmark_pipeline
}
criterion_main!(benches);
