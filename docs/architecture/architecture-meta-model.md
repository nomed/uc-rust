# UC Rust 1.0 Architecture Meta-Model

- Status: Draft
- Governing issue: #62
- Parent epic: #55
- Related: #56, ADR-0022

## Purpose

This document defines the minimum conceptual model required to govern and implement UC Rust 1.0. It separates external knowledge, normative architecture, supporting evidence, implementation artifacts, derived projections and runtime realization so that no subsystem silently becomes authoritative for another.

## Core entities

### Concept

A Concept is a meaning owned by an authoritative knowledge namespace, such as UC-BoK. A concept may describe a business capability, domain term, principle or reference model.

A Concept is not automatically a UC Rust record. UC Rust references concepts and may govern a realization of them, but does not duplicate their external authority.

### Normative Record

A Normative Record is a versioned, governed statement for which UC Rust is authoritative. Records include ADR, RFC, CR, RRR, QAR, ER, SR, IR, DR and RR.

A record owns intent, constraints or accepted knowledge. It does not own the code, tests or reports used to demonstrate conformance.

### Relation

A Relation is a typed, directional assertion connecting identified entities. A relation has an authoritative source, a target, optional temporal validity and provenance.

A generic link is not a normative relation. Inferred relations are projections and cannot silently become normative.

### Evidence

Evidence is an immutable or reproducibly identifiable observation supporting a claim about a record, relation, artifact or runtime state. Examples include test results, benchmarks, traces, audits, deployments and operational observations.

Evidence does not change normative meaning. It can support a review or transition but cannot perform one by itself.

### Artifact

An Artifact is a produced implementation or operational object, including source code, binaries, schemas, manifests, runbooks and reports.

Artifacts may implement or support records. Their existence does not imply conformance, verification or release inclusion.

### Projection

A Projection is a derived representation optimized for query, navigation, reconciliation, visualization or automation. Yukh graphs and indexes are projections.

A projection may detect inconsistencies or suggest inferred relations. It is never the source of truth for a record or external concept.

### Runtime Realization

A Runtime Realization is an executing or deployable manifestation of one or more governed responsibilities. It may be a crate, process, service, edge runtime, embedded module or composed deployment.

Runtime form is not the identity of a Capability Record or Runtime Responsibility Record. Multiple realizations may conform to the same normative responsibility.

### Namespace Authority

A Namespace Authority owns canonical identifiers and governed meaning within its namespace. Initial authorities include:

- `uc-rust` for UC Rust normative records;
- `uc-bok` for UC-BoK concepts;
- `yukh` for Yukh-owned projection metadata only;
- `ebd` for Economics by Design records and models.

Indexing or resolving an identifier does not transfer authority.

## Allowed dependency direction

The intended flow is:

```text
Concept or external authority
          ↓ referenced by
Normative Record ← governed typed Relations
          ↓ constrained/implemented by
Artifact and Runtime Realization
          ↓ observed through
Evidence
          ↓ indexed or reconciled by
Projection
```

The arrows do not imply ownership transfer.

## Fundamental invariants

1. Every normative statement has exactly one authoritative namespace.
2. A record and its evidence are different entities.
3. An artifact cannot become normative merely by being committed to Git.
4. A projection cannot mutate authoritative meaning.
5. Runtime topology cannot redefine the identity of a capability or runtime responsibility.
6. Acceptance, implementation conformance, verification freshness and release inclusion are orthogonal dimensions.
7. Cross-namespace references preserve the target's canonical identifier.
8. Inferred relations are non-normative until accepted by the authority owning the relation source.
9. Unresolved external references remain explicit validation findings; they are not replaced with local copies.
10. The repository remains understandable and reviewable without Yukh or external graph infrastructure.

## Forbidden conflations

The model forbids treating:

- UC-BoK concepts as locally owned UC Rust records;
- records as containers for anonymous evidence blobs;
- implementation artifacts as proof of conformance;
- deployment success as lifecycle acceptance;
- Yukh projections as canonical architecture;
- file paths, titles or aliases as identity;
- runtime components as capability identity;
- inferred graph edges as accepted normative relations.

## CR-0001 example

`uc-rust:CR-0001` governs the Basket capability semantics and constraints. It may reference a Basket concept owned by UC-BoK. One or more artifacts and runtime realizations may implement it. Tests and observations provide evidence of conformance. Yukh may project the record, references and evidence but owns none of their normative meaning.

## RRR-0001 example

`uc-rust:RRR-0001` governs Operation Invocation as a reusable runtime responsibility independent of whether it is realized as a library, service, process or embedded runtime. Artifacts implement the responsibility; runtime instances execute it; evidence demonstrates behavior; projections make the trace queryable.

## Consequences for dependent work

- #63 defines lifecycle only for Normative Records and explicit review dispositions.
- #64 defines controlled Relations and their source/target invariants.
- #65 validates record envelopes, type-specific bodies, relations and identifiers; it does not require Yukh.
- #67 expands namespace authority and cross-repository resolution behavior.
- ADR-0022 remains the governing record taxonomy decision and must align with this meta-model before acceptance.

## Open acceptance points

1. Confirm whether Evidence requires its own canonical identifier in every case or only when referenced normatively.
2. Confirm whether Artifact identifiers are repository-local locators or canonical cross-repository identifiers.
3. Confirm the minimum metadata for Runtime Realization references without introducing a Deployment Record family.
4. Confirm whether the meta-model is accepted through a new ADR or by extending ADR-0022.
