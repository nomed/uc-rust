# ADR-0032 — Risk-based coverage gates

- Status: Accepted
- Date: 2026-07-19
- Governing issue: #72
- Related: ADR-0028, ADR-0030, ADR-0031, RFC-0002

## Context

The Runtime Foundation governance initially required 100% line, function, and region coverage. That target made the metric an end in itself and encouraged low-value tests for generated, defensive, process-entry, and transport setup paths.

Authoritative industry guidance treats coverage as a useful but lossy risk indicator rather than a complete measure of test quality. Google Testing guidance classifies 90% coverage as exemplary, warns that pursuing 100% can create waste and false confidence, and recommends stronger attention to changed code and to the specific uncovered behavior. Martin Fowler similarly warns that numeric targets can be gamed and considers thoughtful coverage in the upper 80s or 90s healthy while treating 100% with suspicion.

The Runtime Foundation still needs an ambitious merge gate because it defines canonical execution, cancellation, deadline, tracing, and transport semantics.

## Decision

Use **90% line and function coverage** as the minimum aggregate merge gate for executable Runtime Foundation libraries:

- `uc-operation`;
- `uc-runtime`;
- `uc-config`;
- `uc-adapters`.

The `uc-cli` binary composition root is excluded from the aggregate percentage because process startup, argument parsing, stdout/stderr, and listener lifetime are entry-point wiring rather than reusable executable semantics. It remains subject to compilation, Clippy, documentation checks, dedicated tests, and workspace integration tests. This is a scope distinction, not permission to leave CLI behavior unverified.

LLVM region coverage remains diagnostic evidence but is not a merge threshold. Regions reflect compiler lowering and macro expansion as well as source decisions, so their aggregate percentage can be unstable and can reward tests that do not reduce meaningful product risk. Critical branches and semantic decisions remain subject to explicit behavior tests and review.

Coverage is complemented by mandatory behavior-oriented evidence:

- red-green-refactor history for new behavior and defects;
- direct semantic tests for canonical Operations;
- live adapter conformance tests for public transport behavior;
- explicit cancellation, deadline, error mapping, and tracing tests;
- dedicated tests for excluded composition roots;
- review of the exact uncovered lines and branches;
- no decrease below the agreed gate.

Critical semantic paths are expected to be fully covered even when aggregate package coverage is below 100%. Generated code and process wiring must not receive artificial tests solely to increase the percentage.

## Consequences

- CI fails below 90% for lines or functions in the executable library scope.
- Region coverage is retained in generated evidence for diagnosis, not used as an aggregate pass/fail gate.
- The CLI composition root must pass its dedicated test step even though it is outside the aggregate metric.
- A value above 90% is desirable when produced by meaningful tests.
- Coverage alone never proves correctness or test quality.
- Reviewers must inspect uncovered behavior and may require additional tests regardless of the aggregate percentage.
- Future work should add changed-code coverage or mutation testing when the toolchain supports it reliably.

## References

- Google Testing Blog, “Code Coverage Best Practices” (2020).
- Google Testing Blog, “Measuring Coverage at Google” (2014).
- Martin Fowler, “Test Coverage” (2012).
