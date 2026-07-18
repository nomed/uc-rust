# SESSION-2026-07-18-06: Reliability and performance invariants

- Date: 2026-07-18
- Governing epic: #10
- Related issues: #19, #28, #29

## Objective

Crystallize hardening, error handling, structured diagnostics, agent-ready issue generation and exceptional resource efficiency as mandatory project qualities.

## Decisions accepted

- ADR-0011: Failure engineering and agent-ready diagnostics.
- ADR-0012: Evidence-driven performance engineering.

## Work completed

- Created issue #28 for typed errors, hardening, logging, tracing, redaction, diagnostic bundles and deduplicated issue automation.
- Created issue #29 for performance budgets, benchmarks, profiling, database efficiency, load testing and regression gates.
- Updated epic #10 to include reliability and performance engineering.
- Updated Project Ready gate #19 to require executable diagnostic and performance controls.

## Key requirements

- Automatically generated issues must contain deterministic reproduction evidence and a stable deduplication fingerprint.
- Failures must be traceable without exposing sensitive data.
- Performance must be expressed through versioned workloads and measurable budgets.
- CPU, memory, allocations, latency, throughput, startup, artifact size and database behavior are governed quality attributes.
- Database critical paths require bounded cardinality, query-count assertions and explain-plan evidence.

## Open implementation work

- Define the concrete Rust error taxonomy and mapping conventions.
- Select structured tracing, metrics and diagnostic artifact tooling.
- Define the issue fingerprint and automation workflow.
- Define the first workload model and quantitative budgets.
- Select benchmark, load-test, profiling and database-plan tooling.
- Add issues #28 and #29 to the declarative GitHub governance manifest.
- Extend AGENTS.md with the accepted reliability and performance rules.

## Risks

- Absolute performance claims are meaningless without workload and hardware context.
- Uncontrolled automated issue creation can create noise; fingerprinting and evidence thresholds are mandatory.
- CI benchmark noise must be handled without masking real regressions.
