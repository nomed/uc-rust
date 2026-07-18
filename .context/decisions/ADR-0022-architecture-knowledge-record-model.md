# ADR-0022 — Architecture Knowledge Record Model

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #56
- Parent epic: #55
- Related: ADR-0021, RFC-0001

## Context

UC Rust already uses ADRs, RFCs, sessions, issues and implementation evidence, but these artifacts do not yet form one coherent architecture language. Capabilities, runtime responsibilities, quality attributes, economics, security, interfaces, data and releases require different semantics and verification evidence.

Using only generic documents or ADRs would make accepted intent, implementation state, verification state and release inclusion ambiguous. Creating an unconstrained document type for every concern would produce taxonomy proliferation and weak interoperability with UC-BoK, Yukh and Economics by Design.

## Decision

UC Rust adopts a governed **Architecture Knowledge Record Model**.

### 1. Normative record families for the 1.0 baseline

- `ADR` — Architecture Decision Record;
- `RFC` — Request for Comments;
- `CR` — Capability Record;
- `RRR` — Runtime Responsibility Record;
- `QAR` — Quality Attribute Record;
- `ER` — Economic Record;
- `SR` — Security Record;
- `IR` — Interface Record;
- `DR` — Data Record;
- `RR` — Release Record.

A record type is justified by a distinct semantic responsibility, not by document layout.

`TR — Technology Record` is deferred from the 1.0 normative baseline. Technology selection remains an ADR; technology evaluations and operating observations remain evidence unless a future ADR demonstrates unique invariants and at least two concrete normative records.

`RRR` owns reusable execution semantics and operational constraints independently of implementation technology and deployment form.

`RR` owns normative release composition, compatibility/migration constraints, entry/exit gates, accepted exceptions and release disposition. Tests, reports, deployment observations and audit results remain external evidence references.

### 2. Common envelope

Every normative record has a common envelope containing stable identity/namespace, type/schema version, title/summary, lifecycle status/content version, accountable owners/reviewers, dates, scope/non-goals, provenance, typed relations, evidence references, freshness and supersession metadata.

Type-specific bodies extend this envelope but may not redefine its semantics.

### 3. Records and evidence are different entities

Records express governed intent or knowledge. Code, tests, benchmarks, traces, reports, deployments and operational observations are evidence. Evidence may support lifecycle transitions but does not silently mutate a record or become normative through release bundling.

### 4. Orthogonal state dimensions

Record lifecycle is distinct from epistemic confidence, implementation conformance, verification freshness and release inclusion. An accepted record may be unimplemented, partially conforming or excluded from a release.

### 5. Typed relationships

Normative records use a controlled directional relationship vocabulary. Generic `related_to` links are insufficient for architecture traceability.

### 6. Canonical human-readable source

The authoritative representation is Markdown with machine-validatable YAML front matter:

- front matter contains the common envelope and typed relations;
- the Markdown body contains type-specific normative content;
- generated JSON, indexes and graph projections are derived artifacts;
- the repository remains understandable without a graph database or Yukh.

### 7. Identity resolution

Canonical identifiers use `<namespace>:<local-id>`. The namespace identifies the authoritative repository or registry. Local identifiers are immutable; titles, aliases and file paths are not identity. Cross-repository references preserve the owning identifier. Yukh indexes and reconciles identifiers but does not become their authority.

### 8. Taxonomy extension rule

A new record family requires all of:

1. a semantic responsibility not safely owned by an existing family;
2. distinct required fields or lifecycle constraints;
3. at least two credible concrete uses;
4. an ADR accepting the extension;
5. schema and migration impact defined.

## Consequences

### Positive

- Capability and runtime design become explicit before implementation.
- UC-BoK concepts can be traced to executable evidence without describing code structure.
- Yukh can project state and relationships without becoming source of truth.
- Economics and quality attributes become first-class constraints.
- Impact, coverage, freshness and release-readiness queries become possible.

### Costs and risks

- More design discipline is required before implementation.
- Schemas and migration rules must be maintained.
- Over-modeling remains possible and is controlled by the extension rule.
- Existing artifacts need incremental migration.

## Rejected alternatives

- **ADRs only:** insufficient for capabilities, quality scenarios and release composition.
- **Free-form Markdown only:** readable but not reliably validatable.
- **Graph database as authority:** premature infrastructure dependency and poor Git-native review.
- **Yukh as record authority:** reverses the intended relationship.
- **One universal record body:** optional-field sprawl and weak ownership.
- **Technology Record in the 1.0 baseline:** currently lacks two concrete normative uses and unique invariants.

## Validation evidence produced

- `docs/knowledge/common-record-envelope.md`;
- `docs/knowledge/record-taxonomy.md`;
- `docs/knowledge/records/CR-0001-basket-capability.md`;
- `docs/knowledge/records/RRR-0001-operation-invocation.md`;
- `docs/knowledge/record-model-validation.md`.

The examples validate the distinction between capability semantics and reusable runtime invocation semantics.

## Required evidence before acceptance

- lifecycle model accepted;
- relationship model accepted;
- schemas validate the common envelope and canonical CR/RRR records;
- migration path for existing ADRs and RFCs documented;
- UC-BoK, Yukh and EbD responsibility/identifier boundaries confirmed;
- accountable human review of CR-0001 and RRR-0001.

Human acceptance remains mandatory under `.context/manifest.yaml`.
