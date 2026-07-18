# Runtime Lifecycle, Configuration and Composition Test Plan

- Governing issue: #48
- Governing decision: ADR-0025
- Target gate: M1 / #54

## Scope

Prove deterministic startup, readiness, degradation, configuration reload, quiesce, drain, shutdown and composition across central and store-edge profiles.

## Required tests

### Lifecycle

- valid and invalid state transitions;
- idempotent start, quiesce and stop requests;
- mandatory dependency failure prevents readiness;
- optional dependency failure creates only the declared degraded impact;
- terminal lifecycle history is immutable.

### Startup and rollback

- dependency-order startup;
- dependency-cycle rejection before startup;
- reverse-order cleanup after partial startup failure;
- readiness only after compatibility and recovery checks;
- redacted startup-failure evidence.

### Configuration

- schema, signature and compatibility validation;
- rejection of unknown normative fields;
- immutable identity protection;
- dynamic-safe activation for future invocations only;
- drain-then-reload sequencing;
- restart-required staging without partial activation;
- rejected candidates leave the active revision unchanged;
- bounded emergency overrides require owner, reason and expiry.

### Composition

- deterministic central and store-edge composition manifests;
- typed port injection without arbitrary dependency lookup;
- no provider selection in delivery adapters;
- profile-specific providers without duplicated canonical Operations;
- incompatible Operation/realization revisions block affected readiness.

### Edge and offline behavior

- store-edge keeps its approved local Operation subset available during WAN loss;
- WAN-dependent Operations become explicitly blocked or degraded;
- stale local manifests produce the declared capability impact;
- local durable-store failure blocks dependent Operations;
- reconnect does not change the binding of in-flight invocations.

### Quiesce, drain and shutdown

- admission closes before dependency teardown;
- mutating Operations complete, defer durably or become explicit indeterminate outcomes;
- workers checkpoint or relinquish leases;
- effects, outbox records and lifecycle evidence flush before resource closure;
- reverse-order shutdown;
- shutdown deadline exhaustion creates evidence for next-start recovery.

## Architecture checks

CI must reject:

- domain/application reads of environment or runtime configuration;
- mutable globals or unrestricted service locators;
- components started outside the lifecycle supervisor;
- unregistered tasks surviving shutdown;
- duplicate central/edge implementations of a canonical Operation;
- delivery-adapter ownership of provider selection or dependency construction.

## Performance and economic evidence

Measure cold/warm startup, readiness time, steady-state memory, idle CPU, per-invocation configuration overhead, component and connection counts, drain duration, restart blast radius and cost per store/day and tenant/month.

## Exit criteria

All mandatory scenarios are automated and reproducible, architecture checks run in CI, edge offline behavior is demonstrated, and scorecards are linked from gate #54.