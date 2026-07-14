#!/usr/bin/env python3
"""Validate compiled fixed-point Kim Boundary Integration against QuantLib QdFp."""

from __future__ import annotations

import argparse
import json
import math
import pathlib
import subprocess

import numpy as np
import QuantLib as ql

from american_quantlib_reference import Contract, QdFpSurfacePricer, contracts


ROOT = pathlib.Path(__file__).resolve().parents[1]
SCALE = 10**12


def metric(values: list[float]) -> dict[str, float | int]:
    data = np.sort(np.abs(np.asarray(values, dtype=np.float64)))
    return {
        "count": int(data.size),
        "median": float(np.quantile(data, 0.50)),
        "p95": float(np.quantile(data, 0.95)),
        "p99": float(np.quantile(data, 0.99)),
        "max": float(data[-1]),
        "mean": float(np.mean(data)),
    }


def scaled(value: float) -> int:
    return round(value * SCALE)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--batch-binary",
        type=pathlib.Path,
        default=ROOT / "target/release/examples/american_kbi_batch",
    )
    parser.add_argument(
        "--report",
        type=pathlib.Path,
        default=ROOT / "benchmark/american_kbi_runtime_accuracy_report.json",
    )
    parser.add_argument("--moneyness-points", type=int, default=33)
    parser.add_argument("--contract-seed", type=lambda value: int(value, 0))
    parser.add_argument("--contract-count", type=int)
    args = parser.parse_args()

    selected = contracts(0x51D3, 24)
    contract_source = "deterministic held-out sample seed 0x51d3"
    if args.contract_seed is not None:
        if args.contract_count is None:
            raise SystemExit("--contract-seed requires --contract-count")
        selected = contracts(args.contract_seed, args.contract_count)
        contract_source = f"deterministic sample seed {args.contract_seed:#x}"

    grid = np.exp(np.linspace(-0.75, 0.75, args.moneyness_points))
    pricer = QdFpSurfacePricer()
    lines: list[str] = []
    truth: list[tuple[str, int, float, float]] = []
    strike = 100.0
    for contract_index, contract in enumerate(selected):
        maturity = contract.days / 365.0
        for kind, is_call in (("call", True), ("put", False)):
            qdfp = strike * pricer.surface(grid, contract, contract.days, is_call)
            for normalized_spot, exact in zip(grid, qdfp):
                lines.append(" ".join([
                    kind,
                    str(scaled(strike * normalized_spot)),
                    str(scaled(strike)),
                    str(scaled(contract.r)),
                    str(scaled(contract.q)),
                    str(scaled(contract.sigma)),
                    str(scaled(maturity)),
                ]))
                truth.append((kind, contract_index, float(normalized_spot), float(exact)))

    process = subprocess.run(
        [str(args.batch_binary)],
        input="\n".join(lines) + "\n",
        text=True,
        capture_output=True,
        check=True,
    )
    outputs = process.stdout.splitlines()
    if len(outputs) != len(truth):
        raise RuntimeError(f"runtime returned {len(outputs)} rows for {len(truth)} inputs")

    errors: dict[str, list[float]] = {"call": [], "put": []}
    worst: dict[str, dict[str, object] | None] = {"call": None, "put": None}
    rows = [asdict_contract(contract) for contract in selected]
    for output, (kind, contract_index, normalized_spot, exact) in zip(outputs, truth):
        if output.startswith("ERR:"):
            raise RuntimeError(f"runtime error for {kind}/{contract_index}: {output}")
        actual = int(output) / SCALE
        residual = actual - exact
        absolute = abs(residual)
        errors[kind].append(residual)
        if worst[kind] is None or absolute > worst[kind]["absolute_error_dollars"]:
            worst[kind] = {
                "contract_index": contract_index,
                "contract": rows[contract_index],
                "normalized_spot": normalized_spot,
                "runtime_price_dollars": actual,
                "qdfp_price_dollars": exact,
                "signed_error_dollars": residual,
                "absolute_error_dollars": absolute,
            }

    digest_line = next(
        line for line in (ROOT / "src/american_kbi_data.rs").read_text().splitlines()
        if line.startswith("// SHA-256:")
    )
    report = {
        "method": "compiled Rust Q40 Kim Boundary Integration versus QuantLib QdFp accurateScheme",
        "runtime_design": "18-node sqrt-time boundary; six-node singularity-cancelled Gaussian history; nine-node QdFp-regularized empirical premium cubature; log-boundary interpolation; all parameter-dependent work on-chain",
        "quantlib_version": ql.__version__,
        "artifact_sha256": digest_line.split(":", 1)[1].strip(),
        "contract_source": contract_source,
        "contract_count": len(selected),
        "moneyness_points_per_contract": args.moneyness_points,
        "validation_log_moneyness": [-0.75, 0.75],
        "price_comparisons_per_leg": len(errors["call"]),
        "absolute_error_dollars_at_100_strike": {
            kind: metric(values) for kind, values in errors.items()
        },
        "worst_cases": worst,
    }
    args.report.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report["absolute_error_dollars_at_100_strike"], indent=2))
    print(f"report={args.report}")


def asdict_contract(contract: Contract) -> dict[str, float | int]:
    return {
        "r": contract.r,
        "q": contract.q,
        "sigma": contract.sigma,
        "days": contract.days,
    }


if __name__ == "__main__":
    main()
