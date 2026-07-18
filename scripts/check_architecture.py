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


def dependency_section(manifest: str) -> str:
    marker = "[dependencies]"
    if marker not in manifest:
        return ""
    remainder = manifest.split(marker, 1)[1]
    next_section = remainder.find("\n[")
    return remainder if next_section < 0 else remainder[:next_section]


def main() -> None:
    domain = dependency_section(read("crates/uc-domain/Cargo.toml"))
    if domain.strip():
        fail("uc-domain must not declare runtime dependencies")

    application = dependency_section(read("crates/uc-application/Cargo.toml"))
    allowed_application = {"uc-domain"}
    declared_application = {
        line.split("=", 1)[0].strip()
        for line in application.splitlines()
        if "=" in line and not line.lstrip().startswith("#")
    }
    prohibited = declared_application - allowed_application
    if prohibited:
        fail(
            "uc-application contains prohibited dependencies: "
            + ", ".join(sorted(prohibited))
        )
    if "uc-domain" not in declared_application:
        fail("uc-application must depend on uc-domain")

    server = dependency_section(read("apps/uc-server/Cargo.toml"))
    if "uc-application" not in server:
        fail("uc-server must compose the application crate")

    for provider_token in (
        "sqlx",
        "diesel",
        "redis",
        "aws-sdk",
        "spicedb",
        "axum",
        "actix-web",
        "tonic",
        "reqwest",
    ):
        if provider_token in domain or provider_token in application:
            fail(
                f"provider/framework dependency {provider_token!r} leaked into the core"
            )

    print("architecture dependency boundaries are valid")


if __name__ == "__main__":
    main()
