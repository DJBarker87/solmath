#!/usr/bin/env python3
"""Generate reproducible arithmetic-Asian references with QuantLib 1.41.

The corpus prices unseasoned, continuously sampled arithmetic-average calls and
puts with QuantLib's ContinuousArithmeticAsianLevyEngine.  This is the exact
QuantLib counterpart of `arithmetic_asian_price(..., averaging_time=t,
fixed_weight=0)`; future-starting and partially fixed contracts remain covered
by the independent high-precision moment tests because QuantLib's Levy engine
does not expose the former state directly.

Outputs:
  benchmark/asian_quantlib_vectors.json       all 10,000 vectors
  tests/asian_quantlib_reference.rs           500 evenly sampled cargo tests

All persisted values are integer fixed point at SCALE=1e12.  Re-running this
file with QuantLib 1.41 is deterministic and rewrites both artifacts.
"""

import json
import math
import os
import random

import QuantLib as ql


SCALE = 10**12
SEED = 0x415349414E
COUNT = 10_000
TEST_COUNT = 500
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JSON_PATH = os.path.join(ROOT, "benchmark", "asian_quantlib_vectors.json")
RUST_PATH = os.path.join(ROOT, "tests", "asian_quantlib_reference.rs")


def fp(value):
    return int(round(value * SCALE))


def build_pricer():
    today = ql.Date(1, 1, 2025)
    ql.Settings.instance().evaluationDate = today
    day_count = ql.Actual365Fixed()
    spot = ql.SimpleQuote(100.0)
    rate = ql.SimpleQuote(0.05)
    dividend = ql.SimpleQuote(0.02)
    vol = ql.SimpleQuote(0.40)
    process = ql.BlackScholesMertonProcess(
        ql.QuoteHandle(spot),
        ql.YieldTermStructureHandle(
            ql.FlatForward(today, ql.QuoteHandle(dividend), day_count)
        ),
        ql.YieldTermStructureHandle(
            ql.FlatForward(today, ql.QuoteHandle(rate), day_count)
        ),
        ql.BlackVolTermStructureHandle(
            ql.BlackConstantVol(
                today, ql.NullCalendar(), ql.QuoteHandle(vol), day_count
            )
        ),
    )
    engine = ql.ContinuousArithmeticAsianLevyEngine(
        process, ql.QuoteHandle(ql.SimpleQuote(0.0)), today
    )

    def price(s, k, r, q, sigma, days):
        spot.setValue(s)
        rate.setValue(r)
        dividend.setValue(q)
        vol.setValue(sigma)
        exercise = ql.EuropeanExercise(today + days)
        result = []
        for side in (ql.Option.Call, ql.Option.Put):
            option = ql.ContinuousAveragingAsianOption(
                ql.Average.Arithmetic,
                today,
                ql.PlainVanillaPayoff(side, k),
                exercise,
            )
            option.setPricingEngine(engine)
            result.append(max(option.NPV(), 0.0))
        return result

    return price


def cases():
    # Explicit boundary/regression rows are followed by a deterministic,
    # stratified production corpus.
    rows = [
        (100.0, 100.0, 0.05, 0.02, 0.40, 365, "atm_1y"),
        (120.0, 100.0, 0.10, 0.04, 0.80, 730, "long_high_vol"),
        (80.0, 100.0, 0.0, 0.0, 0.20, 30, "short_otm"),
        (100.0, 100.0, 0.0, 0.0, 0.05, 1, "one_day_low_vol"),
        (50.0, 100.0, 0.12, 0.0, 1.50, 1825, "deep_otm"),
        (150.0, 100.0, 0.0, 0.12, 1.50, 1825, "deep_itm"),
    ]
    rng = random.Random(SEED)
    maturity_bands = [(1, 7), (8, 30), (31, 182), (183, 365), (366, 730), (731, 1825)]
    money_bands = [(0.50, 0.75), (0.75, 0.95), (0.95, 1.05), (1.05, 1.25), (1.25, 1.50)]
    vol_bands = [(0.05, 0.15), (0.15, 0.40), (0.40, 0.80), (0.80, 1.50)]
    cells = [
        (maturity, money, vol)
        for maturity in maturity_bands
        for money in money_bands
        for vol in vol_bands
    ]
    # Produce spare candidates because the QuantLib f64 engine can return NaN
    # in a few very deep-tail configurations; those are rejected below.
    while len(rows) < COUNT * 2:
        maturity, money, vol = cells[(len(rows) - 6) % len(cells)]
        k = rng.uniform(20.0, 500.0)
        rows.append(
            (
                k * rng.uniform(*money),
                k,
                rng.uniform(0.0, 0.12),
                rng.uniform(0.0, 0.12),
                rng.uniform(*vol),
                rng.randint(*maturity),
                "stratified",
            )
        )
    return rows


