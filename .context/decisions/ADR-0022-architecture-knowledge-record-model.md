# ADR-0022 — Architecture Knowledge Record Model

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #56
- Parent epic: #55
- Related: #62, ADR-0021, RFC-0001

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

Normatively referenced evidence must be reproducibly identifiable through a canonical evidence identifier or a stable locator with immutable revision/digest and observation timestamp. Anonymous or mutable evidence references are insufficient.

### 4. Orthogonal state dimensions

Record lifecycle is distinct from epistemic confidence, implementation conformance, verification freshness and release inclusion. An accepted record may be unimplemented, partially conforming or excluded from a release.

### 5. Typed relationships

Normative records use a controlled directional relationship vocabulary. Generic `related_to` links are insufficient for architecture traceability.

The authority owning the relation source owns the normative assertion. Incoming relations do not imply endorsement by the target authority. Inferred projection edges remain non-normative until accepted by the source authority.

### 6. Canonical human-readable source

The authoritative representation is Markdown with machine-validatable YAML front matter:

- front matter contains the common envelope and typed relations;
- the Markdown body contains type-specific normative content;
- generated JSON, indexes and graph projections are derived artifacts;
- the repository remains understandable without a graph database or Yukh.

### 7. Identity resolution

Canonical identifiers use `<namespace>:<local-id>`. The namespace identifies the authoritative repository or registry. Local identifiers are immutable; titles, aliases and file paths are not identity. Cross-repository references preserve the owning identifier. Yukh indexes and reconciles identifiers but does not become their authority.

Artifacts referenced normatively use an authority-qualified locator plus immutable revision or digest. Runtime realizations use environment-scoped identifiers and preserve the artifact revision and governed responsibility references they claim to realize.

### 8. Taxonomy extension rule

A new record family requires all of:

1. a semantic responsibility not safely owned by an existing family;
2. distinct required fields or lifecycle constraints;
3. at least two credible concrete uses;
4. an ADR accepting the extension;
5. schema and migration impact defined.

### 9. Architecture meta-model

The 1.0 model consists of the following distinct entities:

- **Concept** — meaning owned by an external authoritative knowledge namespace;
- **Normative Record** — governed UC Rust intent, constraint, decision or accepted knowledge;
- **Relation** — typed directional assertion whose authority follows its source;
- **Evidence** — reproducibly identifiable observation supporting a claim without changing normative meaning;
- **Artifact** — produced implementation or operational object;
- **Projection** — derived index, graph, reconciliation, query or visualization representation;
- **Runtime Realization** — deployable or executing manifestation of governed responsibilities;
- **Namespace Authority** — owner of canonical identifiers and meaning within a namespace.

The normative elaboration, identity policies, allowed dependencies, forbidden conflations and canonical traces are defined in `docs/architecture/architecture-meta-model.md`.

The meta-model imposes these additional invariants:

1. every normative statement has exactly one authority;
2. concepts, records, evidence, artifacts, projections and runtime realizations remain separate entities;
3. projection or runtime state cannot mutate a record;
4. deployment topology cannot redefine capability or runtime-responsibility identity;
5. unresolved external references remain explicit findings rather than local copies;
6. no Deployment Record family is introduced for 1.0; deployments remain artifacts and evidence unless the taxonomy extension rule is later satisfied.

## Consequences

### Positive

- Capability and runtime design become explicit before implementation.
- UC-BoK concepts can be traced to executable evidence without describing code structure.
- Yukh can project state and relationships without becoming source of truth.
- Economics and quality attributes become first-class constraints.
- Impact, coverage, freshness and release-readiness queries become possible.
- Dependent lifecycle, relationship and schema work can use a stable primitive entity model.

### Costs and risks

- More design discipline is required before implementation.
- Schemas and migration rules must be maintained.
- Over-modeling remains possible and is controlled by the extension rule.
- Existing artifacts need incremental migration.
- External identifiers and immutable artifact/evidence locators require explicit resolution behavior.

## Rejected alternatives

- **ADRs only:** insufficient for capabilities, quality scenarios and release composition.
- **Free-form Markdown only:** readable but not reliably validatable.
- **Graph database as authority:** premature infrastructure dependency and poor Git-native review.
- **Yukh as record authority:** reverses the intended relationship.
- **One universal record body:** optional-field sprawl and weak ownership.
- **Technology Record in the 1.0 baseline:** currently lacks two concrete normative uses and unique invariants.
- **Deployment Record in the 1.0 baseline:** deployment identity and observations can be represented as Artifact, Runtime Realization and Evidence without a new normative family.

## Validation evidence produced

- `docs/architecture/architecture-meta-model.md`;
- `docs/knowledge/common-record-envelope.md`;
- `docs/knowledge/record-taxonomy.md`;
- `docs/knowledge/records/CR-0001-basket-capability.md`;
- `docs/knowledge/records/RRR-0001-operation-invocation.md`;
- `docs/knowledge/record-model-validation.md`.

The examples validate the distinction between capability semantics and reusable runtime invocation semantics. The architecture meta-model resolves the primitive entity boundaries required by #62.

## Required evidence before acceptance

- lifecycle model accepted;
- relationship model accepted;
- schemas validate the common envelope and canonical CR/RRR records;
- migration path for existing ADRs and RFCs documented;
- UC-BoK, Yukh and EbD responsibility/identifier boundaries confirmed;
- accountable human review of CR-0001 and RRR-0001.

Human acceptance remains mandatory under `.context/manifest.yaml`. Completion of #62 does not itself accept this ADR.