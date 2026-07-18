# Session 2026-07-18-15 — Quick verification and sync harness

## Governing issue

- `nomed/uc-rust#42`

## Objective

Run one deliberately scoped quick verification without re-enabling automatic Actions, and implement the first deterministic central/edge synchronization contract.

## Execution strategy

- attempted local clone in the available execution environment
- local execution was blocked because that environment could not resolve GitHub
- created branch `verification/quick-once`
- added a branch-only one-shot quick workflow
- automatic workflows on `main` remain disabled
- the available connector does not expose push-triggered workflow runs, so no success claim is made

## Implemented synchronization contract

- `uc-sync` crate with no provider dependencies
- stable edge and event identities
- durable in-memory outbox contract
- central deduplication by event identity
- per-edge monotonic sequence handling
- buffering and draining of out-of-order events
- explicit acknowledgement removal
- tests for WAN backlog preservation, duplicate delivery, reorder and acknowledgement

## Quality model impact

The following rows now have repository controls and are marked `Implemented`:

- context freshness
- architecture integrity
- replaceable infrastructure
- edge synchronization and convergence
- correctness
- documentation
- developer experience

The computed maturity baseline is 25.8%. No row is marked `Evidenced` because a clean run has not yet been observed and linked.

## Next work

- inspect the one-shot quick run from GitHub UI or another Actions-capable interface
- fix any compile/lint/test failures and remove the temporary verification branch/workflow
- add explicit sequence-conflict handling for different event IDs claiming the same edge sequence
- persist edge outbox and central inbox state through SQLite
- build a central/store-edge process harness with controllable connectivity
