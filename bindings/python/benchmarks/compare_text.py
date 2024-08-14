from __future__ import annotations

import argparse
import importlib
import platform
import time
import warnings
from collections.abc import Callable
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


def measure_function_time(queue: Queue, function: Callable, *args: Any, **kwargs: Any) -> None:  # type: ignore
    time.sleep(0.5)  # delay for 500 ms for more accurate RAM measurement

    try:
        start = time.perf_counter()
        _ = function(*args, **kwargs)
        elapsed = time.perf_counter() - start
    except Exception:
        elapsed = None
        raise

    time.sleep(0.1)  # delay for 100 ms for more accurate RAM measurement
    queue.put(elapsed)


def monitor_process(pid: int | None, time_limit: int = 30) -> int | None:
    if pid is None:
        return None

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
    name: str,
    method_name: str,
    repeat: int,
    function: Callable,
    args: Any,
    kwargs: Any,  # type: ignore
) -> pd.DataFrame:
    results: list[dict[str, Any]] = []

    for _ in tqdm(range(repeat), desc=f"{method_name} {name}".replace("\n", "-")):
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

        results.append(
            {
                "name": name,
                "method": method_name,
                "time": elapsed,
                "memory": max_memory - starting_memory if max_memory is not None else None,
                **kwargs,
            }
        )

    return pd.DataFrame(results)


