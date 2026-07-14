#!/usr/bin/env python3
"""Fail release checks when SolMath's public Cargo feature contract drifts."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EXPECTED_VERSION = "0.2.0"
EXPECTED_FEATURES = {
    "american-kbi": ["transcendental"],
    "asian": ["transcendental"],
    "barrier": ["transcendental"],
    "bivariate": ["transcendental"],
    "bs": ["transcendental"],
    "complex": ["transcendental"],
    "default": ["transcendental"],
    "full": [
        "transcendental",
        "complex",
        "bs",
        "iv",
        "barrier",
        "asian",
        "nig",
        "heston",
        "sabr",
        "pool",
        "bivariate",
        "american-kbi",
        "rainbow",
    ],
    "heston": ["bs"],
    "iv": ["bs"],
    "nig": ["transcendental"],
    "pade-iv": ["iv"],
    "pool": ["transcendental"],
    "rainbow": ["bivariate"],
    "sabr": ["transcendental"],
    "table-gen": ["bivariate"],
    "transcendental": [],
}


def metadata() -> dict:
    result = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    packages = json.loads(result.stdout)["packages"]
    return next(package for package in packages if package["name"] == "solmath")


def source_features() -> set[str]:
    pattern = re.compile(r'feature\s*=\s*"([^"]+)"')
    found: set[str] = set()
    for source in (ROOT / "src").rglob("*.rs"):
        found.update(pattern.findall(source.read_text(encoding="utf-8")))
    return found


def verify(package: dict) -> None:
    errors: list[str] = []
    actual_features = package["features"]

    if package["version"] != EXPECTED_VERSION:
        errors.append(
            f"release version is {package['version']!r}; expected {EXPECTED_VERSION!r}"
        )
    if package["dependencies"]:
        errors.append("the published library must remain dependency-free")
    if set(actual_features) != set(EXPECTED_FEATURES):
        missing = sorted(set(EXPECTED_FEATURES) - set(actual_features))
        extra = sorted(set(actual_features) - set(EXPECTED_FEATURES))
        errors.append(f"feature names drifted (missing={missing}, extra={extra})")

    for feature, expected_dependencies in EXPECTED_FEATURES.items():
        actual_dependencies = actual_features.get(feature)
        if actual_dependencies is not None and set(actual_dependencies) != set(
            expected_dependencies
        ):
            errors.append(
                f"feature {feature!r} expands to {actual_dependencies}; "
                f"expected {expected_dependencies}"
            )

    undeclared = source_features() - set(actual_features)
    if undeclared:
        errors.append(f"source uses undeclared Cargo features: {sorted(undeclared)}")

    full = set(actual_features.get("full", []))
    for excluded in ("table-gen", "pade-iv"):
        if excluded in full:
            errors.append(f"offline/experimental feature {excluded!r} leaked into 'full'")
    if "complex" in actual_features.get("default", []):
        errors.append("complex arithmetic must not be linked by default")

    if errors:
        raise SystemExit("feature contract failed:\n- " + "\n- ".join(errors))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--list",
        action="store_true",
        help="print independently checkable feature names after validation",
    )
    args = parser.parse_args()
    package = metadata()
    verify(package)
    if args.list:
        print("\n".join(sorted(package["features"])))
    else:
        print("feature contract: ok")


if __name__ == "__main__":
    main()
