#!/usr/bin/env python3
"""Validate repository governance and quality-model consistency."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def require_file(path: str) -> str:
    target = ROOT / path
    if not target.is_file():
        fail(f"required file is missing: {path}")
    return target.read_text(encoding="utf-8")


def validate_json(path: str) -> dict[str, object]:
    try:
        return json.loads(require_file(path))
    except json.JSONDecodeError as error:
        fail(f"invalid JSON in {path}: {error}")


def main() -> None:
    manifest = validate_json("governance/github-manifest.json")
    issues = manifest.get("issues")
    if not isinstance(issues, dict):
        fail("governance manifest must contain an issues object")

    for required_issue in ("19", "37", "38", "39", "40", "41", "42"):
        if required_issue not in issues:
            fail(f"governance manifest does not manage issue #{required_issue}")

    quality_model = require_file(
        ".context/quality-attributes/system-quality-model.md"
    )
    rows = [
        line
        for line in quality_model.splitlines()
        if line.startswith("|") and "Quality attribute" not in line and "---" not in line
    ]
    if len(rows) < 20:
        fail("system quality model contains fewer than 20 governed attributes")

    allowed_statuses = {"Planned", "Defined", "Implemented", "Enforced", "Evidenced", "Accepted"}
    for row in rows:
        columns = [column.strip() for column in row.strip("|").split("|")]
        if len(columns) != 7:
            fail(f"quality-model row has {len(columns)} columns instead of 7: {row}")
        if columns[-1] not in allowed_statuses:
            fail(f"unsupported quality maturity status {columns[-1]!r}")
        if any(not value for value in columns):
            fail(f"quality-model row contains an empty required field: {row}")

    agents = require_file("AGENTS.md")
    for required_heading in (
        "## Single application core",
        "## Testing and coverage",
        "## Distributed retail runtime and synchronization",
        "## Edge fleet control and deployment",
    ):
        if required_heading not in agents:
            fail(f"AGENTS.md is missing {required_heading}")

    traceability = require_file("governance/uc-bok-traceability.yaml")
    if not re.search(r"repository:\s*nomed/uc-bok", traceability):
        fail("UC-BoK traceability does not declare nomed/uc-bok")

    for fixture in (
        "fixtures/contracts/v1/basket/add-product-request.valid.json",
        "fixtures/contracts/v1/basket/add-product-response.valid.json",
    ):
        validate_json(fixture)

    print(f"validated {len(rows)} quality attributes and {len(issues)} managed issues")


if __name__ == "__main__":
    main()
