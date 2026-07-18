#!/usr/bin/env python3
"""Fail unless an LCOV report has complete line and branch coverage."""

from __future__ import annotations

import sys
from pathlib import Path


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: check_coverage.py <lcov.info>")

    report = Path(sys.argv[1])
    if not report.is_file():
        raise SystemExit(f"coverage report not found: {report}")

    totals = {"LF": 0, "LH": 0, "BRF": 0, "BRH": 0}
    for line in report.read_text(encoding="utf-8").splitlines():
        key, separator, value = line.partition(":")
        if separator and key in totals:
            totals[key] += int(value)

    if totals["LF"] == 0:
        raise SystemExit("coverage report contains no executable lines")
    if totals["LH"] != totals["LF"]:
        raise SystemExit(
            f"line coverage is incomplete: {totals['LH']}/{totals['LF']} lines covered"
        )
    if totals["BRF"] == 0:
        raise SystemExit("coverage report contains no measurable branches")
    if totals["BRH"] != totals["BRF"]:
        raise SystemExit(
            f"branch coverage is incomplete: {totals['BRH']}/{totals['BRF']} branches covered"
        )

    print(
        "coverage accepted: "
        f"{totals['LH']}/{totals['LF']} lines, "
        f"{totals['BRH']}/{totals['BRF']} branches"
    )


if __name__ == "__main__":
    main()
