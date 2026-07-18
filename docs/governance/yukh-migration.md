# Yukh governance migration

UC Rust is the first proving ground for `nomed/yukh`.

## Current phase

The migration is in **shadow mode**:

- UC Rust embedded scripts remain authoritative for write operations.
- Yukh reads equivalent fixtures and produces comparison reports.
- Yukh must not mutate UC Rust during shadow mode.
- Generic governance capabilities are implemented in Yukh, not duplicated further in UC Rust.

## Work streams

### Stream A — Governance migration

Tracked by UC Rust issue #20 and the Yukh UC Rust compatibility epic.

Deliverables:

- capture the UC Rust compatibility contract;
- deterministic read-only drift reporting;
- repository and Project v2 adapters;
- parent/sub-issue and dependency adapters;
- sandbox reconciliation and idempotency evidence;
- zero unexplained shadow drift;
- pinned Yukh integration and rollback evidence.

### Stream B — Product implementation

UC Rust continues implementation independently, starting with issue #44 for reproducible Economics by Design reports.

## Exit criteria

Yukh becomes the apply engine only after:

- zero unexplained drift;
- idempotent reconciliation;
- explicit diagnostics for partial or failed projections;
- sandbox evidence;
- rollback evidence;
- acceptance in both repositories.

The embedded scripts are removed only after the pinned Yukh integration has been proven and rollback remains possible.
