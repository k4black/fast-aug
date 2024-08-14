from pytest_benchmark.fixture import BenchmarkFixture

import fast_aug
from common import bench_text_augmenter


WORDS = ["hello", "world", "goodbye", "cruel", "world", "i", "love", "you"]


class TestWordsRandomAugmenter:
    def test_insert(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.WordsRandomInsertAugmenter(0.3, vocabulary=WORDS)
        bench_text_augmenter(benchmark, augmenter)

    def test_substitute(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.WordsRandomSubstituteAugmenter(0.3, vocabulary=WORDS)
        bench_text_augmenter(benchmark, augmenter)

    def test_swap(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.WordsRandomSwapAugmenter(0.3)
        bench_text_augmenter(benchmark, augmenter)

    def test_delete(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.WordsRandomDeleteAugmenter(0.3)
        bench_text_augmenter(benchmark, augmenter)


class TestCharsRandomAugmenter:
    def test_insert(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.CharsRandomInsertAugmenter(0.3, 0.3, locale="en")
        bench_text_augmenter(benchmark, augmenter)

    def test_substitute(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.CharsRandomSubstituteAugmenter(0.3, 0.3, locale="en")
        bench_text_augmenter(benchmark, augmenter)

    def test_swap(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.CharsRandomSwapAugmenter(0.3, 0.3)
        bench_text_augmenter(benchmark, augmenter)

    def test_delete(self, benchmark: BenchmarkFixture) -> None:
        augmenter = fast_aug.text.CharsRandomDeleteAugmenter(0.3, 0.3)
        bench_text_augmenter(benchmark, augmenter)
