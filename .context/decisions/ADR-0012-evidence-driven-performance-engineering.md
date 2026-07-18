# ADR-0012: Evidence-driven performance engineering

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #29

## Context

UC Rust is expected to be exceptionally efficient in latency, throughput, CPU, memory, allocation behavior, I/O and database usage. A generic aspiration to be fast is not enforceable and can lead to premature optimization, hidden regressions or optimizations that damage correctness and maintainability.

## Decision

UC Rust adopts evidence-driven performance engineering.

1. Critical operations require explicit, versioned performance budgets.
2. Budgets cover latency percentiles, throughput, concurrency, CPU per operation, resident and peak memory, allocations, startup time, binary/container size and database behavior where applicable.
3. Representative workloads, datasets and fixtures are versioned with the code.
4. Benchmarks must be reproducible locally and in controlled CI jobs.
5. Performance-sensitive changes require before/after evidence.
6. Regressions above approved thresholds fail CI or block release promotion.
7. CPU, memory, allocation, lock, async scheduling and I/O profiles are required for critical workloads.
8. Critical database paths require query-count assertions, bounded result shapes and explain-plan evidence.
9. N+1 access, unbounded scans and accidental full materialization are prohibited.
10. Pagination, batching, streaming and backpressure are explicit where data volume can grow.
11. Indexes are justified against measured read and write workloads.
12. Resource requests and limits in deployment packaging derive from measured capacity, not arbitrary defaults.
13. Correctness, security and architectural boundaries are not weakened for unmeasured micro-optimizations.

## Performance evidence hierarchy

1. Correct representative benchmark or load test.
2. Profile identifying the limiting resource.
3. Optimization targeted at the measured bottleneck.
4. Regression test and budget protecting the result.
5. Updated capacity and deployment guidance.

## Consequences

- Performance becomes a continuous quality attribute rather than a late project phase.
- Some benchmark and profiling jobs may run separately from the fastest pull-request checks.
- Abstractions must remain measurable and may expose capability or batching semantics when required.
- Database schema and query design are reviewed against workloads.
- Claims such as “faster” or “more efficient” require attached evidence.

## Rejected alternatives

- Optimizing primarily from intuition.
- Using only average latency.
- Benchmarking with trivial datasets that hide scaling behavior.
- Accepting database queries without query plans or bounded cardinality.
- Setting Kubernetes resources before measuring real workloads.
