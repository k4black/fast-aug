from common import bench_text_augmenter
from pytest_benchmark.fixture import BenchmarkFixture

import fast_aug


class TestRandomWordsAugmenter:
    def test_swap(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.RandomWordsAugmenter(action="swap")
        bench_text_augmenter(benchmark, augmenter)

    def test_delete(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.RandomWordsAugmenter(action="delete")
        bench_text_augmenter(benchmark, augmenter)


class TestRandomCharsAugmenter:
    def test_swap(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.RandomCharsAugmenter(action="swap")
        bench_text_augmenter(benchmark, augmenter)

    def test_delete(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.RandomCharsAugmenter(action="delete")
        bench_text_augmenter(benchmark, augmenter)
