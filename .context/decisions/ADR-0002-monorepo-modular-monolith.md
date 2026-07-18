# ADR-0002 — Cargo monorepo with an initial modular monolith

- Status: accepted
- Date: 2026-07-18
- Deciders: project owner
- Governing issue: #12
- Supersedes: none
- Superseded by: none

## Context

The project must support multiple Cargo crates, applications, contracts, migrations, containers, Helm packaging and governance in one coherent product. The domain is broad, but service boundaries and operational scaling requirements are not yet validated.

## Decision

We will use one Cargo monorepo for UC Rust. The initial application architecture will be a modular monolith with explicit domain and application boundaries.

Physical service extraction is permitted only when justified by independently validated needs such as scaling, isolation, ownership, failure containment or release independence. Extraction must preserve contracts and requires an accepted RFC.

## Consequences

The project gets one source tree, coordinated tooling and fast refactoring across boundaries. Runtime complexity remains lower during discovery. Internal modularity and dependency rules must be enforced because process boundaries will not protect them. A future multi-service architecture may remain in the same monorepo.

## Alternatives considered

- Multiple repositories from the start: rejected because boundaries and ownership are not mature enough.
- Immediate microservices: rejected because it adds distributed-system cost before evidence exists.
- Unstructured single crate: rejected because it would hide domain boundaries and hinder extraction.

## Evidence

- The Cargo workspace contains domain, application and executable crates.
- Issue #12 owns the complete target architecture.

## Compliance

New crates and deployables must declare their responsibility and dependency direction. New independently deployable services require an RFC.