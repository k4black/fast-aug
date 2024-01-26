from common import bench_text_augmenter
from pytest_benchmark.fixture import BenchmarkFixture

import fast_aug


class TestWordsRandomAugmenter:
    def test_swap(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.WordsRandomAugmenter(action="swap")
        bench_text_augmenter(benchmark, augmenter)

    def test_delete(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.WordsRandomAugmenter(action="delete")
        bench_text_augmenter(benchmark, augmenter)


class TestCharsRandomAugmenter:
    def test_swap(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.CharsRandomAugmenter(action="swap")
        bench_text_augmenter(benchmark, augmenter)

    def test_delete(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.CharsRandomAugmenter(action="delete")
        bench_text_augmenter(benchmark, augmenter)
