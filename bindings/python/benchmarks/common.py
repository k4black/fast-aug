# Reduce noise, actually improve perf in our case.
import os
from pathlib import Path

from pytest_benchmark.fixture import BenchmarkFixture

from fast_aug.base import BaseAugmenter

os.environ["OPENBLAS_NUM_THREADS"] = "1"


FILENAME = Path(__file__).parent / "../../../test_data/tweet_eval_sentiment_test_text.txt"


def bench_text_augmenter(benchmark: BenchmarkFixture, augmenter: BaseAugmenter) -> None:
    # load data
    text_data = [line.strip() for line in open(FILENAME, "r").readlines()]

    # benchmark
    def augment_data() -> None:
        for d in text_data:
            augmenter.augment(d)

    benchmark(augment_data)
