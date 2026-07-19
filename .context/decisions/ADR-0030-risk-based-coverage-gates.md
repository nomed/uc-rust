# ADR-0030: Risk-based coverage gates

## Status

Accepted

## Context

The Runtime Foundation governance initially required 100% line, function, and region coverage. That target made the metric an end in itself and encouraged low-value tests for generated, defensive, process-entry, and transport setup paths.

Authoritative industry guidance treats coverage as a useful but lossy risk indicator rather than a complete measure of test quality. Google Testing guidance classifies 90% coverage as exemplary, warns that pursuing 100% can create waste and false confidence, and recommends stronger attention to changed code and to the specific uncovered behavior. Martin Fowler similarly warns that numeric targets can be gamed and considers thoughtful coverage in the upper 80s or 90s healthy while treating 100% with suspicion.

The Runtime Foundation still needs an ambitious merge gate because it defines canonical execution, cancellation, deadline, tracing, and transport semantics.

## Decision

Use **90% line, function, and region coverage** as the minimum aggregate merge gate for governed Runtime Foundation packages.

Coverage is complemented by mandatory behavior-oriented evidence:

- red-green-refactor history for new behavior and defects;
- direct semantic tests for canonical Operations;
- live adapter conformance tests for public transport behavior;
- explicit cancellation, deadline, error mapping, and tracing tests;
- review of the exact uncovered lines and branches;
- no decrease below the agreed gate.

Critical semantic paths are expected to be fully covered even when aggregate package coverage is below 100%. Generated code and process wiring must not receive artificial tests solely to increase the percentage.

## Consequences

- CI fails below 90% for lines, functions, or regions.
- A value above 90% is desirable when produced by meaningful tests.
- Coverage alone never proves correctness or test quality.
- Reviewers must inspect uncovered behavior and may require additional tests regardless of the aggregate percentage.
- Future work should add changed-code coverage or mutation testing when the toolchain supports it reliably.

## References

- Google Testing Blog, “Code Coverage Best Practices” (2020).
- Google Testing Blog, “Measuring Coverage at Google” (2014).
- Martin Fowler, “Test Coverage” (2012).
