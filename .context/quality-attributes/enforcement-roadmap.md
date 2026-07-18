# Quality enforcement roadmap

- Status: Active execution plan
- Governing issues: #19, #37
- Last updated: 2026-07-18

## Objective

Move UC Rust from complete requirement coverage but low operational enforcement to a state where every P0 quality row is implemented, enforced, evidenced and accepted.

The target is not a subjective percentage. Completion means every row in `system-quality-model.md` has executable controls and linked evidence.

## Maturity states

1. **Defined** — requirement, owner, issue and threshold exist.
2. **Implemented** — tool, test, workflow or control exists in the repository.
3. **Enforced** — failure blocks merge, release or deployment as appropriate.
4. **Evidenced** — reproducible artifacts prove the control works on a clean environment.
5. **Accepted** — Project Ready review accepts the evidence or records a governed exception.

No P0 row is complete before `Accepted`.

## Workstreams

### W1 — Product and architecture closure

Governing issues: #11, #12, #30, #38, #39, #40, #41

Deliverables:

- approved `PROJECT_CHARTER.md`;
- approved target architecture;
- UC-BoK revision pinned in traceability manifest;
- initial capability mapping;
- central/edge capability and data-authority matrix;
- public contract/versioning policy;
- compatibility envelope schema.

Exit evidence:

- charter/architecture review disposition;
- traceability validation report;
- zero unresolved contradictory accepted records.

### W2 — Repository and agentic governance

Governing issues: #13, #14, #15, #20, #21, #37

Deliverables:

- machine-readable agent role contracts;
- context-impact declaration in PR template;
- context/ADR/RFC/session/handoff validator;
- manifest synchronization dry-run and apply evidence;
- quality model schema validator;
- cross-repository UC-BoK issue workflow.

Exit evidence:

- intentionally invalid governance fixture fails CI;
- valid repository passes from a clean runner;
- agent-created issue contains reproducible structured fields.

### W3 — Rust golden path and complete correctness

Governing issues: #2, #25, #26, #27, #35

Deliverables:

- pinned Rust/MSRV and tool versions;
- one-command bootstrap/check/test/docs/coverage;
- `cargo nextest` or accepted equivalent;
- line and branch coverage gate at 100%;
- rustdoc missing-doc gate;
- canonical fixture validator and round-trip tests;
- mutation testing for selected critical rule;
- deterministic self-provisioned test environments.

Exit evidence:

- clean runner passes all commands;
- deliberate uncovered branch and missing docs fail;
- fixture/schema mismatch fails;
- integration suite provisions and tears down dependencies.

### W4 — Architecture and provider boundaries

Governing issues: #22, #23, #24, #31

Deliverables:

- crate dependency/forbidden-import checks;
- operation ownership manifest;
- repository/cache/storage/AuthZ ports;
- shared adapter contract harnesses;
- provider capability validation;
- forward-only migration framework and tests;
- idempotency and concurrency contracts.

Exit evidence:

- prohibited provider import in core fails CI;
- PostgreSQL/SQLite adapters pass identical behavior suite;
- clean-install and upgrade migration tests pass;
- duplicate/retry scenarios preserve one business effect.

### W5 — Security, identity and data governance

Governing issues: #16, #33, #34, #36

Deliverables:

- threat model;
- OIDC test provider and negative token tests;
- SpiceDB schema, fixtures and contract tests;
- cross-tenant property tests;
- secret/config/feature-flag validators;
- dependency/license/vulnerability gates;
- SBOM, provenance and signing proof;
- retention/export/erasure tests.

Exit evidence:

- invalid issuer/audience/signature/expiry denied;
- cross-tenant access impossible in property/isolation tests;
- stale/insufficient offline authorization cannot expand privilege;
- signed artifact and SBOM verified;
- tenant erasure evidence covers derived stores and backups policy.

### W6 — Reliability, observability and operability

Governing issues: #28, #32

