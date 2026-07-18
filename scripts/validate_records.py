#!/usr/bin/env python3
"""Validate UC Rust 1.0 Markdown architecture records.

The validator is deliberately independent from Yukh and external services. It parses
YAML front matter, validates the common envelope, applies lifecycle/relation rules,
and checks minimal type-specific body sections.
"""
from __future__ import annotations

import argparse
import datetime as dt
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

try:
    import yaml
except ImportError as exc:  # pragma: no cover
    raise SystemExit("PyYAML is required: python -m pip install -r requirements-dev.txt") from exc

ROOT = Path(__file__).resolve().parents[1]
RECORD_TYPES = {"ADR", "RFC", "CR", "RRR", "QAR", "ER", "SR", "IR", "DR", "RR"}
STATES = {"Draft", "Proposed", "Accepted", "Deprecated", "Superseded", "Rejected", "Withdrawn"}
RELATIONS = {
    "realizes_concept", "depends_on", "implements", "constrains", "governs",
    "exposes", "consumes", "produces", "verifies", "included_in",
    "supersedes", "deprecates",
}
ID_RE = re.compile(r"^[a-z][a-z0-9-]*:(ADR|RFC|CR|RRR|QAR|ER|SR|IR|DR|RR)-[0-9]{4,}$")
TARGET_RE = re.compile(r"^[a-z][a-z0-9-]*:[A-Za-z0-9][A-Za-z0-9._-]*$")
SEMVER_RE = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+$")
REQUIRED = {
    "id", "type", "schema_version", "content_version", "title", "summary",
    "status", "owners", "created_at", "updated_at", "scope", "provenance",
    "relations", "review",
}
BODY_SECTIONS = {
    "ADR": {"Context", "Decision", "Consequences"},
    "RFC": {"Context", "Proposal"},
    "CR": {"Purpose", "Invariants", "Authority"},
    "RRR": {"Responsibility", "Boundary", "Invariants"},
    "QAR": {"Quality scenario", "Measure"},
    "ER": {"Economic scope", "Economic constraints"},
    "SR": {"Security scope", "Threats", "Controls"},
    "IR": {"Interface", "Contract"},
    "DR": {"Data scope", "Invariants"},
    "RR": {"Release scope", "Entry criteria", "Exit criteria"},
}

@dataclass(frozen=True)
class Diagnostic:
    path: Path
    field: str
    rule: str
    message: str

    def render(self) -> str:
        return f"{self.path}:{self.field}: [{self.rule}] {self.message}"


