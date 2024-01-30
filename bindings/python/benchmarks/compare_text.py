from __future__ import annotations

import argparse
import platform
import time
import warnings
from collections.abc import Callable, Generator
from multiprocessing import Process, Queue
from typing import Any

import matplotlib.pyplot as plt
import pandas as pd
import psutil
import seaborn as sns
from tqdm import tqdm

from common import get_text_data

warnings.filterwarnings("ignore", category=FutureWarning, module="seaborn.*")


DEFAULT_REPEAT_COUNT = 10
REPEAT_COUNT = DEFAULT_REPEAT_COUNT
WORDS = ["hello", "world", "goodbye", "cruel", "world", "i", "love", "you"]


def system_info() -> None:
    print("System Information")
    print(f"  System: {platform.system()}")
    print(f"  Release: {platform.release()}")
    print(f"  Machine: {platform.machine()}")
    print(f"  Processor: {platform.processor()}")


def cpu_info() -> None:
    print("CPU Information")
    print(f"  Physical cores: {psutil.cpu_count(logical=False)}")
    print(f"  Total cores: {psutil.cpu_count(logical=True)}")
    cpu_freq = psutil.cpu_freq()
    print(f"  Max Frequency: {cpu_freq.max:.0f}Mhz")
    print(f"  Min Frequency: {cpu_freq.min:.0f}Mhz")


def ram_info() -> None:
    print("RAM Information")
    ram = psutil.virtual_memory()
    print(f"  Total: {ram.total / 2**30:.0f}GB")


def measure_function_time(queue: Queue, function: Callable, *args: Any, **kwargs: Any) -> None:
    time.sleep(0.5)  # delay for 500 ms for more accurate RAM measurement

    try:
        start = time.perf_counter()
        _ = function(*args, **kwargs)
        elapsed = time.perf_counter() - start
    except Exception:
        elapsed = None

    time.sleep(0.1)  # delay for 100 ms for more accurate RAM measurement
    queue.put(elapsed)


def monitor_process(pid: int, time_limit: int = 30) -> int | None:
    p = psutil.Process(pid)
    start_time = time.perf_counter()
    max_memory = 0
    while True:
        try:
            mem = p.memory_info().rss
            if mem > max_memory:
                max_memory = mem
            time.sleep(0.01)  # sample every 10 ms
            if time.perf_counter() - start_time > time_limit:
                print(f"Terminating process {pid} due to time limit")
                p.terminate()
                time.sleep(5)
                if p.is_running():
                    print(f"Process {p.pid} did not terminate gracefully; attempting to kill")
                    p.kill()
                return None
        except psutil.NoSuchProcess:
            break
    return max_memory


def measure_function_time_repeat(
    name: str, repeat: int, function: Callable, args: Any, kwargs: Any
) -> Generator[dict[str, float], None, None]:
    for _ in range(repeat):
        queue = Queue()
        starting_memory = psutil.Process().memory_info().rss
        process = Process(target=measure_function_time, args=(queue, function, *args), kwargs=kwargs)
        process.start()

        max_memory = monitor_process(process.pid)

        if process.is_alive():
            process.join()

        if not queue.empty():
            elapsed = queue.get()
        else:
            elapsed = None

        if max_memory is None:
            elapsed = None

        if elapsed is None:
            max_memory = None

        if process.exitcode != 0:
            elapsed = None
            max_memory = None

        yield {
            "name": name,
            "time": elapsed,
            "memory": max_memory - starting_memory if max_memory is not None else None,
            **kwargs,
        }


def _fast_aug_words_insert(batched: bool = False) -> None:
    from fast_aug.text import WordsRandomInsertAugmenter

    augmenter = WordsRandomInsertAugmenter(0.3, vocabulary=WORDS)
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_words_insert(batched: bool = False) -> None:
    raise NotImplementedError("nlpaug does not support insert")


def _fasttextaug_words_insert(batched: bool = False) -> None:
    raise NotImplementedError("fasttextaug does not support insert")


