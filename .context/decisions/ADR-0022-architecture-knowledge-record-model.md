# ADR-0022 — Architecture Knowledge Record Model

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #56
- Parent epic: #55
- Related: ADR-0021, RFC-0001

## Context

UC Rust already uses ADRs, RFCs, sessions, issues and implementation evidence, but these artifacts do not yet form one coherent architecture language. Capabilities, runtime responsibilities, quality attributes, economics, security, interfaces, data and releases require different semantics and verification evidence.

Using only generic documents or ADRs would make accepted intent, implementation state, verification state and release inclusion ambiguous. Creating an unconstrained document type for every concern would produce the opposite problem: taxonomy proliferation and weak interoperability with UC-BoK, Yukh and Economics by Design.

## Decision

UC Rust adopts a governed **Architecture Knowledge Record Model**.

### 1. Normative record families

The initial controlled taxonomy is:

- `ADR` — Architecture Decision Record;
- `RFC` — Request for Comments;
- `CR` — Capability Record;
- `RRR` — Runtime Responsibility Record;
- `QAR` — Quality Attribute Record;
- `ER` — Economic Record;
- `SR` — Security Record;
- `IR` — Interface Record;
- `DR` — Data Record;
- `TR` — Technology Record;
- `RR` — Release Record.

A record type is justified by a distinct semantic responsibility, not by document layout.

### 2. Common envelope

Every normative record has a common envelope containing:

- stable identity and namespace;
- type and schema version;
- title and concise summary;
- lifecycle status and content version;
- owners and accountable reviewers;
- creation, update and review dates;
- bounded scope and explicit non-goals;
- provenance;
- typed relations;
- implementation and verification evidence references;
- freshness and supersession metadata.

Type-specific bodies extend this envelope but may not redefine its semantics.

### 3. Records and evidence are different entities

Records express governed intent or knowledge. Code, tests, benchmarks, traces, reports, deployments and operational observations are evidence. Evidence may support lifecycle transitions but does not silently mutate a record.

### 4. Orthogonal state dimensions

The record lifecycle is distinct from:

- epistemic confidence;
- implementation conformance;
- verification freshness;
- release inclusion.

For example, an accepted record may be unimplemented, partially conforming, or excluded from a release.

### 5. Typed relationships

Normative records use a controlled directional relationship vocabulary. Generic `related_to` links are not sufficient for architecture traceability.

### 6. Human-readable source, machine-validatable projection

The authoritative representation remains reviewable as text in Git. Machine-readable metadata and schemas must be derivable or embedded without requiring a graph database or Yukh to understand the repository.

### 7. Taxonomy extension rule

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
- Yukh can project record state and relationships without becoming the source of truth.
- Economics by Design and quality attributes become first-class constraints.
- Impact, coverage, freshness and release-readiness queries become possible.

### Costs and risks

- More design discipline is required before implementation.
- Schemas and migration rules must be maintained.
- Over-modeling remains possible and must be controlled by the extension rule.
- Existing artifacts need incremental migration rather than bulk rewriting.

## Rejected alternatives

- **ADRs only:** insufficient for capabilities, measurable quality scenarios and release composition.
- **Free-form Markdown only:** human-readable but not reliably queryable or validatable.
- **Graph database as authority:** premature infrastructure dependency and poor Git-native review.
- **Yukh as record authority:** reverses the intended relationship; Yukh projects and reconciles the model.
- **One universal record body:** produces optional-field sprawl and weak semantic ownership.

## Required evidence before acceptance

- common envelope specification reviewed;
- lifecycle and relationship model accepted;
- schemas validate canonical records;
- CR-0001 Basket and RRR-0001 Operation Invocation demonstrate the model;
- migration path for existing ADRs and RFCs documented;
- UC-BoK, Yukh and EbD responsibility boundaries confirmed.
