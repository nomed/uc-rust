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


def owner_candidates(owner: str) -> list[list[str]]:
    candidates: list[list[str]] = [["--owner", owner]]
    if owner != "@me":
        candidates.append(["--owner", "@me"])
    candidates.append([])
    return candidates


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

    failures: list[subprocess.CompletedProcess[str]] = []
    for owner_args in owner_candidates(owner):
        attempt = run(token, "project", "view", number, *owner_args)
        if attempt.returncode == 0:
            if owner_args != ["--owner", owner]:
                if owner_args:
                    print(
                        f"Project owner override {' '.join(owner_args)} succeeded for Project #{number}",
                        flush=True,
                    )
                else:
                    print(
                        f"Project owner flag is unsupported for this token context; continuing without --owner for Project #{number}",
                        flush=True,
                    )
            return 0
        failures.append(attempt)

    message = next(
        (failed.stderr.strip() for failed in failures if failed.stderr and failed.stderr.strip()),
        "PROJECT_TOKEN cannot access the configured project",
    )
    print(message, file=sys.stderr)
    return failures[0].returncode if failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
