#!/usr/bin/env python3
"""Run the compiled Asian/TWAP pricer over the retained 100K/10K corpora."""

from __future__ import annotations

import argparse
import json
import statistics
import subprocess
from pathlib import Path


SCALE = 10**12
ROOT = Path(__file__).resolve().parents[1]
DEFAULT_BINARY = ROOT / "target/release/examples/asian_batch"
DEFAULT_FILES = (
    ROOT / "benchmark/prod_asian_vectors.json",
    ROOT / "benchmark/adv_asian_vectors.json",
)
INPUT_KEYS = (
    "s",
    "k",
    "r",
    "q",
    "sigma",
    "t",
    "averaging_time",
    "fixed_average",
    "fixed_weight",
)
EXPECTED_KEYS = (
    "expected_call",
    "expected_put",
    "expected_mean",
    "expected_log_variance",
)


def percentile(values: list[int], percent: int) -> int:
    return values[min(len(values) - 1, (len(values) * percent + 99) // 100 - 1)]


def validate(path: Path, binary: Path) -> dict:
    payload = json.loads(path.read_text())
    vectors = payload["vectors"]
    process = subprocess.run(
        [str(binary)],
        input="\n".join(" ".join(row[key] for key in INPUT_KEYS) for row in vectors) + "\n",
        text=True,
        capture_output=True,
        check=True,
    )
    lines = process.stdout.splitlines()
    if len(lines) != len(vectors):
        raise RuntimeError(f"{path.name}: got {len(lines)} outputs for {len(vectors)} vectors")

    deviations = [[] for _ in EXPECTED_KEYS]
    errors = []
    categories: dict[str, dict[str, int]] = {}
    max_cases: dict[str, dict | None] = {key: None for key in EXPECTED_KEYS}
    for index, (row, line) in enumerate(zip(vectors, lines)):
        if line.startswith("ERR"):
            errors.append({"index": index, "category": row["category"], "error": line})
            continue
        actual = list(map(int, line.split()))
        expected = [int(row[key]) for key in EXPECTED_KEYS]
        category = row["category"]
        category_max = categories.setdefault(category, {key: 0 for key in EXPECTED_KEYS})
        for output_index, key in enumerate(EXPECTED_KEYS):
            difference = abs(actual[output_index] - expected[output_index])
            deviations[output_index].append(difference)
            category_max[key] = max(category_max[key], difference)
            current_max = max_cases[key]
            if current_max is None or difference > current_max["difference_raw"]:
                max_cases[key] = {
                    "index": index,
                    "category": category,
                    "difference_raw": difference,
                    "actual_raw": actual[output_index],
                    "expected_raw": expected[output_index],
                    "inputs": {input_key: row[input_key] for input_key in INPUT_KEYS},
                }

    metrics = {}
    for key, values in zip(EXPECTED_KEYS, deviations):
        values.sort()
        metrics[key.removeprefix("expected_")] = {
            "median_raw": int(statistics.median(values)) if values else 0,
            "p95_raw": percentile(values, 95) if values else 0,
            "p99_raw": percentile(values, 99) if values else 0,
            "max_raw": values[-1] if values else 0,
            "max_real": (values[-1] / SCALE) if values else 0,
        }
    return {
        "file": str(path.relative_to(ROOT)),
        "reference": payload["meta"]["reference"],
        "vectors": len(vectors),
        "accepted": len(vectors) - len(errors),
        "errors": errors[:20],
        "error_count": len(errors),
        "metrics": metrics,
        "max_cases": max_cases,
        "category_max_raw": categories,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument("--report", type=Path, default=ROOT / "benchmark/asian_accuracy_report.json")
    parser.add_argument("vectors", type=Path, nargs="*", default=list(DEFAULT_FILES))
    args = parser.parse_args()

    if not args.binary.exists():
        subprocess.run(
            [
                "cargo",
                "build",
                "--release",
                "--no-default-features",
                "--features",
                "asian",
                "--example",
                "asian_batch",
            ],
            cwd=ROOT,
            check=True,
        )
    report = {
        "binary": str(args.binary.relative_to(ROOT)),
        "scale": SCALE,
        "corpora": [validate(path, args.binary) for path in args.vectors],
    }
    report["total_vectors"] = sum(corpus["vectors"] for corpus in report["corpora"])
    report["total_errors"] = sum(corpus["error_count"] for corpus in report["corpora"])
    rendered = json.dumps(report, indent=2, sort_keys=True) + "\n"
    args.report.write_text(rendered)
    print(rendered, end="")
    if report["total_errors"]:
        raise SystemExit("compiled Asian pricer rejected retained corpus vectors")


if __name__ == "__main__":
    main()
