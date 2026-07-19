#!/usr/bin/env python3
"""Require every substantive pull request to update a durable session record."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import PurePosixPath

SESSION_ROOT = PurePosixPath(".context/sessions")
EXEMPT_PREFIXES = (
    ".context/sessions/",
    ".context/handoffs/",
)
EXEMPT_FILES = {
    ".gitignore",
}


def changed_files(base_ref: str) -> list[str]:
    result = subprocess.run(
        ["git", "diff", "--name-only", f"{base_ref}...HEAD"],
        check=True,
        capture_output=True,
        text=True,
    )
    return [line.strip() for line in result.stdout.splitlines() if line.strip()]


def is_substantive(path: str) -> bool:
    return path not in EXEMPT_FILES and not path.startswith(EXEMPT_PREFIXES)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-ref", required=True)
    args = parser.parse_args()

    files = changed_files(args.base_ref)
    substantive = [path for path in files if is_substantive(path)]
    sessions = [
        path
        for path in files
        if PurePosixPath(path).parent == SESSION_ROOT
        and PurePosixPath(path).name.startswith("SESSION-")
        and path.endswith(".md")
    ]

    if substantive and not sessions:
        print("Substantive changes require an updated .context/sessions/SESSION-*.md record.", file=sys.stderr)
        print("Changed substantive files:", file=sys.stderr)
        for path in substantive:
            print(f"- {path}", file=sys.stderr)
        return 1

    print(f"Session accountability satisfied: {len(sessions)} session record(s) changed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
