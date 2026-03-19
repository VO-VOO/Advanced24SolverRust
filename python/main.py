from __future__ import annotations

import importlib
import json
import math
import subprocess
import sys
import time
from concurrent.futures import ProcessPoolExecutor
from itertools import combinations_with_replacement
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parent
SRC_DIR = ROOT / "src"
RESULTS_PATH = ROOT / "results.json"
WORKER_COUNT = 8
CARD_VALUES = range(1, 14)
CARDS_PER_HAND = 4

if str(SRC_DIR) not in sys.path:
    sys.path.insert(0, str(SRC_DIR))


def all_combinations() -> list[tuple[int, int, int, int]]:
    return list(combinations_with_replacement(CARD_VALUES, CARDS_PER_HAND))


def chunked(seq: list[tuple[int, int, int, int]], chunks: int) -> list[list[tuple[int, int, int, int]]]:
    chunk_size = math.ceil(len(seq) / chunks)
    return [seq[index : index + chunk_size] for index in range(0, len(seq), chunk_size)]


def ensure_solver_built() -> None:
    try:
        importlib.import_module("solver")
        return
    except ModuleNotFoundError:
        pass

    subprocess.run(
        [sys.executable, "setup.py", "build_ext", "--inplace"],
        cwd=ROOT,
        check=True,
    )
    importlib.invalidate_caches()
    importlib.import_module("solver")


def solve_batch(batch: Iterable[tuple[int, int, int, int]]) -> dict[str, str]:
    solver = importlib.import_module("solver")
    results: dict[str, str] = {}
    for numbers in batch:
        expression = solver.solve_cards(numbers)
        results[",".join(map(str, numbers))] = expression if expression is not None else "无解"
    return results


def write_results(results: dict[str, str]) -> None:
    ordered_items = sorted(results.items(), key=lambda item: tuple(map(int, item[0].split(","))))
    records = []
    for cards_key, expression in ordered_items:
        records.append(
            {
                "cards": [int(value) for value in cards_key.split(",")],
                "cards_key": cards_key,
                "solved": expression != "无解",
                "expression": expression,
            }
        )

    payload = {
        "metadata": {
            "target": 24,
            "tolerance": 1e-6,
            "worker_count": WORKER_COUNT,
            "combination_count": len(records),
        },
        "results": records,
    }
    RESULTS_PATH.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")


def main() -> None:
    start = time.perf_counter()
    ensure_solver_built()

    combinations = all_combinations()
    batches = chunked(combinations, WORKER_COUNT)
    merged: dict[str, str] = {}

    with ProcessPoolExecutor(max_workers=WORKER_COUNT) as executor:
        for batch_result in executor.map(solve_batch, batches):
            merged.update(batch_result)

    write_results(merged)
    elapsed = time.perf_counter() - start
    print(f"完成 {len(combinations)} 种组合计算，结果已写入 {RESULTS_PATH.name}")
    print(f"总耗时: {elapsed:.6f} 秒")


if __name__ == "__main__":
    main()
