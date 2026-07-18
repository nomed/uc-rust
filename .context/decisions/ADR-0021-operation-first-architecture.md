# ADR-0021 — Operation First Architecture

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #47
- Parent epic: #46
- Related: ADR-0002, ADR-0015, ADR-0018

## Context

UC Rust exposes the same Unified Commerce capabilities through REST, gRPC, workers, jobs, CLI, synchronization and future touchpoints. Allowing each adapter to own procedures would duplicate business logic, create semantic drift and make central/edge behavior diverge.

## Decision

Every business use case is implemented as one canonical application **Operation**.

An Operation:

- has a stable identifier and owner;
- declares typed input, output and errors;
- executes through a shared invocation contract;
- receives an explicit execution context;
- owns authorization and transaction-boundary requirements at application level;
- is independent from transport, scheduler, broker and persistence technology;
- is reusable by central and edge runtime profiles;
- emits trace, metric and economic correlation data through ports;
- has canonical human-readable fixtures and contract tests.

REST controllers, gRPC services, CLI commands, workers, schedulers and synchronization handlers are delivery adapters. They decode, validate transport concerns, invoke an Operation and encode the result. They do not implement business procedures.

## Runtime relationship

`uc-runtime` composes and invokes Operations. It owns lifecycle and cross-cutting execution concerns, but does not absorb domain logic.

```text
Delivery adapter
  -> Operation invocation
    -> Application operation
      -> Domain behavior and capability ports
```

## Consequences

- Business behavior exists in one place.
- All delivery adapters share semantics and tests.
- Operation metadata can drive authorization, observability, Economics by Design and capability discovery.
- Runtime APIs must remain smaller than the application model and provider-neutral.
- Direct calls from adapters to repositories or provider SDKs are forbidden.

## Rejected alternatives

- Controller/service-first architecture: couples behavior to transport conventions.
- Separate central and edge services: duplicates semantics.
- Generic service locator available to business code: hides dependencies and weakens architecture tests.
- Mandatory dynamic plugins for internal composition: adds complexity before a proven extension requirement.

## Required evidence

- One operation invoked unchanged by at least REST, gRPC/CLI or worker-style adapters.
- Architecture tests preventing adapter-to-repository bypass.
- Shared fixtures and contract tests.
- Operation-level trace and economic correlation.
