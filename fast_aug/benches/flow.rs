use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use fast_aug::flow::*;
use fast_aug::text::*;
use std::sync::Arc;

mod common;
use common::{bench_text_augmenter, get_config};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("SequentialAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "text",
        &SequentialAugmenter::new(vec![
            Arc::new(WordsRandomAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::default(),
                None,
            )),
            Arc::new(CharsRandomAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::default(),
                TextAugmentParameters::default(),
                None,
            )),
            Arc::new(WordsRandomAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::default(),
                None,
            )),
            Arc::new(CharsRandomAugmenter::new(
                TextAction::Delete,
                TextAugmentParameters::default(),
                TextAugmentParameters::default(),
                None,
            )),
        ]),
    );
    group.finish();

    let mut group = c.benchmark_group("SelectorAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "text",
        &SelectorAugmenter::new(
            vec![
                Arc::new(WordsRandomAugmenter::new(
                    TextAction::Swap,
                    TextAugmentParameters::default(),
                    None,
                )),
                Arc::new(CharsRandomAugmenter::new(
                    TextAction::Swap,
                    TextAugmentParameters::default(),
                    TextAugmentParameters::default(),
                    None,
                )),
                Arc::new(WordsRandomAugmenter::new(
                    TextAction::Delete,
                    TextAugmentParameters::default(),
                    None,
                )),
                Arc::new(CharsRandomAugmenter::new(
                    TextAction::Delete,
                    TextAugmentParameters::default(),
                    TextAugmentParameters::default(),
                    None,
                )),
            ],
            None,
        ),
    );
    group.finish();

    let mut group = c.benchmark_group("ChanceAugmenter");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "text",
        &ChanceAugmenter::new(
            Arc::new(WordsRandomAugmenter::new(
                TextAction::Swap,
                TextAugmentParameters::default(),
                None,
            )),
            0.5,
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
