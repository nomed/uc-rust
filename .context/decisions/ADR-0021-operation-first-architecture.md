# ADR-0021 — Operation First Architecture

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #47
- Parent epic: #46
- Related: ADR-0002, ADR-0015, ADR-0018, ADR-0024

## Context

UC Rust exposes the same Unified Commerce capabilities through REST, gRPC, workers, jobs, CLI, synchronization and future touchpoints. Allowing each adapter to own procedures would duplicate business logic, create semantic drift and make central/edge behavior diverge.

The canonical Operation contract must also remain stable when its underlying capability is realized natively, delegated to an external specialist system, composed from multiple realizations or executed as a governed pipeline.

## Decision

Every business use case is exposed as one canonical application **Operation**.

An Operation:

- has a stable identifier and owner;
- declares typed input, output and errors;
- executes through a shared invocation contract;
- receives an explicit execution context;
- owns authorization and transaction-boundary requirements at application level;
- is independent from transport, scheduler, broker and persistence technology;
- is reusable by central and edge runtime profiles;
- emits trace, metric and economic correlation data through ports;
- has canonical human-readable fixtures and contract tests;
- is invoked identically regardless of its selected Capability Realization.

REST controllers, gRPC services, CLI commands, workers, schedulers and synchronization handlers are delivery adapters. They decode, validate transport concerns, invoke an Operation and encode the result. They do not implement business procedures and do not select or invoke external providers directly.

## Runtime relationship

`uc-runtime` composes and invokes Operations. It owns lifecycle and cross-cutting execution concerns, but does not absorb domain logic.

When an Operation depends on a capability, the runtime resolves an approved realization according to ADR-0024. The application contract remains canonical while the realization may be native, delegated, composed, pipeline or hybrid.

```text
Delivery adapter
  -> canonical Operation invocation
    -> Application Operation
      -> capability-oriented port
        -> Capability Binder
          -> governed Capability Realization
```

## Consequences

- Business meaning exists in one canonical Operation contract.
- All delivery adapters share semantics and tests.
- Operation metadata can drive authorization, observability, Economics by Design and capability discovery.
- Runtime APIs remain smaller than the application model and provider-neutral.
- Direct calls from adapters to repositories or provider SDKs are forbidden.
- Provider replacement or routing does not create a new consumer-facing Operation.
- Native and external realizations must pass shared semantic conformance evidence.

## Rejected alternatives

- Controller/service-first architecture: couples behavior to transport conventions.
- Separate central and edge services: duplicates semantics.
- Provider-specific Operations such as `CallRGK`: leaks realization identity into the business contract.
- Generic service locator available to business code: hides dependencies and weakens architecture tests.
- Direct adapter-to-provider calls: bypass canonical semantics and cross-cutting controls.
- Mandatory dynamic plugins for internal composition: adds complexity before a proven extension requirement.

## Required evidence

- One Operation invoked unchanged by at least REST, gRPC/CLI or worker-style adapters.
- The same Operation bound to at least one native and one delegated realization.
- Architecture tests preventing adapter-to-repository and adapter-to-provider bypass.
- Shared semantic fixtures and contract tests across realizations.
- Operation- and realization-level trace and economic correlation.
