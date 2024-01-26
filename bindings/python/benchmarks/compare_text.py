from __future__ import annotations

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
from common import get_text_data
from tqdm import tqdm

warnings.filterwarnings("ignore", category=FutureWarning, module="seaborn.*")


REPEAT_COUNT = 10


def system_info() -> None:
    print("System Information")
    print(f"  System: {platform.system()}")
    print(f"  Release: {platform.release()}")
    print(f"  Machine: {platform.machine()}")
    print(f"  Processor: {platform.processor()}")


def cpu_info() -> None:
    print("\nCPU Information")
    print(f"  Physical cores: {psutil.cpu_count(logical=False)}")
    print(f"  Total cores: {psutil.cpu_count(logical=True)}")
    cpu_freq = psutil.cpu_freq()
    print(f"  Max Frequency: {cpu_freq.max:.0f}Mhz")
    print(f"  Min Frequency: {cpu_freq.min:.0f}Mhz")


def ram_info() -> None:
    print("\nRAM Information")
    ram = psutil.virtual_memory()
    print(f"  Total: {ram.total / 2**30:.0f}GB")


def measure_function_time(queue: Queue, function: Callable, *args: Any, **kwargs: Any) -> None:
    time.sleep(0.1)  # delay for 100 ms for more accurate RAM measurement

    try:
        start = time.perf_counter()
        _ = function(*args, **kwargs)
        elapsed = time.perf_counter() - start
    except Exception:
        elapsed = None

    time.sleep(0.1)  # delay for 100 ms for more accurate RAM measurement
    queue.put(elapsed)


def measure_function_time_repeat(
    name: str, repeat: int, function: Callable, args: Any, kwargs: Any
) -> Generator[dict[str, float], None, None]:
    for _ in range(repeat):
        queue = Queue()
        starting_memory = psutil.Process().memory_info().rss
        process = Process(target=measure_function_time, args=(queue, function, *args), kwargs=kwargs)
        process.start()

        max_memory = monitor_process(process.pid)

        process.join()
        elapsed = queue.get()

        if elapsed is None:
            elapsed = None
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


def monitor_process(pid: int) -> int:
    p = psutil.Process(pid)
    max_memory = 0
    while True:
        try:
            mem = p.memory_info().rss
            if mem > max_memory:
                max_memory = mem
            time.sleep(0.01)  # sample every 10 ms
        except psutil.NoSuchProcess:
            break
    return max_memory