def run_fast_aug(cls_str: str, cls_args: Any, cls_kwargs: Any, batched: bool) -> None:
    module_path, class_name = cls_str.rsplit(".", 1)
    module = importlib.import_module(module_path)
    cls = getattr(module, class_name)

    text_data = get_text_data()
    augmenter = cls(*cls_args, **cls_kwargs)
    if batched:
        augmenter.augment_batch(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_fast_aug_augmenter(name: str, method_name: str, cls_str: str, args: Any, kwargs: Any) -> pd.DataFrame:
    # not batched
    df_not_batched = measure_function_time_repeat(
        name, method_name, REPEAT_COUNT, run_fast_aug, (cls_str, args, kwargs), {"batched": False}
    )
    # batched
    df_batched = measure_function_time_repeat(
        name, method_name, REPEAT_COUNT, run_fast_aug, (cls_str, args, kwargs), {"batched": True}
    )
    return pd.concat([df_not_batched, df_batched])


def run_nlpaug(cls_str: str, cls_args: Any, cls_kwargs: Any, batched: bool) -> None:
    module_path, class_name = cls_str.rsplit(".", 1)
    module = importlib.import_module(module_path)
    cls = getattr(module, class_name)

    text_data = get_text_data()
    augmenter = cls(*cls_args, **cls_kwargs)
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_nlpaug_augmenter(name: str, method_name: str, cls_str: str, args: Any, kwargs: Any) -> pd.DataFrame:
    # not batched
    df_not_batched = measure_function_time_repeat(
        name, method_name, REPEAT_COUNT, run_nlpaug, (cls_str, args, kwargs), {"batched": False}
    )
    # batched
    df_batched = measure_function_time_repeat(
        name, method_name, REPEAT_COUNT, run_nlpaug, (cls_str, args, kwargs), {"batched": True}
    )
    return pd.concat([df_not_batched, df_batched])


def run_fasttextaug(cls_str: str, cls_args: Any, cls_kwargs: Any, batched: bool) -> None:
    module_path, class_name = cls_str.rsplit(".", 1)
    module = importlib.import_module(module_path)
    cls = getattr(module, class_name)

    text_data = get_text_data()
    augmenter = cls(*cls_args, **cls_kwargs)
    if batched:
        augmenter.augment(text_data)
    else:
        for d in text_data:
            augmenter.augment(d)


def measure_fasttextaug_augmenter(name: str, method_name: str, cls_str: str, args: Any, kwargs: Any) -> pd.DataFrame:
    # not batched
    df_not_batched = measure_function_time_repeat(
        name, method_name, REPEAT_COUNT, run_fasttextaug, (cls_str, args, kwargs), {"batched": False}
    )
    # batched
    df_batched = measure_function_time_repeat(
        name, method_name, REPEAT_COUNT, run_fasttextaug, (cls_str, args, kwargs), {"batched": True}
    )
    return pd.concat([df_not_batched, df_batched])


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

    # collect results
    df = pd.concat(
        [
            measure_fast_aug_augmenter(
                "fast-aug",
                "words\nrandom\ninsert",
                "fast_aug.text.WordsRandomInsertAugmenter",
                (0.3, WORDS),
                {},
            ),
            # not implemented for nlpaug
            # not implemented for fasttextaug
            measure_fast_aug_augmenter(
                "fast-aug",
                "words\nrandom\nsubstitute",
                "fast_aug.text.WordsRandomSubstituteAugmenter",
                (0.3, WORDS),
                {},
            ),
            measure_nlpaug_augmenter(
                "nlpaug",
                "words\nrandom\nsubstitute",
                "nlpaug.augmenter.word.RandomWordAug",
                ("substitute",),
                {"aug_p": 0.3, "target_words": WORDS},
            ),
            measure_fasttextaug_augmenter(
                "fasttextaug",
                "words\nrandom\nsubstitute",
                "fasttextaug.augmenter.word.RandomWordAug",
                ("substitute",),
                {"aug_p": 0.3, "target_words": WORDS},
            ),
            measure_fast_aug_augmenter(
                "fast-aug",
                "words\nrandom\ndelete",
                "fast_aug.text.WordsRandomDeleteAugmenter",
                (0.3,),
                {},
            ),
            measure_nlpaug_augmenter(
                "nlpaug",
                "words\nrandom\ndelete",
                "nlpaug.augmenter.word.RandomWordAug",
                ("delete",),
                {"aug_p": 0.3},
            ),
            measure_fasttextaug_augmenter(
                "fasttextaug",
                "words\nrandom\ndelete",
                "fasttextaug.augmenter.word.RandomWordAug",
                ("delete",),
                {"aug_p": 0.3},
            ),
            measure_fast_aug_augmenter(
                "fast-aug",
                "words\nrandom\nswap",
                "fast_aug.text.WordsRandomSwapAugmenter",
                (0.3,),
                {},
            ),
            measure_nlpaug_augmenter(
                "nlpaug",
                "words\nrandom\nswap",
                "nlpaug.augmenter.word.RandomWordAug",
                ("swap",),
                {"aug_p": 0.3},
            ),
            measure_fasttextaug_augmenter(
                "fasttextaug",
                "words\nrandom\nswap",
                "fasttextaug.augmenter.word.RandomWordAug",
                ("swap",),
                {"aug_p": 0.3},
            ),
            measure_fast_aug_augmenter(
                "fast-aug",
                "chars\nrandom\ninsert",
                "fast_aug.text.CharsRandomInsertAugmenter",
                (0.3, 0.3, "en"),
                {},
            ),
            measure_nlpaug_augmenter(
                "nlpaug",
                "chars\nrandom\ninsert",
                "nlpaug.augmenter.char.RandomCharAug",
                ("insert",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3},  # only english
            ),
            measure_fasttextaug_augmenter(
                "fasttextaug",
                "chars\nrandom\ninsert",
                "fasttextaug.augmenter.char.RandomCharAug",
                ("insert",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3, "lang": "en"},
            ),
            measure_fast_aug_augmenter(
                "fast-aug",
                "chars\nrandom\nsubstitute",
                "fast_aug.text.CharsRandomSubstituteAugmenter",
                (0.3, 0.3, "en"),
                {},
            ),
            measure_nlpaug_augmenter(
                "nlpaug",
                "chars\nrandom\nsubstitute",
                "nlpaug.augmenter.char.RandomCharAug",
                ("substitute",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3},  # only english
            ),
            measure_fasttextaug_augmenter(
                "fasttextaug",
                "chars\nrandom\nsubstitute",
                "fasttextaug.augmenter.char.RandomCharAug",
                ("substitute",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3, "lang": "en"},
            ),
            measure_fast_aug_augmenter(
                "fast-aug",
                "chars\nrandom\ndelete",
                "fast_aug.text.CharsRandomDeleteAugmenter",
                (0.3, 0.3),
                {},
            ),
            measure_nlpaug_augmenter(
                "nlpaug",
                "chars\nrandom\ndelete",
                "nlpaug.augmenter.char.RandomCharAug",
                ("delete",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3},
            ),
            measure_fasttextaug_augmenter(
                "fasttextaug",
                "chars\nrandom\ndelete",
                "fasttextaug.augmenter.char.RandomCharAug",
                ("delete",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3},
            ),
            measure_fast_aug_augmenter(
                "fast-aug",
                "chars\nrandom\nswap",
                "fast_aug.text.CharsRandomSwapAugmenter",
                (0.3, 0.3),
                {},
            ),
            measure_nlpaug_augmenter(
                "nlpaug",
                "chars\nrandom\nswap",
                "nlpaug.augmenter.char.RandomCharAug",
                ("swap",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3},
            ),
            measure_fasttextaug_augmenter(
                "fasttextaug",
                "chars\nrandom\nswap",
                "fasttextaug.augmenter.char.RandomCharAug",
                ("swap",),
                {"aug_char_p": 0.3, "aug_word_p": 0.3},
            ),
        ]
    )
    print(df)

    draw_barplot_time_memory(
        df,
        "comparison-python-text",
        hue_order=["fast-aug", "fasttextaug", "nlpaug"],
    )
