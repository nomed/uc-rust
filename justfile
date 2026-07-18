set shell := ["bash", "-euo", "pipefail", "-c"]

default: check

bootstrap:
    rustup show
    cargo install cargo-nextest --locked
    cargo install cargo-llvm-cov --locked

fmt:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo nextest run --workspace --all-features

docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

validate:
    python3 scripts/validate_repository.py

coverage:
    cargo llvm-cov nextest --workspace --all-features --branch --fail-under-lines 100 --fail-under-branches 100

check: fmt lint test docs validate coverage
