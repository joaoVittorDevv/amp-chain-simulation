#!/usr/bin/env python3
"""Fail when a Criterion mean estimate regresses beyond the configured limit."""

from __future__ import annotations

import json
import sys
from pathlib import Path


def estimate(path: Path) -> float:
    with path.open(encoding="utf-8") as handle:
        return float(json.load(handle)["mean"]["point_estimate"])


def main() -> int:
    if len(sys.argv) != 4:
        print(
            "usage: check_regressions.py <criterion-dir> <baseline> <threshold-percent>",
            file=sys.stderr,
        )
        return 2

    criterion_dir = Path(sys.argv[1])
    baseline_name = sys.argv[2]
    threshold = float(sys.argv[3]) / 100.0
    current_estimates = sorted(criterion_dir.glob("**/new/estimates.json"))

    if not current_estimates:
        print(f"no Criterion estimates found below {criterion_dir}", file=sys.stderr)
        return 2

    regressions: list[tuple[str, float]] = []
    missing: list[Path] = []

    for current_path in current_estimates:
        benchmark_dir = current_path.parent.parent
        baseline_path = benchmark_dir / baseline_name / "estimates.json"
        if not baseline_path.is_file():
            missing.append(baseline_path)
            continue

        baseline = estimate(baseline_path)
        current = estimate(current_path)
        change = current / baseline - 1.0
        name = benchmark_dir.relative_to(criterion_dir).as_posix()
        print(f"{name}: {change:+.2%}")
        if change > threshold:
            regressions.append((name, change))

    if missing:
        print(
            f"missing {len(missing)} '{baseline_name}' baseline estimate(s); "
            f"run `make bench-baseline BENCH_BASELINE={baseline_name}` first",
            file=sys.stderr,
        )
        return 2

    if regressions:
        print(
            f"performance gate failed: {len(regressions)} benchmark(s) regressed "
            f"by more than {threshold:.2%}",
            file=sys.stderr,
        )
        for name, change in regressions:
            print(f"  {name}: {change:+.2%}", file=sys.stderr)
        return 1

    print(f"performance gate passed: no regression exceeded {threshold:.2%}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
