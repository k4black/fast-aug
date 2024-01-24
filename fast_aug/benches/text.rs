use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use fast_aug::text::*;

mod common;
use common::{bench_text_augmenter, get_config};

// Criterion entry point
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandomWordsAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "swap",
        &RandomWordsAugmenter::new(TextAction::Swap, TextAugmentParameters::default(), None),
    );
    bench_text_augmenter(
        &mut group,
        "delete",
        &RandomWordsAugmenter::new(TextAction::Delete, TextAugmentParameters::default(), None),
    );
    group.finish();

    let mut group = c.benchmark_group("RandomCharsAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "swap",
        &RandomCharsAugmenter::new(
            TextAction::Swap,
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            None,
        ),
    );
    bench_text_augmenter(
        &mut group,
        "delete",
        &RandomCharsAugmenter::new(
            TextAction::Delete,
            TextAugmentParameters::default(),
            TextAugmentParameters::default(),
            None,
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
