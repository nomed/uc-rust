# Session — Sync economic measurement harness

- Date: 2026-07-18
- Status: Implemented, not evidenced
- Governing decisions: ADR-0018, ADR-0019
- Related areas: Data Plane, Economics Plane, Cost efficiency, Edge synchronization

## Intent

Implement the first executable Economics by Design measurement slice around the durable SQLite synchronization adapter, without embedding monetary rates or provider-specific FinOps dependencies in the Data Plane.

## Changes

- Added workspace crate `crates/uc-sync-economics`.
- Added `SyncWorkload`, `SyncRunFacts`, `SyncEconomicReport` and `run_sync_workload`.
- The harness executes durable enqueue, pending read, central accept, duplicate detection and acknowledgement against a temporary file-backed SQLite database.
- Added canonical EbD observations for `uc.synced_business_event` and workload `sync-durable-event-v1`.
- Added workload governance under `governance/economics/workloads/sync-durable-event-v1.yaml`.
- Updated consumption-driver governance to distinguish elapsed wall-clock time from CPU time.

## Directly measured facts

- elapsed wall-clock nanoseconds;
- SQLite main-file growth;
- logical payload bytes submitted over the edge-to-central boundary;
- delivery attempts;
- duplicate deliveries.

## Attributed facts

- database transaction count is currently a partial adapter-contract attribution for unique central accepts.

It is not yet a complete count of enqueue, duplicate accept and acknowledgement transactions and therefore cannot enforce a release gate.

## Deliberate exclusions

- No monetary rate card.
- No full Cost-to-Serve claim.
- No cloud or FinOps provider SDK.
- No CPU-time claim based on wall-clock timing.
- No memory allocation measurement.
- No complete SQLite read/write/transaction counters.
- No physical disk write-amplification claim.

## Evidence status

The connector wrote repository files but did not execute the Rust toolchain. No claim is made that the workspace compiles, tests pass or the workload has produced a representative baseline.

Required next evidence:

1. compile and lint the workspace;
2. run deterministic tests;
3. execute repeated representative workloads;
4. capture environment fingerprint and machine-readable reports;
5. correct or replace the partial transaction attribution with database-level counters;
6. define noise tolerance before introducing regression gates.

## UC-BoK and EbD impact

- UC-BoK impact: implementation-level economic observability for distributed synchronization; no normative UC-BoK change asserted.
- EbD impact: implements existing Economic Unit, consumption-driver, costing-policy and economic-observability concepts from `nomed/ebd-bok`; any conceptual mismatch must become a reciprocal issue.
