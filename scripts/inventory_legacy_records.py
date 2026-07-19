#!/usr/bin/env python3
"""Inventory and classify legacy ADR/RFC records for UC Rust 1.0 migration."""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

import yaml

ROOT = Path(__file__).resolve().parents[1]
REGISTRY_PATH = ROOT / "governance/record-migration-registry.yaml"
ID_RE = re.compile(r"\b(ADR|RFC)-(\d{4})\b", re.IGNORECASE)
TITLE_RE = re.compile(r"^#\s+(ADR|RFC)-\d{4}\s*(?:[—-]|:)\s+(.+?)\s*$", re.MULTILINE)
STATUS_RE = re.compile(r"^-\s*Status:\s*(.+?)\s*$", re.IGNORECASE | re.MULTILINE)


@dataclass(frozen=True)
class InventoryItem:
    path: str
    discovered_id: str | None
    canonical_id: str | None
    record_type: str | None
    title: str | None
    status: str | None
    disposition: str
    aliases: tuple[str, ...]
    diagnostics: tuple[str, ...]


def load_registry() -> dict[str, Any]:
    if not REGISTRY_PATH.is_file():
        raise SystemExit(f"missing migration registry: {REGISTRY_PATH.relative_to(ROOT)}")
    data = yaml.safe_load(REGISTRY_PATH.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise SystemExit("migration registry must be a YAML mapping")
    return data


def normalize_status(raw: str | None, registry: dict[str, Any]) -> str | None:
    if raw is None:
        return None
    token = raw.strip().split("—", 1)[0].strip().lower()
    normalized = registry.get("status_normalization", {}).get(token)
    return normalized


def explicit_by_path(registry: dict[str, Any]) -> dict[str, dict[str, Any]]:
    result: dict[str, dict[str, Any]] = {}
    for entry in registry.get("explicit_records", []):
        if isinstance(entry, dict) and isinstance(entry.get("path"), str):
            result[entry["path"]] = entry
    return result


def classify(path: Path, registry: dict[str, Any]) -> InventoryItem:
    relative = path.relative_to(ROOT).as_posix()
    text = path.read_text(encoding="utf-8")
    override = explicit_by_path(registry).get(relative, {})
    diagnostics: list[str] = []

    id_match = ID_RE.search(text) or ID_RE.search(path.name)
    discovered_id = id_match.group(0).upper() if id_match else None
    record_type = id_match.group(1).upper() if id_match else None

    title_match = TITLE_RE.search(text)
    title = override.get("title") or (title_match.group(2).strip() if title_match else None)
    raw_status_match = STATUS_RE.search(text)
    raw_status = raw_status_match.group(1).strip() if raw_status_match else None
    status = override.get("status") or normalize_status(raw_status, registry)

    if discovered_id is None:
        diagnostics.append("missing record identifier")
    if title is None:
        diagnostics.append("missing canonical heading/title")
    if raw_status is None and override.get("status") is None:
        diagnostics.append("missing legacy Status field")
    elif raw_status is not None and status is None:
        diagnostics.append(f"unknown legacy status: {raw_status}")

    canonical_id = override.get("id")
    if canonical_id is None and discovered_id is not None:
        canonical_id = f"{registry.get('namespace', 'uc-rust')}:{discovered_id}"

    disposition = override.get("disposition")
    if disposition is None:
        if diagnostics:
            disposition = "manual-review"
        elif status in {"Accepted", "Deprecated", "Superseded", "Rejected", "Withdrawn"}:
            disposition = "grandfathered"
        else:
            disposition = "backfill-in-place"

    aliases = tuple(str(value) for value in override.get("aliases", []))
    return InventoryItem(
        path=relative,
        discovered_id=discovered_id,
        canonical_id=canonical_id,
        record_type=override.get("type", record_type),
        title=title,
        status=status,
        disposition=disposition,
        aliases=aliases,
        diagnostics=tuple(diagnostics),
    )


def discover(registry: dict[str, Any]) -> list[InventoryItem]:
    items: list[InventoryItem] = []
    for root_name in registry.get("legacy_roots", []):
        root = ROOT / root_name
        if not root.is_dir():
            raise SystemExit(f"missing legacy root: {root_name}")
        for path in sorted(root.glob("*.md")):
            items.append(classify(path, registry))
    return items


def add_duplicate_diagnostics(items: list[InventoryItem]) -> list[InventoryItem]:
    by_id: dict[str, list[InventoryItem]] = {}
    for item in items:
        if item.canonical_id:
            by_id.setdefault(item.canonical_id, []).append(item)

    result: list[InventoryItem] = []
    for item in items:
        diagnostics = list(item.diagnostics)
        if item.canonical_id and len(by_id[item.canonical_id]) > 1:
            diagnostics.append(f"duplicate canonical identifier: {item.canonical_id}")
        result.append(
            InventoryItem(
                path=item.path,
                discovered_id=item.discovered_id,
                canonical_id=item.canonical_id,
                record_type=item.record_type,
                title=item.title,
                status=item.status,
                disposition=item.disposition,
                aliases=item.aliases,
                diagnostics=tuple(diagnostics),
            )
        )
    return result


def validate(items: list[InventoryItem]) -> list[str]:
    errors: list[str] = []
    for item in items:
        for diagnostic in item.diagnostics:
            errors.append(f"{item.path}: {diagnostic}")
        if item.disposition == "manual-review":
            errors.append(f"{item.path}: unresolved migration disposition")
    return errors


def render_markdown(items: list[InventoryItem]) -> str:
    lines = [
        "# Legacy ADR/RFC Migration Inventory",
        "",
        "| Path | Canonical ID | Status | Disposition | Diagnostics |",
        "|---|---|---|---|---|",
    ]
    for item in items:
        diagnostics = "; ".join(item.diagnostics) or "—"
        lines.append(
            f"| `{item.path}` | `{item.canonical_id or '—'}` | "
            f"{item.status or '—'} | `{item.disposition}` | {diagnostics} |"
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=("json", "markdown"), default="markdown")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    registry = load_registry()
    items = add_duplicate_diagnostics(discover(registry))
    errors = validate(items)

    if args.format == "json":
        output = json.dumps([asdict(item) for item in items], indent=2) + "\n"
    else:
        output = render_markdown(items)

    if args.output:
        target = args.output if args.output.is_absolute() else ROOT / args.output
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(output, encoding="utf-8")
    else:
        print(output, end="")

    if args.check and errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
