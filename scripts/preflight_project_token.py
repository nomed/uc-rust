#!/usr/bin/env python3
"""Validate that PROJECT_TOKEN can access the configured governance Project."""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

MANIFEST = Path("governance/github-manifest.json")


def run(token: str, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["gh", *args, "--format", "json"],
        check=False,
        capture_output=True,
        text=True,
        env={**os.environ, "GH_TOKEN": token},
    )


def main() -> int:
    token = os.environ.get("PROJECT_TOKEN", "")
    if not token:
        print(
            "PROJECT_TOKEN is missing. Create a repository Actions secret with Project v2 access to the configured governance Project.",
            file=sys.stderr,
        )
        return 1

    project = json.loads(MANIFEST.read_text(encoding="utf-8"))["project"]
    number = str(project["number"])
    owner = project["owner"]

    preferred = run(token, "project", "view", number, "--owner", owner)
    if preferred.returncode == 0:
        return 0

    if "unknown owner type" in (preferred.stderr or "").lower():
        fallback = run(token, "project", "view", number)
        if fallback.returncode == 0:
            print(
                f"Project owner flag is unsupported for this token context; continuing without --owner for Project #{number}",
                flush=True,
            )
            return 0
        print(
            fallback.stderr.strip() or "PROJECT_TOKEN cannot access the configured project",
            file=sys.stderr,
        )
        return fallback.returncode

    print(
        preferred.stderr.strip() or "PROJECT_TOKEN cannot access the configured project",
        file=sys.stderr,
    )
    return preferred.returncode


if __name__ == "__main__":
    raise SystemExit(main())
