#!/usr/bin/env python3
"""Validate UC Rust capability realization manifests against the governed schema.

Usage:
    python tools/validate_capability_realization_manifests.py path/to/manifest.json [...]

Requires the `jsonschema` package. The script intentionally accepts JSON only so the
validated representation is deterministic and does not depend on YAML parser rules.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

SCHEMA_PATH = Path("governance/schemas/capability-realization-manifest.schema.json")


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise RuntimeError(f"file not found: {path}") from exc
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"invalid JSON in {path}: {exc}") from exc


def main(argv: list[str]) -> int:
    if not argv:
        print("usage: validate_capability_realization_manifests.py MANIFEST [...]")
        return 2

    try:
        from jsonschema import Draft202012Validator
    except ImportError:
        print("error: install the 'jsonschema' package to run this validator", file=sys.stderr)
        return 2

    schema = load_json(SCHEMA_PATH)
    validator = Draft202012Validator(schema)
    failed = False

    for raw_path in argv:
        path = Path(raw_path)
        try:
            instance = load_json(path)
        except RuntimeError as exc:
            failed = True
            print(f"FAIL {exc}", file=sys.stderr)
            continue

        errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
        if errors:
            failed = True
            print(f"FAIL {path}", file=sys.stderr)
            for error in errors:
                location = "/".join(str(part) for part in error.absolute_path) or "<root>"
                print(f"  {location}: {error.message}", file=sys.stderr)
        else:
            print(f"OK   {path}")

    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
