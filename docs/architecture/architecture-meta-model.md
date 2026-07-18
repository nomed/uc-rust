# UC Rust 1.0 Architecture Meta-Model

- Status: Complete
- Governing issue: #62
- Parent epic: #55
- Related: #56, #63, #64, #65, #67, ADR-0022

## Purpose

This document defines the minimum conceptual model required to govern and implement UC Rust 1.0. It separates external knowledge, normative architecture, supporting evidence, implementation artifacts, derived projections and runtime realization so that no subsystem silently becomes authoritative for another.

This meta-model is intentionally Git-native. Yukh or another graph/indexing system may project it, but UC Rust records remain understandable, reviewable and authoritative in the repository.

## Entity model

| Entity | Semantic responsibility | Identity | Authority | Normative? |
|---|---|---|---|---|
| Concept | Externally governed business/domain meaning | Canonical external identifier | Owning knowledge namespace | According to source authority |
| Normative Record | Governed UC Rust intent, constraint, decision or accepted knowledge | `<namespace>:<local-id>` | UC Rust record namespace | Yes |
| Relation | Typed directional assertion between identified entities | Source record + relation type + target + optional discriminator | Authority of the relation source | Yes when declared by source authority |
| Evidence | Reproducibly identifiable observation supporting a claim | Canonical ID when referenced by a record; otherwise stable locator + digest | Evidence producer/custodian | No |
| Artifact | Produced implementation or operational object | Repository/package/registry locator plus immutable revision or digest | Artifact owner/registry | No |
| Projection | Derived query, graph, index, reconciliation or visualization representation | Projection-local identity | Projection system | No |
| Runtime Realization | Deployable or executing manifestation of governed responsibilities | Environment-scoped realization reference | Runtime/deployment authority | No |
| Namespace Authority | Owner of identifiers and governed meaning inside a namespace | Namespace identifier | Explicit repository or registry | Governs identity |

## Core entities

### Concept

A Concept is meaning owned by an authoritative knowledge namespace, such as UC-BoK. A concept may describe a business capability, domain term, principle or reference model.

A Concept is not automatically a UC Rust record. UC Rust references concepts and may govern a realization of them, but does not duplicate their external authority. A local explanatory summary must never replace or fork the canonical concept identifier.

### Normative Record

A Normative Record is a versioned, governed statement for which UC Rust is authoritative. The 1.0 families are ADR, RFC, CR, RRR, QAR, ER, SR, IR, DR and RR.

A record owns intent, constraints, decisions or accepted knowledge. It does not own code, tests, reports, deployments or observations used to demonstrate conformance.

The canonical record representation is Markdown with machine-validatable YAML front matter. Generated JSON, graph nodes and indexes are projections.

### Relation

A Relation is a typed, directional assertion connecting identified entities. It has:

- an authoritative source;
- a controlled relation type;
- an identified target;
- optional temporal validity;
- optional provenance and rationale;
- optional relation-local discriminator when more than one assertion of the same type and target is valid.

The authority that owns the relation source owns the normative assertion. A target authority does not automatically endorse an incoming relation.

A generic hyperlink is not a normative relation. Inferred relations are projections and cannot silently become normative.

### Evidence

Evidence is an immutable or reproducibly identifiable observation supporting a claim about a record, relation, artifact or runtime state. Examples include test results, benchmarks, traces, audits, deployments and operational observations.

Evidence does not change normative meaning. It can support review and lifecycle transitions but cannot perform them by itself.

Evidence identity policy:

- evidence referenced normatively by a record must have a canonical identifier in an evidence namespace;
- ephemeral or exploratory evidence may use a stable locator, immutable revision/digest and observed-at timestamp;
- mutable URLs without revision, digest or captured timestamp are insufficient for normative evidence references;
- evidence replacement creates a new evidence identity or revision; it does not overwrite historical claims.

### Artifact

An Artifact is a produced implementation or operational object, including source code, binaries, schemas, manifests, runbooks and reports.

Artifacts use canonical cross-repository identifiers when they are referenced by normative records. The minimum form is an authority-qualified locator plus immutable revision or digest. Repository-local paths alone are not stable cross-repository identity.

Artifacts may implement or support records. Their existence does not imply conformance, verification or release inclusion.

### Projection

A Projection is a derived representation optimized for query, navigation, reconciliation, visualization or automation. Yukh graphs and indexes are projections.

A projection may detect inconsistencies or suggest inferred relations. It is never the source of truth for a record, concept, evidence item or artifact. Rebuilding a projection must not change normative meaning.

### Runtime Realization

A Runtime Realization is an executing or deployable manifestation of one or more governed responsibilities. It may be a crate, process, service, edge runtime, embedded module or composed deployment.

Runtime form is not the identity of a Capability Record or Runtime Responsibility Record. Multiple realizations may conform to the same normative responsibility.

A runtime reference must contain at least:

- realization identifier within its runtime authority;
- environment or deployment scope;
- artifact revision/digest from which it was realized;
- observed or deployed timestamp;
- responsibility/capability references it claims to realize;
- optional instance identifier for transient observations.

