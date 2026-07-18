# RFC-0001 — UC Runtime Foundation

- Status: Draft
- Date: 2026-07-18
- Governing epic: to be assigned
- Decision dependency: ADR-0021

## Objective

Define the smallest shared runtime required to execute canonical UC Rust Operations consistently across central, store-edge and future runtime profiles.

## Scope

The Runtime Foundation covers:

1. operation registration and invocation;
2. typed execution context and cancellation/deadline propagation;
3. lifecycle, startup, readiness, quiesce and graceful shutdown;
4. explicit dependency composition;
5. configuration loading, validation, revision and safe reload;
6. structured errors and failure correlation;
7. logging, tracing, metrics and health contribution contracts;
8. scheduled and background work execution through Operations;
9. runtime capability discovery;
10. extension registration boundaries;
11. operation-level Economics by Design correlation.

## Non-goals

The first runtime release does not provide:

- a general-purpose dependency injection container;
- a custom async executor;
- an internal distributed consensus implementation;
- arbitrary dynamic code loading;
- a mandatory message broker;
- business workflow logic;
- transport-specific business services.

## Proposed crate boundaries

```text
uc-operation        identifiers, contracts, invocation context and outcome
uc-runtime          composition, invocation pipeline and runtime profile
uc-lifecycle        lifecycle states, shutdown and readiness
uc-config           typed configuration, revision and reload contract
uc-observability    logging/tracing/metrics/health ports and correlation
uc-work             scheduler, job and worker execution contracts
uc-extension        governed extension registration and compatibility
```

Crates may be consolidated where separate packages would add cost without enforcing a meaningful boundary. Final boundaries require a component scorecard and architecture review.

## Invocation pipeline

```text
adapter decode
 -> request/identity/correlation context
 -> deadline and cancellation
 -> authorization decision
 -> operation invocation
 -> transaction/unit-of-work policy
 -> domain/application behavior
 -> effects/outbox
 -> metrics, traces and economic observation
 -> adapter encode
```

Each stage must be optional or replaceable through an explicit contract. The pipeline must not become a hidden business-rule engine.

## Runtime profiles

- `central`: complete configured capability set and global integrations;
- `store-edge`: declared offline-capable subset with local persistence and sync;
- `warehouse-edge`: future profile using the same Operation contracts;
- test profiles: deterministic in-memory and provider-backed composition.

## Quality and cost constraints

- no runtime dependency in domain crates;
- no hidden global mutable state;
- no service locator accessible from Operations;
- allocation and latency budgets per invocation;
- idle CPU and memory budgets per profile;
- one operation must be callable through multiple adapters without semantic drift;
- all public contracts documented and covered by fixtures/tests;
- provider components selected with cost-to-serve scorecards.

## Open questions

- trait object versus generic/static composition per boundary;
- sync/async Operation contract and cancellation semantics;
- transaction boundary ownership;
- reloadable versus restart-required configuration;
- extension ABI stability before 1.0;
- scheduler guarantees and persistent job ownership;
- minimum embedded edge footprint.

## Exit evidence

- accepted ADRs for unresolved questions;
- operation contract and invocation tests;
- lifecycle failure tests;
- two delivery adapters calling the same Operation;
- central and edge composition proof;
- benchmark and economic report;
- architecture dependency evidence.