def measure_words_insert() -> pd.DataFrame:
    method_name = "words\nrandom\ninsert"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_words_insert),
            ("nlpaug", _nlpaug_words_insert),
            ("fasttextaug", _fasttextaug_words_insert),
            # ("augmenty", None),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def _fast_aug_words_substitute(batched: bool = False) -> None:
    from fast_aug.text import WordsRandomSubstituteAugmenter

    augmenter = WordsRandomSubstituteAugmenter(0.3, vocabulary=WORDS)
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_words_substitute(batched: bool = False) -> None:
    from nlpaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="substitute", aug_p=0.3, target_words=WORDS)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _fasttextaug_words_substitute(batched: bool = False) -> None:
    from fasttextaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="substitute", aug_p=0.3, target_words=WORDS)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_words_substitute() -> pd.DataFrame:
    method_name = "words\nrandom\nsubstitute"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_words_substitute),
            ("nlpaug", _nlpaug_words_substitute),
            ("fasttextaug", _fasttextaug_words_substitute),
            # ("augmenty", None),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def _fast_aug_words_swap(batched: bool = False) -> None:
    from fast_aug.text import WordsRandomSwapAugmenter

    augmenter = WordsRandomSwapAugmenter(0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_words_swap(batched: bool = False) -> None:
    from nlpaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="swap", aug_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _fasttextaug_words_swap(batched: bool = False) -> None:
    from fasttextaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="swap", aug_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _augmenty_words_swap(batched: bool = False) -> None:
    import augmenty
    from spacy.lang.en import English

    nlp = English()
    augmenter = augmenty.load("token_swap_v1", level=0.3)
    text_data = get_text_data()

    if batched:
        list(augmenty.texts(text_data, augmenter=augmenter, nlp=nlp))
    else:
        for text in text_data:
            list(augmenty.texts([text], augmenter=augmenter, nlp=nlp))


def measure_words_swap() -> pd.DataFrame:
    method_name = "words\nrandom\nswap"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_words_swap),
            ("nlpaug", _nlpaug_words_swap),
            ("fasttextaug", _fasttextaug_words_swap),
            # ("augmenty", _augmenty_words_swap),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def _fast_aug_words_delete(batched: bool = False) -> None:
    from fast_aug.text import WordsRandomDeleteAugmenter

    augmenter = WordsRandomDeleteAugmenter(0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_words_delete(batched: bool = False) -> None:
    from nlpaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="delete", aug_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _fasttextaug_words_delete(batched: bool = False) -> None:
    from fasttextaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="delete", aug_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_words_delete() -> pd.DataFrame:
    method_name = "words\nrandom\ndelete"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_words_delete),
            ("nlpaug", _nlpaug_words_delete),
            ("fasttextaug", _fasttextaug_words_delete),
            # ("augmenty", None),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def _fast_aug_chars_insert(batched: bool = False) -> None:
    from fast_aug.text import CharsRandomInsertAugmenter

    augmenter = CharsRandomInsertAugmenter(0.3, 0.3, locale="en")
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_chars_insert(batched: bool = False) -> None:
    from nlpaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="insert", aug_char_p=0.3, aug_word_p=0.3)  # ascii only, so english
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _fasttextaug_chars_insert(batched: bool = False) -> None:
    from fasttextaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="insert", aug_char_p=0.3, aug_word_p=0.3, lang="en")
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_chars_insert() -> pd.DataFrame:
    method_name = "chars\nrandom\ninsert"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_chars_insert),
            ("nlpaug", _nlpaug_chars_insert),
            ("fasttextaug", _fasttextaug_chars_insert),
            # ("augmenty", None),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def _fast_aug_chars_substitute(batched: bool = False) -> None:
    from fast_aug.text import CharsRandomSubstituteAugmenter

    augmenter = CharsRandomSubstituteAugmenter(0.3, 0.3, locale="en")
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_chars_substitute(batched: bool = False) -> None:
    from nlpaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="substitute", aug_char_p=0.3, aug_word_p=0.3)  # ascii only, so english
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _fasttextaug_chars_substitute(batched: bool = False) -> None:
    from fasttextaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="substitute", aug_char_p=0.3, aug_word_p=0.3, lang="en")
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_chars_substitute() -> pd.DataFrame:
    method_name = "chars\nrandom\nsubstitute"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_chars_substitute),
            ("nlpaug", _nlpaug_chars_substitute),
            ("fasttextaug", _fasttextaug_chars_substitute),
            # ("augmenty", None),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def _fast_aug_chars_swap(batched: bool = False) -> None:
    from fast_aug.text import CharsRandomSwapAugmenter

    augmenter = CharsRandomSwapAugmenter(0.3, 0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_chars_swap(batched: bool = False) -> None:
    from nlpaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="swap", aug_char_p=0.3, aug_word_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _fasttextaug_chars_swap(batched: bool = False) -> None:
    from fasttextaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="swap", aug_char_p=0.3, aug_word_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _augmenty_chars_swap(batched: bool = False) -> None:
    import augmenty
    from spacy.lang.en import English

    nlp = English()
    augmenter = augmenty.load("token_swap_v1", level=0.3)
    text_data = get_text_data()

    if batched:
        list(augmenty.texts(text_data, augmenter=augmenter, nlp=nlp))
    else:
        for text in text_data:
            list(augmenty.texts([text], augmenter=augmenter, nlp=nlp))


def measure_chars_swap() -> pd.DataFrame:
    method_name = "chars\nrandom\nswap"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_chars_swap),
            ("nlpaug", _nlpaug_chars_swap),
            ("fasttextaug", _fasttextaug_chars_swap),
            # ("augmenty", _augmenty_chars_swap),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def _fast_aug_chars_delete(batched: bool = False) -> None:
    from fast_aug.text import CharsRandomDeleteAugmenter

    augmenter = CharsRandomDeleteAugmenter(0.3, 0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment_batch(text_data)
    if not batched:
        for d in text_data:
            augmenter.augment(d)


def _nlpaug_chars_delete(batched: bool = False) -> None:
    from nlpaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="delete", aug_char_p=0.3, aug_word_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def _fasttextaug_chars_delete(batched: bool = False) -> None:
    from fasttextaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="delete", aug_char_p=0.3, aug_word_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_chars_delete() -> pd.DataFrame:
    method_name = "chars\nrandom\ndelete"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast-aug", _fast_aug_chars_delete),
            ("nlpaug", _nlpaug_chars_delete),
            ("fasttextaug", _fasttextaug_chars_delete),
            # ("augmenty", None),
        ]:
            for result in tqdm(
                measure_function_time_repeat(name, REPEAT_COUNT, function, (), {"batched": batched}),
                total=REPEAT_COUNT,
                desc=f"{name} [{'batched' if batched else 'single'}]",
            ):
                results.append(result)

    df = pd.DataFrame(results, columns=["method", "name", "time", "memory", "batched"])
    df["method"] = method_name
    return df


