use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use fast_aug::text::*;

mod common;
use common::{bench_text_augmenter, get_config};
use fast_aug::models::text::AlphabetModel;

// Criterion entry point
fn criterion_benchmark(c: &mut Criterion) {
    let word_vector: Vec<String> = vec![
        "hello".to_string(),
        "world".to_string(),
        "this".to_string(),
        "is".to_string(),
        "a".to_string(),
        "test".to_string(),
        "sentence".to_string(),
        "with".to_string(),
        "some".to_string(),
        "words".to_string(),
    ];
    let mut group = c.benchmark_group("RandomWordsAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "insert",
        &WordsRandomAugmenter::new(
            TextAction::Insert,
            TextAugmentParameters::default(),
            None,
            Some(word_vector.clone()),
        ),
    );
    bench_text_augmenter(
        &mut group,
        "substitute",
        &WordsRandomAugmenter::new(
            TextAction::Substitute,
            TextAugmentParameters::default(),
            None,
            Some(word_vector.clone()),
        ),
    );
    bench_text_augmenter(
        &mut group,
        "swap",
        &WordsRandomAugmenter::new(TextAction::Swap, TextAugmentParameters::default(), None, None),
    );
    bench_text_augmenter(
        &mut group,
        "delete",
        &WordsRandomAugmenter::new(TextAction::Delete, TextAugmentParameters::default(), None, None),
    );
    group.finish();

    let mut group = c.benchmark_group("RandomCharsAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "insert",
        &CharsRandomAugmenter::new(
            TextAction::Insert,
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            None,
            Some(AlphabetModel::from_locale_str("en")),
            false,
        ),
    );
    bench_text_augmenter(
        &mut group,
        "substitute",
        &CharsRandomAugmenter::new(
            TextAction::Substitute,
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            None,
            Some(AlphabetModel::from_locale_str("en")),
            false,
        ),
    );
    bench_text_augmenter(
        &mut group,
        "swap",
        &CharsRandomAugmenter::new(
            TextAction::Swap,
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            None,
            None,
            false,
        ),
    );
    bench_text_augmenter(
        &mut group,
        "delete",
        &CharsRandomAugmenter::new(
            TextAction::Delete,
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            None,
            None,
            false,
        ),
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
