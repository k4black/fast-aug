use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use fast_aug::text::*;

mod common;
use common::{bench_text_augmenter, get_config};
use fast_aug::models::text::AlphabetModel;

// Criterion entry point
fn criterion_benchmark(c: &mut Criterion) {
    let vocal: Vec<String> = vec![
        "hello", "world", "this", "is", "a", "test", "of", "fast_aug", "library", "for", "rust",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let mut group = c.benchmark_group("words");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "WordsRandomInsertAugmenter",
        &WordsRandomInsertAugmenter::new(TextAugmentParameters::default(), vocal.clone(), None),
    );
    bench_text_augmenter(
        &mut group,
        "WordsRandomSubstituteAugmenter",
        &WordsRandomSubstituteAugmenter::new(TextAugmentParameters::default(), vocal.clone(), None),
    );
    bench_text_augmenter(
        &mut group,
        "WordsRandomSwapAugmenter",
        &WordsRandomSwapAugmenter::new(TextAugmentParameters::default(), None),
    );
    bench_text_augmenter(
        &mut group,
        "WordsRandomDeleteAugmenter",
        &WordsRandomDeleteAugmenter::new(TextAugmentParameters::default(), None),
    );
    group.finish();

    let mut group = c.benchmark_group("chars");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "CharsRandomInsertAugmenter",
        &CharsRandomInsertAugmenter::new(
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            AlphabetModel::from_locale_str("en"),
            None,
        ),
    );
    bench_text_augmenter(
        &mut group,
        "CharsRandomSubstituteAugmenter",
        &CharsRandomSubstituteAugmenter::new(
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            AlphabetModel::from_locale_str("en"),
            None,
        ),
    );
    bench_text_augmenter(
        &mut group,
        "CharsRandomSwapAugmenter",
        &CharsRandomSwapAugmenter::new(TextAugmentParameters::default(), TextAugmentParameters::default(), None),
    );
    bench_text_augmenter(
        &mut group,
        "CharsRandomDeleteAugmenter",
        &CharsRandomDeleteAugmenter::new(TextAugmentParameters::default(), TextAugmentParameters::default(), None),
    );
    group.finish();
}

// Define the groups using the functions
criterion_group! {
    name = benches;
    config = get_config();
    targets = criterion_benchmark
}
criterion_main!(benches);
