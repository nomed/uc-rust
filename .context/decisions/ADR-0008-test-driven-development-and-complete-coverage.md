# ADR-0008 — Test-driven development and complete coverage

- Status: Accepted
- Date: 2026-07-18
- Decision owners: Daniele Favara
- Governing issue: #25

## Context

UC Rust is intended to remain clean, modular and safe to evolve through humans and agents. High confidence requires every production behavior, branch, adapter contract and migration path to be executable and verifiable. Coverage alone is insufficient, but uncovered production paths are not acceptable.

## Decision

UC Rust adopts test-driven development as the default implementation workflow and requires 100% line and branch coverage for production Rust code.

The standard development cycle is red, green, refactor:

1. add or change a test that demonstrates the required behavior and initially fails;
2. implement the smallest correct change;
3. refactor while preserving behavior and coverage.

Verification must use the appropriate test level rather than forcing every concern into unit tests:

- domain unit and property tests for invariants and state transitions;
- application tests for use-case orchestration and error semantics;
- shared contract suites for infrastructure adapters;
- serialization round-trip and compatibility tests for DTOs;
- integration tests for composition and real infrastructure;
- clean-install and upgrade-path tests for migrations;
- end-to-end tests for critical vertical slices.

Coverage is a mandatory gate but not a quality proxy by itself. Tests must contain meaningful assertions and exercise successful, negative, boundary and concurrency behavior where relevant.

Generated code and genuinely unreachable defensive paths may be excluded only through an explicit, reviewed and narrowly scoped policy. Exclusions cannot be used to hide difficult-to-test design.

Critical commercial and lifecycle rules should additionally use mutation testing or equivalent evidence when practical.

## Consequences

- CI fails when production line or branch coverage drops below 100%.
- Each new production path requires an automated test.
- Testability becomes an architectural requirement.
- Large implementation-first changes are rejected.
- Slow, flaky or nondeterministic tests must be redesigned rather than ignored.
- Coverage tooling and exact exclusion syntax must be selected before implementation resumes.

## Alternatives considered

### Lower global threshold

Rejected because it permits unowned gaps to accumulate and makes omissions negotiable.

### Coverage only for changed lines

Rejected as the final policy because legacy gaps would remain indefinitely, though changed-line reporting may supplement the global gate.

### Implementation-first with tests added later

Rejected because it increases coupling and makes tests validate the implementation rather than drive the contract.
