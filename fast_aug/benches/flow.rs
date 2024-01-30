use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use fast_aug::flow::*;
use fast_aug::text::*;
use std::sync::Arc;

mod common;
use common::{bench_text_augmenter, get_config};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("text");
    group.sampling_mode(SamplingMode::Flat);
    bench_text_augmenter(
        &mut group,
        "SequentialAugmenter",
        &SequentialAugmenter::new(vec![
            Arc::new(WordsRandomSwapAugmenter::new(TextAugmentParameters::default(), None)),
            Arc::new(CharsRandomSwapAugmenter::new(
                TextAugmentParameters::default(),
                TextAugmentParameters::default(),
                None,
            )),
            Arc::new(WordsRandomDeleteAugmenter::new(TextAugmentParameters::default(), None)),
            Arc::new(CharsRandomDeleteAugmenter::new(
                TextAugmentParameters::default(),
                TextAugmentParameters::default(),
                None,
            )),
        ]),
    );
    bench_text_augmenter(
        &mut group,
        "SelectorAugmenter",
        &SelectorAugmenter::new(
            vec![
                Arc::new(WordsRandomSwapAugmenter::new(TextAugmentParameters::default(), None)),
                Arc::new(CharsRandomSwapAugmenter::new(
                    TextAugmentParameters::default(),
                    TextAugmentParameters::default(),
                    None,
                )),
                Arc::new(WordsRandomDeleteAugmenter::new(TextAugmentParameters::default(), None)),
                Arc::new(CharsRandomDeleteAugmenter::new(
                    TextAugmentParameters::default(),
                    TextAugmentParameters::default(),
                    None,
                )),
            ],
            None,
        ),
    );
    bench_text_augmenter(
        &mut group,
        "ChanceAugmenter",
        &ChanceAugmenter::new(
            Arc::new(WordsRandomSwapAugmenter::new(TextAugmentParameters::default(), None)),
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
