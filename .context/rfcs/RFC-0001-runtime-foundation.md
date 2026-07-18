---
id: uc-rust:RFC-0002
type: RFC
schema_version: 1
content_version: 0.3.0
title: UC Runtime Foundation
summary: Define the smallest shared runtime required to execute canonical UC Rust Operations consistently across central, store-edge and future runtime profiles.
status: Draft
owners:
  - role: runtime-architecture
authors:
  - role: architecture
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Shared runtime invocation, lifecycle, configuration, observability, work scheduling, capability binding and extension responsibilities.
non_goals:
  - General-purpose dependency injection container
  - Custom async executor
  - Internal distributed consensus implementation
  - Arbitrary dynamic code loading
  - Mandatory message broker
  - Business workflow logic
  - Transport-specific business services
provenance:
  - kind: migration
    locator: governance/record-migration-registry.yaml
relations:
  - type: depends_on
    target: uc-rust:ADR-0021
  - type: depends_on
    target: uc-rust:ADR-0024
  - type: depends_on
    target: uc-rust:ADR-0025
review:
  required_roles: [runtime-architecture, reliability, security]
  reviewers: []
  disposition: pending
aliases:
  - RFC-0001-runtime-foundation.md
  - RFC-0001 — UC Runtime Foundation
lifecycle_events:
  - from: null
    to: Draft
    at: 2026-07-18
    actor: architecture
    rationale: Initial draft; canonical identifier remapped during 1.0 migration to resolve duplicate RFC-0001 reservation.
    content_version: 0.1.0
---

# RFC-0002 — UC Runtime Foundation

## Objective

Define the smallest shared runtime required to execute canonical UC Rust Operations consistently across central, store-edge and future runtime profiles.

## Scope

The Runtime Foundation covers:

1. operation registration and invocation;
2. typed execution context and cancellation/deadline propagation;
3. lifecycle, startup, readiness, degradation, quiesce, drain and graceful shutdown;
4. explicit dependency composition;
5. typed immutable configuration snapshots, validation, revision and safe reload;
6. structured errors and failure correlation;
7. logging, tracing, metrics and health contribution contracts;
8. scheduled and background work execution through Operations;
9. runtime capability discovery;
10. Governed Capability Realization manifests, binding and invocation handles;
11. extension registration boundaries;
12. operation-, component- and realization-level Economics by Design correlation.

## Non-goals

The first runtime release does not provide:

- a general-purpose dependency injection container;
- a custom async executor;
- an internal distributed consensus implementation;
- arbitrary dynamic code loading;
- a mandatory message broker;
- business workflow logic;
- transport-specific business services;
- provider-specific Operations or unrestricted provider clients;
- implicit fallback that changes authority or semantic guarantees;
- mutable global configuration or direct environment reads from Operations.

## Proposed crate boundaries

```text
uc-operation        identifiers, contracts, invocation context and outcome
uc-runtime          composition, invocation pipeline and runtime profile
uc-capability       realization manifests, binding decisions and invocation handles
uc-lifecycle        lifecycle states, readiness, degradation, drain and shutdown
uc-config           typed snapshots, validation, revision and reload protocol
uc-observability    logging/tracing/metrics/health ports and correlation
uc-work             scheduler, job and worker execution contracts
uc-extension        governed extension registration and compatibility
```

Crates may be consolidated where separate packages would add cost without enforcing a meaningful boundary. Final boundaries require a component scorecard and architecture review. `uc-capability` is a logical boundary; it need not become a separate crate if the same dependency constraints are enforced inside `uc-runtime`.

## Invocation pipeline

```text
adapter decode
 -> request/identity/correlation context
 -> deadline and cancellation
 -> authorization decision
 -> canonical Operation invocation
 -> Capability Binder when the Operation requires a capability port
 -> governed realization invocation
 -> transaction/unit-of-work policy where applicable
 -> domain/application behavior and canonical outcome
 -> effects/outbox
 -> metrics, traces and economic observation
 -> adapter encode
```

Each stage must be optional or replaceable through an explicit contract. The pipeline must not become a hidden business-rule engine. Delivery adapters never select realizations and Operations never receive an unrestricted service locator.

## Lifecycle and composition contract

ADR-0025 governs lifecycle, configuration and composition.

The runtime uses the lifecycle:

```text
Created -> Bootstrapping -> Starting -> Ready/Degraded
Ready/Degraded -> Quiescing -> Draining -> Stopping -> Stopped
startup or invariant failure -> Failed
```