def draw_barplot_time_memory(df: pd.DataFrame, output_name: str, hue_order: list[str]) -> None:
    num_samples = len(get_text_data())

    # Plot settings
    sns.set(style="whitegrid")

    # Group by method, name, and batched, then calculate mean time and memory
    df = df.reset_index()
    df["memory_MB"] = df["memory"] / (1024 * 1024)
    not_batched_df = df[df["batched"] == False]  # noqa: E712
    batched_df = df[df["batched"] == True]  # noqa: E712

    # Create subplots for time
    fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(20, 6), sharey="row")
    sns.barplot(x="method", y="time", hue="name", data=not_batched_df, ax=axes[0], hue_order=hue_order)
    axes[0,].set_title("Average Time - Not Batched")
    axes[0].set_ylabel("Average Time (s)")
    axes[0].set_xlabel("Method")
    axes[0].legend(loc="upper left", title="Library")
    axes[0].text(
        0.99,
        1.03,
        f"{num_samples/1000:.1f}k samples, average by {REPEAT_COUNT} runs",
        transform=axes[0].transAxes,
        ha="right",
        va="top",
        fontsize=10,
        color="grey",
    )

    sns.barplot(x="method", y="time", hue="name", data=batched_df, ax=axes[1], hue_order=hue_order)
    axes[1].set_title("Average Time - Batched")
    axes[1].set_ylabel("Average Time (s)")
    axes[1].set_xlabel("Method")
    axes[1].legend(loc="upper left", title="Library")
    axes[1].text(
        0.99,
        1.03,
        f"{num_samples/1000:.1f}k samples, average by {REPEAT_COUNT} runs",
        transform=axes[1].transAxes,
        ha="right",
        va="top",
        fontsize=10,
        color="grey",
    )

    plt.tight_layout()
    plt.savefig(output_name + "-time" + ".svg")

    # Create subplots for memory
    fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(20, 6), sharey="row")
    sns.barplot(x="method", y="memory_MB", hue="name", data=not_batched_df, ax=axes[0], hue_order=hue_order)
    axes[0].set_title("Average Memory - Not Batched")
    axes[0].set_ylabel("Average Memory (MB)")
    axes[0].set_xlabel("Method")
    axes[0].legend(loc="upper left", title="Library")
    axes[0].text(
        0.99,
        1.03,
        f"{num_samples/1000:.1f}k samples, average by {REPEAT_COUNT} runs",
        transform=axes[0].transAxes,
        ha="right",
        va="top",
        fontsize=9,
        color="grey",
    )

    sns.barplot(x="method", y="memory_MB", hue="name", data=batched_df, ax=axes[1], hue_order=hue_order)
    axes[1].set_title("Average Memory - Batched")
    axes[1].set_ylabel("Average Memory (MB)")
    axes[1].set_xlabel("Method")
    axes[1].legend(loc="upper left", title="Library")
    axes[1].text(
        0.99,
        1.03,
        f"{num_samples/1000:.1f}k samples, average by {REPEAT_COUNT} runs",
        transform=axes[1].transAxes,
        ha="right",
        va="top",
        fontsize=9,
        color="grey",
    )

    plt.tight_layout()
    plt.savefig(output_name + "-memory" + ".svg")

    # Show the plot in interactive window (both time and memory) - if interactive window is not available, show nothing
    if plt.isinteractive():
        plt.show()
        plt.close()


