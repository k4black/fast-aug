from pytest_benchmark.fixture import BenchmarkFixture

import fast_aug
from common import bench_text_augmenter


class TestSequentialAugmenter:
    def test_text(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.flow.SequentialAugmenter(
            [
                fast_aug.text.WordsRandomSwapAugmenter(0.3),
                fast_aug.text.CharsRandomSwapAugmenter(0.3, 0.3),
                fast_aug.text.WordsRandomDeleteAugmenter(0.3),
                fast_aug.text.CharsRandomDeleteAugmenter(0.3, 0.3),
            ]
        )
        bench_text_augmenter(benchmark, augmenter)


class TestSelectorAugmenter:
    def test_text(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.flow.SelectorAugmenter(
            [
                fast_aug.text.WordsRandomSwapAugmenter(0.3),
                fast_aug.text.CharsRandomSwapAugmenter(0.3, 0.3),
                fast_aug.text.WordsRandomDeleteAugmenter(0.3),
                fast_aug.text.CharsRandomDeleteAugmenter(0.3, 0.3),
            ]
        )
        bench_text_augmenter(benchmark, augmenter)


class TestChanceAugmenter:
    def test_text(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.flow.ChanceAugmenter(
            fast_aug.text.WordsRandomSwapAugmenter(0.3),
            0.5,
        )
        bench_text_augmenter(benchmark, augmenter)
