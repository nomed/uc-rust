#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VALIDATOR = ROOT / "scripts" / "validate_records.py"


def run(*paths: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(VALIDATOR), *paths],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


def main() -> int:
    valid = run(
        "docs/knowledge/records/CR-0001-basket-capability.md",
        "docs/knowledge/records/RRR-0001-operation-invocation.md",
    )
    if valid.returncode != 0:
        print(valid.stdout)
        print(valid.stderr, file=sys.stderr)
        return 1

    invalid = run("tests/fixtures/records/invalid-unknown-relation.md")
    if invalid.returncode != 1:
        print("invalid fixture did not fail as expected", file=sys.stderr)
        print(invalid.stdout)
        print(invalid.stderr, file=sys.stderr)
        return 1
    expected = "[relation-type] unsupported relation: 'related_to'"
    if expected not in invalid.stderr:
        print(f"missing expected diagnostic: {expected}", file=sys.stderr)
        print(invalid.stderr, file=sys.stderr)
        return 1

    print("record validator tests passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