Readiness is profile- and Operation-set-specific. Process liveness, runtime readiness and capability operability are independent. Store-edge may remain operational for an approved offline subset while central connectivity and remote realizations are unavailable.

Every executable has one explicit composition root. It loads an accepted immutable configuration snapshot, constructs typed providers, validates Operation and realization compatibility, wires typed ports, registers lifecycle-managed components and emits a redacted composition manifest. No application Operation can access a service locator, global mutable state or arbitrary configuration source.

Components declare dependencies, criticality, affected Operations, start/stop deadlines, reload class, health contribution and resource ownership. Startup follows dependency order; rollback and shutdown use reverse order.

## Configuration and reload contract

Configuration snapshots are schema-versioned, content-digested and immutable. Fields are classified as:

- `dynamic_safe`;
- `drain_then_reload`;
- `restart_required`;
- `immutable_identity`.

Reload follows prepare, validate, optional quiesce/drain, atomic commit, activate and verify. A rejected candidate leaves the current snapshot unchanged. Partial mutation and silent mixed revisions are forbidden. In-flight invocations retain the configuration and binding revisions captured at admission.

The normative schema is `governance/schemas/runtime-profile-config.schema.json`; detailed semantics are in `docs/architecture/runtime-lifecycle-and-composition.md`.

## Governed Capability Realization runtime contract

The runtime:

- loads and validates approved realization manifests;
- resolves an Operation through a deterministic, versioned Capability Binder;
- enforces semantic revision, authority, profile, security, compatibility, deadline, offline and quality constraints before economic preference;
- returns an invocation handle restricted to the selected Operation and execution context;
- isolates provider credentials, SDK types and diagnostics inside provider adapters;
- preserves binding revision for the lifetime of an invocation;
- records realization, provider, policy, authority, latency, failure and economic attribution;
- applies explicit fallback, reconciliation and circuit-breaker policies;
- rejects conflicting equal-precedence bindings as invalid configuration.

The runtime does not claim that remote and local realizations have identical operational behavior. It normalizes canonical semantics while retaining explicit latency, partial-failure, idempotency, authority and offline differences.

## Runtime profiles

- `central`: complete configured capability set and global integrations;
- `store-edge`: declared offline-capable subset with local persistence, durable local manifests, local/delegated realizations and sync;
- `warehouse-edge`: future profile using the same Operation contracts;
- test profiles: deterministic in-memory, native, delegated and composed realization composition.

A profile advertises eligible realization manifests and offline classes. Edge deployment alone never implies that an Operation has an offline-capable realization. WAN connectivity is not a universal store-edge readiness dependency.

## Quality and cost constraints

- no runtime dependency in domain crates;
- no hidden global mutable state;
- no service locator accessible from Operations;
- no provider SDK type in canonical Operation contracts;
- allocation and latency budgets per invocation and per realization stage;
- startup, readiness, idle CPU and memory budgets per profile;
- bounded quiesce/drain/shutdown deadlines;
- independent provider concurrency, bulkhead and circuit-breaker budgets;
- one Operation must be callable through multiple adapters without semantic drift;
- native and delegated realizations must pass the same semantic conformance fixtures;
- all public contracts documented and covered by fixtures/tests;
- provider and composition choices selected with cost-to-serve scorecards only after semantic, authority, security and quality eligibility.

## Open questions

- final physical crate consolidation after component scorecards;
- scheduler guarantees and persistent job ownership;
- minimum embedded edge footprint;
- durable reconciliation ownership for indeterminate external outcomes;
- exact health/operability aggregation contract, governed by #49.

## Exit evidence

- accepted ADR-0021, ADR-0024 and ADR-0025;
- operation contract and invocation tests;
- lifecycle state-machine and startup rollback tests;
- atomic configuration reload and rejected-candidate tests;
- quiesce, drain and shutdown tests;
- two delivery adapters calling the same Operation;
- native and delegated realizations passing shared canonical fixtures;
- binding tests across at least tenant and runtime-profile dimensions;
- failure, timeout, idempotency, fallback and indeterminate-outcome tests;
- one governed composed/pipeline proof;
- central and edge composition proof, including WAN-loss operability;
- benchmark and component/realization-attributed economic report;
- architecture dependency evidence preventing adapter bypass, provider leakage, mutable globals and service-locator access.