#!/usr/bin/env python3
"""Validate repository-level integrity of UC Rust architecture-record relations."""
from __future__ import annotations

import argparse
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

try:
    import yaml
except ImportError as exc:  # pragma: no cover
    raise SystemExit("PyYAML is required: python -m pip install -r requirements-dev.txt") from exc

ROOT = Path(__file__).resolve().parents[1]
INVERSE_RELATIONS = {
    "implemented_by", "required_by", "constrained_by", "satisfied_by",
    "realized_by", "includes", "used_by", "governs",
}
ISOLATION_ERROR_TYPES = {"CR", "RRR", "IR", "DR", "QAR", "SR", "ER"}


@dataclass(frozen=True)
class Finding:
    path: Path
    field: str
    rule: str
    message: str
    severity: str = "error"

    def render(self) -> str:
        return f"{self.path}:{self.field}: [{self.severity}:{self.rule}] {self.message}"


def parse_front_matter(path: Path) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise ValueError("record must start with YAML front matter")
    marker = text.find("\n---\n", 4)
    if marker < 0:
        raise ValueError("record YAML front matter has no closing delimiter")
    data = yaml.safe_load(text[4:marker])
    if not isinstance(data, dict):
        raise ValueError("front matter must decode to a mapping")
    return data


def paths_from(inputs: Iterable[str]) -> list[Path]:
    result: list[Path] = []
    for raw in inputs:
        path = (ROOT / raw).resolve() if not Path(raw).is_absolute() else Path(raw)
        if path.is_dir():
            result.extend(sorted(path.rglob("*.md")))
        elif path.is_file():
            result.append(path)
        else:
            raise SystemExit(f"input does not exist: {raw}")
    return result


def load_authorities(path: Path) -> set[str]:
    if not path.is_file():
        raise SystemExit(f"namespace authority registry is missing: {path}")
    data = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise SystemExit("namespace authority registry must be a mapping")
    authorities = data.get("authorities", data)
    if isinstance(authorities, dict):
        return set(authorities)
    if isinstance(authorities, list):
        return {str(item.get("namespace")) for item in authorities if isinstance(item, dict) and item.get("namespace")}
    raise SystemExit("namespace authority registry has unsupported shape")


def find_cycle(graph: dict[str, set[str]]) -> list[str] | None:
    visiting: set[str] = set()
    visited: set[str] = set()
    stack: list[str] = []

    def visit(node: str) -> list[str] | None:
        if node in visiting:
            start = stack.index(node)
            return stack[start:] + [node]
        if node in visited:
            return None
        visiting.add(node)
        stack.append(node)
        for target in sorted(graph.get(node, set())):
            cycle = visit(target)
            if cycle:
                return cycle
        stack.pop()
        visiting.remove(node)
        visited.add(node)
        return None

    for node in sorted(graph):
        cycle = visit(node)
        if cycle:
            return cycle
    return None


def validate(files: list[Path], authority_registry: Path) -> list[Finding]:
    findings: list[Finding] = []
    records: dict[str, tuple[Path, dict[str, Any]]] = {}
    authorities = load_authorities(authority_registry)

    for path in files:
        try:
            data = parse_front_matter(path)
        except (OSError, ValueError, yaml.YAMLError) as exc:
            findings.append(Finding(path, "front_matter", "parse", str(exc)))
            continue
        rid = data.get("id")
        if not isinstance(rid, str):
            findings.append(Finding(path, "id", "graph-id", "record has no canonical string identifier"))
            continue
        if rid in records:
            findings.append(Finding(path, "id", "duplicate-id", f"identifier already declared by {records[rid][0]}"))
        else:
            records[rid] = (path, data)

    outgoing_count: defaultdict[str, int] = defaultdict(int)
    incoming_count: defaultdict[str, int] = defaultdict(int)
    supersedes_graph: defaultdict[str, set[str]] = defaultdict(set)
    depends_graph: defaultdict[str, set[str]] = defaultdict(set)

    for rid, (path, data) in records.items():
        relations = data.get("relations", [])
        if not isinstance(relations, list):
            continue
        seen: set[tuple[str, str, str]] = set()
        for index, relation in enumerate(relations):
            field = f"relations[{index}]"
            if not isinstance(relation, dict):
                continue
            kind = relation.get("type")
            target = relation.get("target")
            scope = str(relation.get("scope", ""))
            if kind in INVERSE_RELATIONS:
                findings.append(Finding(path, f"{field}.type", "inverse-relation", f"{kind!r} is projection-only"))
            if not isinstance(target, str) or ":" not in target:
                continue
            key = (str(kind), target, scope)
            if key in seen:
                findings.append(Finding(path, field, "duplicate-edge", "duplicate source/type/target/scope edge"))
            seen.add(key)
            if target == rid:
                findings.append(Finding(path, f"{field}.target", "self-edge", "self-relations are forbidden"))
            outgoing_count[rid] += 1
            incoming_count[target] += 1

            namespace = target.split(":", 1)[0]
            if namespace == "uc-rust" and target not in records:
                findings.append(Finding(path, f"{field}.target", "unresolved-local", f"local target {target} does not exist in scanned records"))
            elif namespace != "uc-rust" and namespace not in authorities:
                findings.append(Finding(path, f"{field}.target", "unknown-authority", f"namespace {namespace!r} is not declared"))

            if kind == "supersedes" and target in records:
                supersedes_graph[rid].add(target)
            if kind == "depends_on" and target in records:
                depends_graph[rid].add(target)

    for graph, rule in ((supersedes_graph, "supersedes-cycle"), (depends_graph, "dependency-cycle")):
        cycle = find_cycle(dict(graph))
        if cycle:
            first = cycle[0]
            path = records[first][0]
            findings.append(Finding(path, "relations", rule, " -> ".join(cycle)))

    for rid, (path, data) in records.items():
        rtype = str(data.get("type", ""))
        degree = outgoing_count[rid] + incoming_count[rid]
        if degree == 0:
            severity = "error" if rtype in ISOLATION_ERROR_TYPES else "warning"
            findings.append(Finding(path, "relations", "isolated-record", f"{rid} has no incoming or outgoing architecture relation", severity))
        owners = data.get("owners")
        if data.get("status") == "Accepted" and (not isinstance(owners, list) or not owners):
            findings.append(Finding(path, "owners", "accepted-orphan", "Accepted record has no accountable owner"))

    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="*", default=["docs/knowledge/records"])
    parser.add_argument("--authority-registry", default="governance/namespace-authorities.yaml")
    parser.add_argument("--warnings-as-errors", action="store_true")
    args = parser.parse_args()

    files = paths_from(args.paths)
    findings = validate(files, (ROOT / args.authority_registry).resolve())
    for finding in findings:
        print(finding.render(), file=sys.stderr)
    errors = [item for item in findings if item.severity == "error" or args.warnings_as_errors]
    if errors:
        print(f"graph validation failed: {len(errors)} blocking finding(s), {len(findings)} total", file=sys.stderr)
        return 1
    print(f"validated graph integrity for {len(files)} record(s) with {len(findings)} warning(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
