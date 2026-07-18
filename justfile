set shell := ["bash", "-euo", "pipefail", "-c"]

default: check-quick

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

test-postgres:
    test -n "${UC_TEST_POSTGRES_URL:-}" || { echo "UC_TEST_POSTGRES_URL is required" >&2; exit 1; }
    cargo test -p uc-persistence-postgres satisfies_basket_repository_contract -- --ignored --exact

docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

validate:
    python3 scripts/validate_repository.py
    python3 scripts/check_architecture.py

coverage:
    mkdir -p target/coverage
    cargo llvm-cov nextest --workspace --all-features --branch --lcov --output-path target/coverage/lcov.info
    python3 scripts/check_coverage.py target/coverage/lcov.info

check-quick: fmt lint test docs validate

check-full: check-quick coverage

check: check-quick
