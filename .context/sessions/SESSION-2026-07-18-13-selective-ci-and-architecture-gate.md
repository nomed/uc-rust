# Session 2026-07-18-13 — Selective CI and architecture gate

## Objective

Stop wasteful, low-signal GitHub Actions execution and continue Project Ready enforcement locally and deterministically.

## Completed

- Replaced automatic PR/main triggers in `.github/workflows/quality.yml` with intentional `workflow_dispatch` execution.
- Added `quick` and `full` suites.
- Made coverage tooling conditional on the full suite.
- Added failure log artifacts and GitHub step summary output.
- Accepted ADR-0017 for selective CI execution.
- Added `scripts/check_architecture.py`.
- Added the architecture dependency check to `just validate` and therefore to `just check`.

## Policy

- Normal commits do not start Actions.
- Agents run versioned local commands first.
- Quick Actions runs are requested only when runner-specific validation is useful.
- Full runs are reserved for formal evidence, readiness, release, enforcement changes or investigation.
- Automatic PR checks may return only after the quick suite is stable, fast and diagnostic.

## Evidence status

- Selective CI policy: Implemented.
- Architecture dependency gate: Implemented.
- Actions diagnostic behavior: Implemented but not yet Evidenced by a manual run.
- Coverage and other quality gates remain unaccepted until clean and deliberate-failure evidence is recorded.

## Remaining #42 work

- Execute one manual quick run and inspect its diagnostic output.
- Correct any actual failures before executing a full run.
- Add PostgreSQL/SQLite contract harness.
- Add central/store-edge ephemeral topology and WAN partition tests.
- Add duplicate/reorder synchronization proof.
