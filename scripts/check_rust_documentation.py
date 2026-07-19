#!/usr/bin/env python3
"""Enforce crate/module documentation and documented public Rust APIs.

This repository treats documentation as executable architecture evidence. Every
checked Rust source file must begin with an inner module-level doc comment
(`//!`). Public APIs remain protected separately by the workspace
`missing_docs = "deny"` lint.

The check intentionally excludes generated code and build output. A file may
only be excluded by adding its path to ``EXCLUDED_FILES`` together with a
reviewed rationale in this module.
"""

from __future__ import annotations

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
CHECKED_ROOTS = (ROOT / "crates", ROOT / "apps")
EXCLUDED_FILES: frozenset[Path] = frozenset()


def rust_sources() -> list[Path]:
    """Return all governed Rust source files in deterministic order."""
    files: list[Path] = []
    for source_root in CHECKED_ROOTS:
        files.extend(
            path
            for path in source_root.rglob("*.rs")
            if "target" not in path.parts and path.relative_to(ROOT) not in EXCLUDED_FILES
        )
    return sorted(files)


def has_module_documentation(path: Path) -> bool:
    """Return whether the first meaningful source line is a module doc comment."""
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#!["):
            continue
        return line.startswith("//!")
    return False


def main() -> int:
    """Validate all governed Rust files and return a process exit status."""
    undocumented = [path.relative_to(ROOT) for path in rust_sources() if not has_module_documentation(path)]
    if undocumented:
        print("Rust files missing a leading //! module-level documentation block:")
        for path in undocumented:
            print(f"- {path}")
        return 1
    print(f"Validated module documentation for {len(rust_sources())} Rust files.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
