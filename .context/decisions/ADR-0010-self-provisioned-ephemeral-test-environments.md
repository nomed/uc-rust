# ADR-0010: Self-provisioned ephemeral test environments

- Status: Accepted
- Date: 2026-07-18
- Decision owners: Project owner
- Governing issues: #17, #23, #24, #25, #27

## Context

UC Rust requires exhaustive unit, contract, integration, migration and end-to-end verification. Some suites require real databases, caches, object storage, brokers or other external services. Depending on manually provisioned or shared test infrastructure would make the test system non-reproducible, stateful, difficult to debug and unsafe for parallel execution.

## Decision

GitHub Actions and local development must be able to provision every external test dependency from versioned repository configuration.

The project adopts a layered orchestration model:

1. Rust Testcontainers for test-owned, capability-scoped dependencies and reusable adapter contract suites.
2. Docker Compose for multi-service system tests, end-to-end environments and scenarios developers must start manually.
3. GitHub Actions service containers only for simple job-wide dependencies when they materially reduce complexity.

All environments are ephemeral, isolated and destroyed automatically.

## Required properties

- Container images and service versions are pinned deliberately.
- Readiness is established through explicit health checks or protocol probes, never fixed sleeps.
- Each test job receives isolated schemas, databases, buckets, namespaces, queues and credentials.
- Database migrations run against the real supported engines before integration tests.
- Test fixtures are deterministic and do not depend on shared mutable state.
- Service logs, container metadata and relevant diagnostics are collected on failure.
- The same environment definition is runnable locally and in GitHub Actions.
- External network services are replaced by local containers or deterministic test doubles unless the test explicitly validates an approved live integration.
- Secrets used by tests are synthetic and scoped to the ephemeral environment.

## Consequences

Positive:

- clean runners can reproduce the complete test environment;
- adapter contract tests validate real provider behavior;
- parallel CI execution remains isolated;
- failures include actionable infrastructure diagnostics;
- developers can reproduce CI locally.

Costs:

- container startup increases integration-test duration;
- image pinning and lifecycle maintenance become explicit work;
- test suites must distinguish unit tests from infrastructure-backed tests;
- CI requires Docker-capable runners.

## Compliance

A test suite requiring an external dependency is incomplete until its environment provisioning, readiness, cleanup and failure diagnostics are automated. Shared long-lived test infrastructure is not part of the default verification path.
