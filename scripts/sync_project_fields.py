#!/usr/bin/env python3
"""Set managed GitHub Project field values from the governance manifest.

This script intentionally uses the native Project `Status` field rather than a
parallel custom Stage field. It runs only after project access has been proven.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

MANIFEST = Path("governance/github-manifest.json")


class ProjectSyncError(RuntimeError):
    """Project reconciliation failure."""


def gh(token: str, *args: str) -> Any:
    env = os.environ.copy()
    env["GH_TOKEN"] = token
    result = subprocess.run(
        ["gh", *args, "--format", "json"],
        check=False,
        capture_output=True,
        text=True,
        env=env,
    )
    if result.returncode != 0:
        raise ProjectSyncError(result.stderr.strip() or "GitHub CLI command failed")
    return json.loads(result.stdout)


def values(payload: Any, key: str) -> list[dict[str, Any]]:
    if isinstance(payload, list):
        return payload
    if isinstance(payload, dict) and isinstance(payload.get(key), list):
        return payload[key]
    raise ProjectSyncError(f"Unexpected GitHub CLI response for {key}")


def option_ids(field: dict[str, Any]) -> dict[str, str]:
    return {option["name"]: option["id"] for option in field.get("options") or []}


def main() -> int:
    token = os.environ.get("PROJECT_TOKEN", "")
    if not token:
        raise ProjectSyncError("PROJECT_TOKEN is missing")

    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    project = manifest["project"]
    number = str(project["number"])
    owner = project["owner"]

    project_view = gh(token, "project", "view", number, "--owner", owner)
    project_id = project_view["id"]

    fields = values(
        gh(token, "project", "field-list", number, "--owner", owner, "--limit", "100"),
        "fields",
    )
    fields_by_name = {field["name"]: field for field in fields}

    required = {"Status", "Priority", "Type", "Area", "Release", "Size"}
    missing = sorted(required - fields_by_name.keys())
    if missing:
        raise ProjectSyncError(f"Missing managed Project fields: {missing}")

    items = values(
        gh(token, "project", "item-list", number, "--owner", owner, "--limit", "500"),
        "items",
    )
    items_by_number: dict[int, dict[str, Any]] = {}
    for item in items:
        content = item.get("content") or {}
        if content.get("type") == "Issue" and content.get("repository") == manifest["repository"]:
            items_by_number[int(content["number"])] = item

    for issue_number_text, definition in manifest["issues"].items():
        issue_number = int(issue_number_text)
        item = items_by_number.get(issue_number)
        if item is None:
            raise ProjectSyncError(f"Issue #{issue_number} is missing from Project #{number}")

        for field_name, value in definition.get("project", {}).items():
            field = fields_by_name[field_name]
            option_id = option_ids(field).get(value)
            if option_id is None:
                available = sorted(option_ids(field))
                raise ProjectSyncError(
                    f"Field {field_name} has no option {value!r}; available={available}"
                )
            gh(
                token,
                "project",
                "item-edit",
                "--id",
                item["id"],
                "--project-id",
                project_id,
                "--field-id",
                field["id"],
                "--single-select-option-id",
                option_id,
            )
            print(f"set issue #{issue_number} {field_name}={value}", flush=True)

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProjectSyncError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        raise SystemExit(1)