def _fast_aug_words_swap(batched: bool = False) -> None:
    from fast_aug.text import RandomWordsAugmenter

    augmenter = RandomWordsAugmenter("swap", 0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
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


def _fastnlpaug_words_swap(batched: bool = False) -> None:
    from fasttextaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="swap", aug_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_words_swap() -> pd.DataFrame:
    method_name = "words_swap"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast_aug", _fast_aug_words_swap),
            ("nlpaug", _nlpaug_words_swap),
            ("fastnlpaug", _fastnlpaug_words_swap),
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
    from fast_aug.text import RandomWordsAugmenter

    augmenter = RandomWordsAugmenter("delete", 0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
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


def _fastnlpaug_words_delete(batched: bool = False) -> None:
    from fasttextaug.augmenter.word import RandomWordAug

    augmenter = RandomWordAug(action="delete", aug_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_words_delete() -> pd.DataFrame:
    method_name = "words_delete"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast_aug", _fast_aug_words_delete),
            ("nlpaug", _nlpaug_words_delete),
            ("fastnlpaug", _fastnlpaug_words_delete),
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
    from fast_aug.text import RandomCharsAugmenter

    augmenter = RandomCharsAugmenter("swap", 0.3, 0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
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


def _fastnlpaug_chars_swap(batched: bool = False) -> None:
    from fasttextaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="swap", aug_char_p=0.3, aug_word_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_chars_swap() -> pd.DataFrame:
    method_name = "chars_swap"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast_aug", _fast_aug_chars_swap),
            ("nlpaug", _nlpaug_chars_swap),
            ("fastnlpaug", _fastnlpaug_chars_swap),
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
    from fast_aug.text import RandomCharsAugmenter

    augmenter = RandomCharsAugmenter("delete", 0.3, 0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
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


def _fastnlpaug_chars_delete(batched: bool = False) -> None:
    from fasttextaug.augmenter.char import RandomCharAug

    augmenter = RandomCharAug(action="delete", aug_char_p=0.3, aug_word_p=0.3)
    text_data = get_text_data()
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_chars_delete() -> pd.DataFrame:
    method_name = "chars_delete"
    results = []
    for batched in [False, True]:
        for name, function in [
            ("fast_aug", _fast_aug_chars_delete),
            ("nlpaug", _nlpaug_chars_delete),
            ("fastnlpaug", _fastnlpaug_chars_delete),
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


def draw_barplot_time_memory(df: pd.DataFrame, output_name: str) -> None:
    # Plot settings
    sns.set(style="whitegrid")

    # Group by method, name, and batched, then calculate mean time and memory
    df["memory_MB"] = df["memory"] / (1024 * 1024)
    grouped_df = df.groupby(["method", "name", "batched"]).agg({"time": "mean", "memory_MB": "mean"}).reset_index()
    not_batched_df = grouped_df[grouped_df["batched"] is False]
    batched_df = grouped_df[grouped_df["batched"] is True]

    # Create subplots for time
    fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(20, 6), sharey="row")
    sns.barplot(x="method", y="time", hue="name", data=not_batched_df, ax=axes[0])
    axes[0,].set_title(f"Average Time - Not Batched (average {REPEAT_COUNT} runs, lower is better)")
    axes[0].set_ylabel("Average Time (s)")
    axes[0].set_xlabel("Method")
    axes[0].legend(loc="upper left", title="Library")

    sns.barplot(x="method", y="time", hue="name", data=batched_df, ax=axes[1])
    axes[1].set_title(f"Average Time - Batched (average {REPEAT_COUNT} runs, lower is better)")
    axes[1].set_ylabel("Average Time (s)")
    axes[1].set_xlabel("Method")
    axes[1].legend(loc="upper left", title="Library")

    plt.tight_layout()
    plt.savefig(output_name + "_time" + ".svg")

    # Create subplots for memory
    fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(20, 6), sharey="row")
    sns.barplot(x="method", y="memory_MB", hue="name", data=not_batched_df, ax=axes[0])
    axes[0].set_title(f"Average Memory - Not Batched (average {REPEAT_COUNT} runs, lower is better)")
    axes[0].set_ylabel("Average Memory (MB)")
    axes[0].set_xlabel("Method")
    axes[0].legend(loc="upper left", title="Library")

    sns.barplot(x="method", y="memory_MB", hue="name", data=batched_df, ax=axes[1])
    axes[1].set_title(f"Average Memory - Batched (average {REPEAT_COUNT} runs, lower is better)")
    axes[1].set_ylabel("Average Memory (MB)")
    axes[1].set_xlabel("Method")
    axes[1].legend(loc="upper left", title="Library")

    plt.tight_layout()
    plt.savefig(output_name + "_memory" + ".svg")

    # Show the plot in interactive window (both time and memory) - if interactive window is not available, show nothing
    if plt.get_backend() != "agg" and plt.isinteractive():
        plt.show()
        plt.close()


if __name__ == "__main__":
    # Display the information
    system_info()
    cpu_info()
    ram_info()

    df_word_swap = measure_words_swap()
    print(df_word_swap)
    df_word_delete = measure_words_delete()
    print(df_word_delete)
    df_char_swap = measure_chars_swap()
    print(df_char_swap)
    df_char_delete = measure_chars_delete()
    print(df_char_delete)

    draw_barplot_time_memory(
        pd.concat(
            [
                df_word_swap,
                df_word_delete,
                df_char_swap,
                df_char_delete,
            ],
        ),
        "comparison_text",
    )