def parse_record(path: Path) -> tuple[dict[str, Any], str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise ValueError("record must start with YAML front matter delimiter '---'")
    marker = text.find("\n---\n", 4)
    if marker < 0:
        raise ValueError("record YAML front matter has no closing delimiter")
    raw = text[4:marker]
    data = yaml.safe_load(raw)
    if not isinstance(data, dict):
        raise ValueError("front matter must decode to a mapping")
    return data, text[marker + 5:]


def diag(path: Path, field: str, rule: str, message: str) -> Diagnostic:
    return Diagnostic(path, field, rule, message)


def target_kind(target: str) -> str:
    local = target.split(":", 1)[1] if ":" in target else target
    return local.split("-", 1)[0]


def validate(path: Path) -> list[Diagnostic]:
    out: list[Diagnostic] = []
    try:
        data, body = parse_record(path)
    except (OSError, ValueError, yaml.YAMLError) as exc:
        return [diag(path, "front_matter", "parse", str(exc))]

    missing = sorted(REQUIRED - data.keys())
    for field in missing:
        out.append(diag(path, field, "required", "required field is missing"))

    unknown = sorted(set(data) - (REQUIRED | {"non_goals", "evidence", "deprecation", "superseded_by", "waivers"}))
    for field in unknown:
        out.append(diag(path, field, "unknown-field", "field is not part of schema version 1"))

    rid = data.get("id")
    rtype = data.get("type")
    if not isinstance(rid, str) or not ID_RE.fullmatch(rid):
        out.append(diag(path, "id", "id-format", "expected <namespace>:<TYPE>-<digits>"))
    elif rtype and target_kind(rid) != rtype:
        out.append(diag(path, "id", "id-type", f"identifier type does not match type {rtype}"))
    if rtype not in RECORD_TYPES:
        out.append(diag(path, "type", "record-type", f"unsupported record family: {rtype!r}"))
    if data.get("schema_version") != 1:
        out.append(diag(path, "schema_version", "schema-version", "only schema version 1 is supported"))
    if not isinstance(data.get("content_version"), str) or not SEMVER_RE.fullmatch(data["content_version"]):
        out.append(diag(path, "content_version", "content-version", "must be semantic version x.y.z"))
    if data.get("status") not in STATES:
        out.append(diag(path, "status", "lifecycle-state", "unknown normative lifecycle state"))

    for date_field in ("created_at", "updated_at"):
        value = data.get(date_field)
        if isinstance(value, dt.date):
            continue
        try:
            dt.date.fromisoformat(str(value))
        except ValueError:
            out.append(diag(path, date_field, "date", "must be an ISO-8601 date"))

    owners = data.get("owners")
    if not isinstance(owners, list) or not owners:
        out.append(diag(path, "owners", "owners", "at least one accountable owner is required"))
    elif any(not isinstance(x, dict) or not x.get("role") for x in owners):
        out.append(diag(path, "owners", "owners", "each owner must declare a role"))

    review = data.get("review")
    if not isinstance(review, dict):
        out.append(diag(path, "review", "review", "review must be a mapping"))
    else:
        for field in ("required_roles", "reviewers", "disposition"):
            if field not in review:
                out.append(diag(path, f"review.{field}", "required", "required review field is missing"))
        disposition = review.get("disposition")
        status = data.get("status")
        if status == "Accepted" and disposition != "accepted":
            out.append(diag(path, "review.disposition", "acceptance", "Accepted requires disposition=accepted"))
        if status == "Rejected" and disposition != "rejected":
            out.append(diag(path, "review.disposition", "rejection", "Rejected requires disposition=rejected"))
        if disposition in {"accepted", "rejected", "withdrawn"} and not review.get("reviewed_at"):
            out.append(diag(path, "review.reviewed_at", "review-date", "final disposition requires reviewed_at"))

    status = data.get("status")
    if status == "Superseded" and not data.get("superseded_by"):
        out.append(diag(path, "superseded_by", "supersession", "Superseded requires successor identifiers"))
    if status == "Deprecated" and not data.get("deprecation"):
        out.append(diag(path, "deprecation", "deprecation", "Deprecated requires deprecation metadata"))

    relations = data.get("relations")
    if not isinstance(relations, list):
        out.append(diag(path, "relations", "relations", "relations must be a list"))
    else:
        seen: set[tuple[str, str, str]] = set()
        for index, relation in enumerate(relations):
            prefix = f"relations[{index}]"
            if not isinstance(relation, dict):
                out.append(diag(path, prefix, "relation-shape", "relation must be a mapping")); continue
            kind, target = relation.get("type"), relation.get("target")
            if kind not in RELATIONS:
                out.append(diag(path, f"{prefix}.type", "relation-type", f"unsupported relation: {kind!r}"))
            if not isinstance(target, str) or not TARGET_RE.fullmatch(target):
                out.append(diag(path, f"{prefix}.target", "relation-target", "target must be a canonical identifier")); continue
            if target == rid:
                out.append(diag(path, f"{prefix}.target", "self-relation", "self-relations are forbidden"))
            key = (str(kind), target, str(relation.get("scope", "")))
            if key in seen:
                out.append(diag(path, prefix, "duplicate-relation", "duplicate type/target/scope relation"))
            seen.add(key)
            if relation.get("inferred") is True:
                out.append(diag(path, f"{prefix}.inferred", "normative-authority", "canonical records cannot contain inferred normative edges"))
            tk = target_kind(target)
            if kind == "realizes_concept" and target.startswith("uc-rust:"):
                out.append(diag(path, prefix, "relation-target-kind", "realizes_concept must target an external concept authority"))
            if kind == "included_in" and tk != "RR":
                out.append(diag(path, prefix, "relation-target-kind", "included_in must target a Release Record"))
            if kind in {"supersedes", "deprecates"} and rtype != tk:
                out.append(diag(path, prefix, "relation-family", f"{kind} must target the same record family"))

    headings = set(re.findall(r"^##\s+(.+?)\s*$", body, flags=re.MULTILINE))
    for heading in sorted(BODY_SECTIONS.get(str(rtype), set()) - headings):
        out.append(diag(path, "body", "required-section", f"{rtype} requires section '## {heading}'"))
    return out


def record_paths(inputs: Iterable[str]) -> list[Path]:
    paths: list[Path] = []
    for raw in inputs:
        path = (ROOT / raw).resolve() if not Path(raw).is_absolute() else Path(raw)
        if path.is_dir():
            paths.extend(sorted(path.rglob("*.md")))
        elif path.is_file():
            paths.append(path)
        else:
            raise SystemExit(f"input does not exist: {raw}")
    return paths


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="*", default=["docs/knowledge/records"])
    args = parser.parse_args()
    files = record_paths(args.paths)
    if not files:
        print("no records found", file=sys.stderr); return 2
    diagnostics = [item for path in files for item in validate(path)]
    for item in diagnostics:
        print(item.render(), file=sys.stderr)
    if diagnostics:
        print(f"validation failed: {len(diagnostics)} diagnostic(s) across {len(files)} file(s)", file=sys.stderr)
        return 1
    print(f"validated {len(files)} architecture record(s)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