def generate():
    price = build_pricer()
    vectors = []
    for s, k, r, q, sigma, days, category in cases():
        call, put = price(s, k, r, q, sigma, days)
        if not (math.isfinite(call) and math.isfinite(put)):
            continue
        t = days / 365.0
        vectors.append(
            {
                "s": str(fp(s)),
                "k": str(fp(k)),
                "r": str(fp(r)),
                "q": str(fp(q)),
                "sigma": str(fp(sigma)),
                "t": str(fp(t)),
                "averaging_time": str(fp(t)),
                "fixed_average": "0",
                "fixed_weight": "0",
                "ql_call": str(fp(call)),
                "ql_put": str(fp(put)),
                "t_days": days,
                "category": category,
            }
        )
        if len(vectors) == COUNT:
            break

    if len(vectors) != COUNT:
        raise RuntimeError(f"generated only {len(vectors)} finite QuantLib vectors")

    payload = {
        "meta": {
            "reference": (
                f"QuantLib {ql.__version__} ContinuousArithmeticAsianLevyEngine"
            ),
            "quantlib_version": ql.__version__,
            "evaluation_date": "2025-01-01",
            "day_count": "Actual/365 (Fixed)",
            "average": "continuous arithmetic, unseasoned, averaging starts at valuation",
            "scale": SCALE,
            "seed": SEED,
            "count": len(vectors),
            "generator": "scripts/generate_asian_quantlib_vectors.py",
        },
        "vectors": vectors,
    }
    with open(JSON_PATH, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2)
        handle.write("\n")

    step = max(1, len(vectors) // TEST_COUNT)
    selected = vectors[::step][:TEST_COUNT]
    with open(RUST_PATH, "w", encoding="utf-8") as handle:
        handle.write(
            "//! Auto-generated QuantLib arithmetic-Asian reference tests.\n"
            "//! Source: scripts/generate_asian_quantlib_vectors.py\n\n"
            '#![cfg(feature = "asian")]\n\n'
            "use solmath::arithmetic_asian_price;\n\n"
            "#[test]\n"
            "fn matches_quantlib_1_41_continuous_arithmetic_levy_engine() {\n"
            "    const VECTORS: &[[u128; 11]] = &[\n"
        )
        for vector in selected:
            handle.write(
                "        ["
                + ", ".join(
                    vector[key]
                    for key in (
                        "s",
                        "k",
                        "r",
                        "q",
                        "sigma",
                        "t",
                        "averaging_time",
                        "fixed_average",
                        "fixed_weight",
                        "ql_call",
                        "ql_put",
                    )
                )
                + "],\n"
            )
        handle.write(
            "    ];\n"
            "    // Covers the full-corpus max (<595,000,000 raw), which occurs\n"
            "    // in QuantLib's cancellation-sensitive two-day Levy tail.\n"
            "    const TOLERANCE: u128 = 1_000_000_000; // $0.001\n"
            "    let mut max_call = (0u128, 0usize);\n"
            "    let mut max_put = (0u128, 0usize);\n"
            "    for (index, vector) in VECTORS.iter().enumerate() {\n"
            "        let [s, k, r, q, sigma, t, averaging_time, fixed_average, fixed_weight, expected_call, expected_put] = *vector;\n"
            "        let actual = arithmetic_asian_price(s, k, r, q, sigma, t, averaging_time, fixed_average, fixed_weight).unwrap();\n"
            "        let call_diff = actual.call.abs_diff(expected_call);\n"
            "        let put_diff = actual.put.abs_diff(expected_put);\n"
            "        if call_diff > max_call.0 { max_call = (call_diff, index); }\n"
            "        if put_diff > max_put.0 { max_put = (put_diff, index); }\n"
            "    }\n"
            "    eprintln!(\"max QuantLib diffs: call={} at {}, put={} at {}\", max_call.0, max_call.1, max_put.0, max_put.1);\n"
            "    assert!(max_call.0 <= TOLERANCE, \"QuantLib call max diff={} at {}\", max_call.0, max_call.1);\n"
            "    assert!(max_put.0 <= TOLERANCE, \"QuantLib put max diff={} at {}\", max_put.0, max_put.1);\n"
            "}\n"
        )

    print(f"QuantLib {ql.__version__}: {len(vectors):,} vectors -> {JSON_PATH}")
    print(f"Cargo subset: {len(selected):,} vectors -> {RUST_PATH}")


if __name__ == "__main__":
    if ql.__version__ != "1.41":
        raise SystemExit(f"QuantLib 1.41 required, found {ql.__version__}")
    generate()
