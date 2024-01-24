from common import bench_text_augmenter
from pytest_benchmark.fixture import BenchmarkFixture

import fast_aug


class TestSequentialAugmenter:
    def test_text(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.flow.SequentialAugmenter(
            [
                fast_aug.text.RandomWordsAugmenter(action="swap"),
                fast_aug.text.RandomCharsAugmenter(action="swap"),
                fast_aug.text.RandomWordsAugmenter(action="delete"),
                fast_aug.text.RandomCharsAugmenter(action="delete"),
            ]
        )
        bench_text_augmenter(benchmark, augmenter)


class TestSelectorAugmenter:
    def test_text(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.flow.SelectorAugmenter(
            [
                fast_aug.text.RandomWordsAugmenter(action="swap"),
                fast_aug.text.RandomCharsAugmenter(action="swap"),
                fast_aug.text.RandomWordsAugmenter(action="delete"),
                fast_aug.text.RandomCharsAugmenter(action="delete"),
            ]
        )
        bench_text_augmenter(benchmark, augmenter)


class TestChanceAugmenter:
    def test_text(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.flow.ChanceAugmenter(
            fast_aug.text.RandomWordsAugmenter(action="swap"),
            0.5,
        )
        bench_text_augmenter(benchmark, augmenter)
