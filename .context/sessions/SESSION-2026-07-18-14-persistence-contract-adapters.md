# Session 2026-07-18-14 — Persistence contract and adapters

## Governing issue

- `nomed/uc-rust#42`

## Objective

Introduce one reusable Basket persistence contract and make SQLite and PostgreSQL adapters implement the same application port without enabling automatic GitHub Actions consumption.

## Changes

- extended `BasketRepository` with provider-neutral `save` and `load`
- exposed read-only aggregate state required by adapters
- added `uc-persistence-contract`
- added SQLite adapter with in-memory contract test
- added PostgreSQL adapter with opt-in ignored contract test
- added `just test-postgres`, requiring `UC_TEST_POSTGRES_URL`
- updated architecture validation for explicit adapter boundaries

## Execution policy

- SQLite contract runs in the ordinary local workspace suite
- PostgreSQL contract is intentionally opt-in until ephemeral provisioning is introduced
- no GitHub Action was triggered

## Verification status

Repository changes are implemented but have not yet been compiled or executed by a clean runner in this session. They must remain `Implemented`, not `Evidenced`, until `just check-quick` and the PostgreSQL contract complete successfully.

## Next work

- compile and repair the workspace using a deliberate local or manual quick run
- introduce ephemeral PostgreSQL provisioning without automatic per-commit execution
- add migration files rather than embedded schema setup
- build the first central/store-edge topology and WAN partition harness