if __name__ == "__main__":
    # Parse arguments
    parser = argparse.ArgumentParser()
    parser.add_argument("--repeat", type=int, default=DEFAULT_REPEAT_COUNT)
    args = parser.parse_args()
    REPEAT_COUNT = args.repeat
    print(f"Repeat count: {REPEAT_COUNT}")

    # Display the information
    system_info()
    cpu_info()
    ram_info()
    print()

    df_word_insert = measure_words_insert()
    print(df_word_insert)
    df_word_substitute = measure_words_substitute()
    print(df_word_substitute)
    df_word_swap = measure_words_swap()
    print(df_word_swap)
    df_word_delete = measure_words_delete()
    print(df_word_delete)
    df_char_insert = measure_chars_insert()
    print(df_char_insert)
    df_char_substitute = measure_chars_substitute()
    print(df_char_substitute)
    df_char_swap = measure_chars_swap()
    print(df_char_swap)
    df_char_delete = measure_chars_delete()
    print(df_char_delete)

    draw_barplot_time_memory(
        pd.concat(
            [
                df_word_insert,
                df_word_substitute,
                df_word_swap,
                df_word_delete,
                df_char_insert,
                df_char_substitute,
                df_char_swap,
                df_char_delete,
            ],
        ),
        "comparison-python-text",
        hue_order=["fast-aug", "fasttextaug", "nlpaug"],
    )
