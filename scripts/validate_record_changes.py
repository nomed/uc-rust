#!/usr/bin/env python3
"""Validate immutable metadata and lifecycle transitions across a Git diff."""
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

try:
    import yaml
except ImportError as exc:  # pragma: no cover
    raise SystemExit("PyYAML is required: python -m pip install -r requirements-dev.txt") from exc

ROOT = Path(__file__).resolve().parents[1]
ALLOWED_TRANSITIONS = {
    "Draft": {"Proposed", "Withdrawn"},
    "Proposed": {"Draft", "Accepted", "Rejected", "Withdrawn"},
    "Accepted": {"Proposed", "Deprecated", "Superseded"},
    "Deprecated": {"Proposed", "Superseded"},
    "Superseded": set(),
    "Rejected": set(),
    "Withdrawn": set(),
}
SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")
LIFECYCLE_ONLY_FIELDS = {
    "status", "updated_at", "review", "deprecation", "superseded_by", "waivers"
}


@dataclass(frozen=True)
class Finding:
    path: str
    field: str
    rule: str
    message: str

    def render(self) -> str:
        return f"{self.path}:{self.field}: [{self.rule}] {self.message}"


def git(*args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", *args], cwd=ROOT, text=True, capture_output=True, check=check
    )


def parse_front_matter(text: str) -> dict[str, Any]:
    if not text.startswith("---\n"):
        raise ValueError("record must start with YAML front matter")
    marker = text.find("\n---\n", 4)
    if marker < 0:
        raise ValueError("record YAML front matter has no closing delimiter")
    data = yaml.safe_load(text[4:marker])
    if not isinstance(data, dict):
        raise ValueError("front matter must decode to a mapping")
    return data


def read_base(base_ref: str, path: str) -> str | None:
    result = git("show", f"{base_ref}:{path}", check=False)
    return result.stdout if result.returncode == 0 else None


def semver(value: Any) -> tuple[int, int, int] | None:
    if not isinstance(value, str):
        return None
    match = SEMVER_RE.fullmatch(value)
    return tuple(map(int, match.groups())) if match else None


def selected_paths(base_ref: str, inputs: Iterable[str]) -> list[str]:
    roots = [str(Path(raw).as_posix()).rstrip("/") for raw in inputs]
    result = git("diff", "--name-only", "--diff-filter=ACMRT", f"{base_ref}...HEAD")
    paths = []
    for line in result.stdout.splitlines():
        path = line.strip()
        if not path.endswith(".md"):
            continue
        if any(path == root or path.startswith(root + "/") for root in roots):
            paths.append(path)
    return sorted(set(paths))


def validate_change(path: str, before_text: str | None, after_text: str) -> list[Finding]:
    findings: list[Finding] = []
    try:
        after = parse_front_matter(after_text)
    except (ValueError, yaml.YAMLError) as exc:
        return [Finding(path, "front_matter", "parse", str(exc))]

    if before_text is None:
        if after.get("status") not in {"Draft", "Proposed"}:
            findings.append(Finding(path, "status", "initial-state", "new records must start as Draft or Proposed"))
        return findings

    try:
        before = parse_front_matter(before_text)
    except (ValueError, yaml.YAMLError) as exc:
        return [Finding(path, "front_matter", "base-parse", str(exc))]

    for field in ("id", "type", "created_at"):
        if before.get(field) != after.get(field):
            findings.append(Finding(path, field, "immutable-field", f"{field} cannot change after record creation"))

    old_status, new_status = before.get("status"), after.get("status")
    if old_status != new_status:
        allowed = ALLOWED_TRANSITIONS.get(str(old_status), set())
        if new_status not in allowed:
            findings.append(Finding(path, "status", "invalid-transition", f"transition {old_status!r} -> {new_status!r} is not allowed"))

    old_version, new_version = semver(before.get("content_version")), semver(after.get("content_version"))
    if old_version and new_version and new_version < old_version:
        findings.append(Finding(path, "content_version", "version-regression", "content_version cannot decrease"))

    semantic_before = {k: v for k, v in before.items() if k not in LIFECYCLE_ONLY_FIELDS}
    semantic_after = {k: v for k, v in after.items() if k not in LIFECYCLE_ONLY_FIELDS}
    if semantic_before != semantic_after and old_version == new_version:
        findings.append(Finding(path, "content_version", "semantic-version", "semantic envelope change requires a content_version increment"))

    if new_status == "Accepted":
        review = after.get("review")
        if not isinstance(review, dict) or review.get("disposition") != "accepted" or not review.get("reviewed_at"):
            findings.append(Finding(path, "review", "acceptance-evidence", "Accepted requires disposition=accepted and reviewed_at"))

    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="*", default=["docs/knowledge/records"])
    parser.add_argument("--base-ref", required=True)
    args = parser.parse_args()

    findings: list[Finding] = []
    for path in selected_paths(args.base_ref, args.paths):
        current_path = ROOT / path
        if not current_path.is_file():
            continue
        findings.extend(validate_change(path, read_base(args.base_ref, path), current_path.read_text(encoding="utf-8")))

    deleted = git("diff", "--name-only", "--diff-filter=D", f"{args.base_ref}...HEAD").stdout.splitlines()
    for path in deleted:
        if path.endswith(".md") and any(path.startswith(str(Path(raw).as_posix()).rstrip("/") + "/") for raw in args.paths):
            before_text = read_base(args.base_ref, path)
            if before_text:
                try:
                    before = parse_front_matter(before_text)
                    if before.get("status") == "Accepted":
                        findings.append(Finding(path, "status", "accepted-deletion", "Accepted records cannot be deleted; deprecate or supersede them"))
                except (ValueError, yaml.YAMLError):
                    pass

    for finding in findings:
        print(finding.render(), file=sys.stderr)
    if findings:
        print(f"transition validation failed: {len(findings)} finding(s)", file=sys.stderr)
        return 1
    print("architecture record transitions are valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