Deliverables:

- typed error taxonomy and boundary mapping;
- structured logs/traces/metrics and redaction tests;
- retry/timeout/backpressure/circuit/degraded policies;
- sanitized diagnostic bundle;
- stable failure fingerprint and issue generator;
- SLI/SLO and error budgets;
- backup/restore and rollback drills;
- runbooks and incident evidence.

Exit evidence:

- injected failure produces one deduplicated actionable issue;
- secrets/PII redaction test passes;
- restore test meets approved RPO/RTO;
- degraded dependency behavior is explicit and tested.

### W7 — Performance and efficiency

Governing issue: #29

Deliverables:

- representative workloads and datasets;
- Criterion or accepted benchmark harness;
- latency/throughput/CPU/memory/allocation budgets;
- DB query-count and plan assertions;
- load, stress and soak environments;
- baseline storage and regression comparison;
- Helm/runtime resource sizing from evidence.

Exit evidence:

- intentional regression crosses a threshold and fails;
- critical query plan and bounded row behavior recorded;
- memory/CPU profiles attached to evidence;
- resource requests/limits trace to measurements.

### W8 — Distributed edge runtime and synchronization

Governing issue: #39

Deliverables:

- capability/offline matrix completed for first vertical slice;
- data authority/freshness/conflict policies completed;
- durable outbox/inbox and checkpoint model;
- central→edge snapshot/delta protocol;
- edge→central publication/reconciliation protocol;
- bootstrap/re-sync procedure;
- central/edge/API/schema compatibility validation;
- WAN partition, restart, duplicate, reorder and backlog tests.

Exit evidence:

- zero lost or duplicated business effects;
- no silent conflicts;
- edge can operate/restart through prolonged partition;
- re-sync preserves unsent local work;
- unsupported offline capability is clearly reported.

### W9 — Fleet control and safe deployment

Governing issue: #40

Deliverables:

- unique edge identity and certificate lifecycle;
- signed inventory/actual-state reports;
- desired-state reconciliation;
- signed immutable update manifest;
- preflight and compatibility checks;
- staged/A-B or equivalent atomic installation;
- canary/cohort rollout, pause and rollback;
- technical and retail-operational health model;
- drift detection and typed remote diagnostics.

Exit evidence:

- interrupted download/install cannot corrupt active runtime or business data;
- unsigned/revoked/incompatible artifact is rejected;
- canary threshold pauses rollout;
- rollback restores known-good service;
- offline telemetry buffers and reports later.

## Critical path to Project Ready

1. W1 Product and architecture closure.
2. W2 governance validator and quality model schema.
3. W3 minimal CI golden path.
4. W4 architecture boundary and migration proof.
5. W5 identity/security baseline.
6. W8 first central/edge Basket proof.
7. W9 minimal fleet update proof.
8. W6 failure issue and restore proof.
9. W7 performance baseline and regression gate.
10. Project Ready #19 evidence review.

Workstreams may run in parallel after W1 defines stable boundaries, but no feature issue may bypass the applicable gates.

## Progress accounting

A percentage may be reported only from row maturity, using equal or explicitly documented weights:

- Defined: 20%
- Implemented: 40%
- Enforced: 60%
- Evidenced: 80%
- Accepted: 100%

A row still marked `Planned` counts as 0%. The dashboard must compute progress from repository evidence rather than narrative estimates.

## Immediate next executable slice

The first enforcement slice should deliver:

1. context/quality manifest validator;
2. pinned Rust toolchain and one-command check;
3. rustdoc and coverage failure proof on current code;
4. architecture dependency rule;
5. canonical Basket DTO fixture and test;
6. PostgreSQL/SQLite contract-test skeleton;
7. central + store-edge ephemeral test topology;
8. initial Basket offline/authority rows completed;
9. evidence artifact linked into #37.

This slice converts governance and architecture from prose to executable constraints before expanding domain functionality.
