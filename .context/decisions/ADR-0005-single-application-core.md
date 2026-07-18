# ADR-0005 — Single application core for all delivery mechanisms

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #22
- Decision owner: Human project owner

## Context

UC Rust may expose the same business capabilities through REST, gRPC, message consumers, workers, scheduled jobs, CLI tools and batch processes. Without a strict boundary, each adapter can gradually acquire its own procedure, validation, state transition or commercial decision. That produces duplicated business logic, inconsistent behavior and code that cannot be safely evolved.

## Decision

UC Rust will use a hexagonal, ports-and-adapters architecture with a use-case-oriented application core.

There will be one canonical implementation of every business operation.

### Responsibilities

- **Domain layer** owns business invariants, state transitions, value objects and domain behavior.
- **Application layer** owns use cases, command/query handlers, transaction boundaries, idempotency orchestration and cross-aggregate/process coordination.
- **Inbound adapters** such as REST, gRPC, messaging, workers, jobs and CLI translate transport input, establish technical context, invoke the application API and translate results.
- **Outbound adapters** implement repositories, external services, messaging, clock, identity and other ports defined inward.

Inbound adapters must not implement pricing rules, eligibility rules, lifecycle transitions, workflow decisions, retry semantics with business meaning, or duplicated procedural sequences.

A worker or scheduled job is not a second implementation of a use case. It is another trigger for the same application operation.

## Structural rules

- Dependency direction is inward: adapters -> application -> domain.
- Domain and application crates do not depend on Axum, Tonic, SQLx, Kafka clients, schedulers or other delivery/infrastructure frameworks.
- Transport DTOs are not domain types.
- Shared business procedures are not placed in generic utility modules; they are named application operations or domain behaviors.
- Process managers/sagas are explicit application components when orchestration spans multiple aggregates or asynchronous steps.
- Exceptions require an accepted ADR or RFC.

## Verification

- Architecture tests must enforce allowed crate dependencies.
- Contract tests must prove that different adapters produce equivalent business outcomes for the same operation.
- Reviews must check for duplicated decision logic and suspicious branching inside adapters.
- Each public adapter endpoint or message handler must identify the canonical application operation it invokes.

## Consequences

### Positive

- Business behavior is consistent across channels.
- New interfaces can be added without copying procedures.
- Domain testing remains fast and framework-independent.
- Extraction into services remains possible because boundaries are explicit.

### Negative

- Mapping code is required at adapter boundaries.
- Application APIs must be designed carefully to serve multiple transports.
- Some apparently simple handlers will delegate through more layers than ad-hoc code would require.

## Alternatives rejected

- Framework-centric service classes per interface: encourages duplicated procedures and framework coupling.
- Shared helper functions called by adapters: hides business operations and weakens ownership.
- Active Record with logic spread across controllers and models: makes boundaries and transaction ownership ambiguous.
