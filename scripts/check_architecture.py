#!/usr/bin/env python3
"""Enforce the initial UC Rust crate dependency boundaries."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def read(path: str) -> str:
    target = ROOT / path
    if not target.is_file():
        fail(f"required manifest is missing: {path}")
    return target.read_text(encoding="utf-8")


def dependency_names(path: str) -> set[str]:
    manifest = read(path)
    marker = "[dependencies]"
    if marker not in manifest:
        return set()
    remainder = manifest.split(marker, 1)[1]
    next_section = remainder.find("\n[")
    section = remainder if next_section < 0 else remainder[:next_section]
    return {
        line.split("=", 1)[0].strip()
        for line in section.splitlines()
        if "=" in line and not line.lstrip().startswith("#")
    }


def require_exact(path: str, expected: set[str]) -> None:
    declared = dependency_names(path)
    if declared != expected:
        fail(
            f"{path} dependencies differ: expected {sorted(expected)}, "
            f"found {sorted(declared)}"
        )


def main() -> None:
    require_exact("crates/uc-domain/Cargo.toml", set())
    require_exact("crates/uc-application/Cargo.toml", {"serde", "uc-domain"})
    require_exact(
        "crates/uc-persistence-contract/Cargo.toml",
        {"uc-application", "uc-domain"},
    )
    require_exact(
        "crates/uc-persistence-sqlite/Cargo.toml",
        {"rusqlite", "uc-application", "uc-domain"},
    )
    require_exact(
        "crates/uc-persistence-postgres/Cargo.toml",
        {"postgres", "uc-application", "uc-domain"},
    )
    require_exact("crates/uc-sync/Cargo.toml", set())

    server = dependency_names("apps/uc-server/Cargo.toml")
    if "uc-application" not in server:
        fail("uc-server must compose the application crate")

    core_text = read("crates/uc-domain/Cargo.toml") + read(
        "crates/uc-application/Cargo.toml"
    )
    for provider_token in (
        "sqlx",
        "diesel",
        "rusqlite",
        "postgres",
        "redis",
        "aws-sdk",
        "spicedb",
        "axum",
        "actix-web",
        "tonic",
        "reqwest",
    ):
        if provider_token in core_text:
            fail(f"provider/framework dependency {provider_token!r} leaked into the core")

    print("architecture dependency boundaries are valid")


if __name__ == "__main__":
    main()
