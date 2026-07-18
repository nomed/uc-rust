# Session — Testing, documentation and executable examples

- Date: 2026-07-18
- Governing epic: #10
- Related issues: #19, #25, #26
- Status: Complete

## Objective

Crystallize requirements for complete automated test coverage, test-driven development, comprehensive rustdoc and human-readable executable DTO examples.

## Decisions accepted

- ADR-0008: test-driven development is the default workflow and production Rust code requires 100% line and branch coverage.
- ADR-0009: production APIs require complete rustdoc and external serialized contracts require canonical human-readable executable examples.

## Key outcomes

- Coverage is a blocking CI gate but does not replace meaningful assertions or appropriate test levels.
- Every production behavior, branch, adapter contract, serializer and migration path requires automated verification.
- Generated or genuinely unreachable code may be excluded only through a narrow reviewed policy.
- Canonical JSON payloads should be pretty-printed fixture files.
- Fixtures should be reused by serialization/deserialization tests, API documentation, rustdoc and compatibility tests where practical.
- Round-trip tests must parse canonical input, exercise behavior, serialize output and validate semantic content.
- Snapshot tests may support readability but cannot replace explicit assertions for important behavior.

## Repository changes

- Added issues #25 and #26.
- Added ADR-0008 and ADR-0009.
- Updated Epic #10 and Project Ready gate #19.
- Updated `AGENTS.md` with mandatory testing, coverage, documentation and fixture rules.
- Updated `governance/github-manifest.json` to manage issues #25 and #26.

## Open implementation choices

- Select coverage tooling and exact branch-coverage implementation.
- Define permitted coverage exclusions and approval process.
- Select mutation testing tooling and scope.
- Define rustdoc lint levels for public and internal production APIs.
- Define final fixture directory conventions and OpenAPI integration.