No Deployment Record family is introduced for 1.0. Deployment manifests and observations remain Artifacts and Evidence unless a later ADR satisfies the taxonomy extension rule.

### Namespace Authority

A Namespace Authority owns canonical identifiers and governed meaning within its namespace. Initial authorities are:

- `uc-rust` for UC Rust normative records;
- `uc-bok` for UC-BoK concepts;
- `yukh` for Yukh-owned projection metadata only;
- `ebd` for Economics by Design records and models;
- explicit evidence, artifact and runtime authorities for their own identifiers.

Indexing, caching or resolving an identifier does not transfer authority.

## Allowed dependency direction

```text
Concept / external authority
          ↓ referenced by
Normative Record ← normative typed Relations
          ↓ constrained or implemented by
Artifact ───────────────→ Runtime Realization
   ↓                           ↓
   └──────── observed through Evidence
                               ↓
                    indexed/reconciled by Projection
```

The arrows do not imply ownership transfer.

### Dependency rules

| Source | May normatively reference | Must not claim |
|---|---|---|
| Concept | Source-defined entities | UC Rust implementation conformance |
| Normative Record | Concepts, records, evidence, artifacts and runtime references through controlled relations | Ownership of external concepts or derived projection state |
| Relation | Identified source and target entities allowed by the relation vocabulary | Meaning beyond its accepted relation type |
| Evidence | Subject observed and provenance | Acceptance or normative mutation |
| Artifact | Dependencies and build provenance | Automatic conformance |
| Runtime Realization | Artifact revision and governed responsibilities | Redefinition of record identity |
| Projection | Any resolvable entity for indexing/query | Canonical authority for projected content |

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
11. Normative relations are authoritative only from their declared source.
12. Normatively referenced evidence and artifacts must be reproducibly identifiable.
13. Mutable location, title, filename and display alias are never canonical identity.
14. Runtime observations cannot create, accept, deprecate or supersede a Normative Record.
15. A projection rebuild is semantically idempotent with respect to authoritative records.

## Forbidden conflations

The model forbids treating:

- UC-BoK concepts as locally owned UC Rust records;
- records as containers for anonymous evidence blobs;
- implementation artifacts as proof of conformance;
- deployment success as lifecycle acceptance;
- Yukh projections as canonical architecture;
- file paths, titles or aliases as identity;
- runtime components as capability identity;
- inferred graph edges as accepted normative relations;
- incoming references as endorsement by the target authority;
- mutable URLs as durable normative evidence.

## Mapping to the common envelope

The common envelope represents this meta-model as follows:

- `id` and `namespace` identify the Normative Record and its Namespace Authority;
- `type` identifies the record family;
- `status` and version fields govern record lifecycle and content evolution;
- `provenance` records authorship and derivation without transferring authority;
- `relations` contains normative typed Relation assertions owned by the record;
- `evidence` contains references to identified Evidence, never anonymous embedded proof;
- `supersedes`/`superseded_by` are lifecycle-aware relations, not file-history aliases;
- artifact and runtime references use authority-qualified immutable locators;
- projection-only metadata must not be written back as authoritative envelope content unless explicitly accepted.

## Canonical trace examples

### CR-0001 — Basket capability

`uc-rust:CR-0001` governs Basket capability semantics and constraints. It may reference `uc-bok:<basket-concept-id>` without copying UC-BoK authority. Interface, data, security and quality records may constrain it through controlled relations. One or more artifacts and runtime realizations may implement it. Tests and observations provide identified evidence of conformance. Yukh may project the complete trace but owns none of its normative meaning.

### RRR-0001 — Operation Invocation

`uc-rust:RRR-0001` governs Operation Invocation as a reusable runtime responsibility independent of whether it is realized as a library, service, process or embedded runtime. Artifacts implement the responsibility; runtime realizations execute it; evidence demonstrates observed behavior; projections make the trace queryable. Changing deployment topology does not change the identity or meaning of `uc-rust:RRR-0001`.

## Mapping to ADR-0022

ADR-0022 governs the record taxonomy, common envelope, evidence separation, identity model and canonical representation. This document is the normative elaboration of ADR-0022's architecture meta-model and resolves the entity boundaries required by issue #62.

ADR-0022 remains `Proposed` until the broader acceptance evidence listed in that ADR is complete. Completion of #62 does not bypass human acceptance and does not imply acceptance of ADR-0022.

## Consequences for dependent work

- #63 defines lifecycle only for Normative Records and explicit review dispositions.
- #64 defines controlled Relations and source/target invariants based on the entities in this document.
- #65 validates record envelopes, type-specific bodies, relations and identifiers without requiring Yukh.
- #67 expands cross-repository authority and resolution behavior without redefining these entities.

## Completion criteria

The meta-model is complete for 1.0 because:

- all required entities have one semantic responsibility and identity policy;
- evidence, artifact and runtime reference decisions are resolved;
- allowed dependencies and forbidden conflations are explicit;
- common-envelope and ADR-0022 mappings are defined;
- CR-0001 and RRR-0001 demonstrate the model;
- dependent lifecycle, relation and validator work can proceed without inventing new primitive entity types.

Any change to these primitives after architecture freeze requires an ADR and impact analysis across schemas, migration and projections.