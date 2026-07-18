# ADR-0001: Start as a modular monolith

- Status: Accepted
- Date: 2026-07-18

## Context

Unified Commerce crosses many business capabilities and can easily become a distributed monolith if service boundaries are chosen before the domain is understood.

The project also needs a coherent Rust golden path before introducing operational complexity such as service discovery, distributed tracing, network retries and independent deployments.

## Decision

UC Rust starts as a Cargo workspace implementing a modular monolith.

The initial layers are:

- `uc-domain`: pure business model and invariants;
- `uc-application`: use cases and ports;
- infrastructure and transport adapters added outside the domain;
- `uc-server`: executable composition root.

Modules must not access another module's persistence directly. Communication between future bounded contexts happens through explicit application interfaces or versioned events.

A module can be extracted into a deployable service only when at least one of these conditions is demonstrated:

- independent scalability;
- fault or security isolation;
- distinct ownership;
- independent release cadence;
- different runtime or deployment requirements.

## Consequences

Positive:

- low operational overhead;
- fast refactoring while boundaries evolve;
- transactions remain local where appropriate;
- one executable walking skeleton.

Trade-offs:

- boundaries require architectural tests and review discipline;
- independent deployment is deferred;
- high-load modules may need later extraction.
