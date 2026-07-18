# UC Rust 1.0 Cross-Repository Authority Boundaries

- Status: Complete
- Governing issue: #67
- Depends on: #62
- Related: #56, #64, #65, ADR-0022

## Purpose

This document defines source-of-truth, namespace ownership, identifier resolution and projection responsibilities across UC-BoK, UC Rust, Yukh and Economics by Design (EbD). It prevents local convenience copies, indexes and derived graphs from silently becoming authoritative.

## Authority matrix

| System | Canonical authority | Owns | May project/reference | Must not own |
|---|---|---|---|---|
| UC-BoK | `uc-bok` namespace | Business/domain concepts, reference semantics, capability and domain model identifiers | UC Rust realizations, evidence and mappings | UC Rust implementation decisions, runtime responsibilities, conformance or release state |
| UC Rust | `uc-rust` namespace | ADR, RFC, CR, RRR, QAR, ER, SR, IR, DR and RR records; their lifecycle and normative outgoing relations | UC-BoK concepts, EbD models/records, Yukh projection locators, external standards | Canonical UC-BoK concept meaning, EbD economic model meaning, Yukh-derived graph truth |
| Yukh | `yukh` namespace | Projection metadata, index definitions, reconciliation results, inference provenance and projection-local identities | Any resolvable canonical entity | Canonical records, concepts, lifecycle decisions, accepted relations or evidence meaning |
| Economics by Design | `ebd` namespace | Economic concepts, economic units, cost models, measurement models and EbD-owned economic records | UC Rust implementation/evidence traces | UC Rust release decisions, architecture lifecycle or implementation conformance |

An entity has exactly one canonical authority. Caching, mirroring, indexing, importing or rendering does not transfer authority.

## Namespace contract

Canonical identifiers use:

```text
<namespace>:<local-id>
```

Baseline namespaces:

- `uc-rust` — normative records governed in this repository;
- `uc-bok` — concepts governed by UC-BoK;
- `ebd` — Economics by Design concepts, models and records;
- `yukh` — Yukh-owned projection metadata only.

Rules:

1. The namespace determines the authority responsible for canonical meaning.
2. The local identifier is immutable and never reused.
3. A title, path, URL, slug or projection node identifier is not a replacement for the canonical identifier.
4. A repository may retain a display label or cached summary, but it must also retain the canonical identifier and must mark cached text as non-authoritative.
5. Renames and path moves do not change canonical identity.
6. Alias resolution may redirect to a canonical identifier but may not create a second authority.
7. Namespace introduction or transfer requires an ADR and an explicit migration contract.

## Cross-repository reference object

A durable external reference contains at least:

```yaml
target: uc-bok:CAP-COMMERCE-BASKET
authority: uc-bok
resolution:
  locator: https://example.invalid/canonical/uc-bok/CAP-COMMERCE-BASKET
  revision: optional-immutable-revision
  observed_at: 2026-07-18
```

The normative relation remains the `type` + canonical `target` assertion in the source record. Resolution metadata supports retrieval and freshness; it does not become identity.

Minimum rules:

- `target` is mandatory;
- `authority` must equal the target namespace when present;
- mutable locators should include `observed_at` and, where available, immutable revision or digest;
- credentials, transport details and local cache paths are not part of canonical identity;
- unavailable resolution does not authorize copying the target into the local namespace.

## Resolution algorithm

A conforming resolver performs these steps deterministically:

1. Parse `<namespace>:<local-id>`.
2. Look up the namespace in `governance/namespace-authorities.yaml`.
3. Reject unknown namespaces as `unknown-authority`.
4. Select the registered resolver contract for that namespace.
5. Resolve the canonical identifier without rewriting it.
6. Verify returned identity equals the requested canonical identifier.
7. Capture revision/digest and observation time where supported.
8. Return one of the outcomes below.

Resolution outcomes:

| Outcome | Meaning | Normative behavior |
|---|---|---|
| `resolved` | Canonical entity was retrieved and identity matched | Reference may be evaluated normally |
| `unavailable` | Authority or network is temporarily unavailable | Preserve reference; emit explicit freshness/availability finding |
| `not_found` | Authority responded but identifier does not exist | Validation error for normative references |
| `identity_mismatch` | Returned entity claims another canonical ID | Validation error; never rewrite automatically |
| `unknown_authority` | Namespace is not registered | Validation error |
| `stale_cache` | Only a cache older than policy permits is available | Preserve as non-authoritative context; block transitions requiring current resolution |

No failure outcome permits creation of a local replacement identifier.

## Concept-to-record mapping

UC-BoK owns the concept; UC Rust owns its implementation-independent realization contract.

Canonical mapping:

```yaml
id: uc-rust:CR-0001
relations:
  - type: realizes_concept
    target: uc-bok:CAP-COMMERCE-BASKET
```

Rules:

- a CR may realize one or more UC-BoK concepts;
- multiple concepts require explicit `scope` where the realization is partial or composed;
- the CR may restate only the minimum contextual summary needed for review;
- local text cannot override the external concept;
- divergence is represented explicitly with `conflicts_with`, a scoped ADR or an unresolved finding, never by silently editing the concept copy;
- UC-BoK may derive an incoming `realized_by` projection, but UC Rust stores only the normative outgoing relation.

## Economics-to-record mapping

EbD owns generic economic concepts, unit definitions and reusable economic models. UC Rust owns project-specific architecture constraints and implementation/release decisions.

Use these patterns:

- `realizes_concept` when a UC Rust ER realizes an EbD-owned economic concept;
- `references` when an ADR, QAR, ER or RR uses an EbD model as context;
- `depends_on` when the UC Rust contract cannot be satisfied without a specific EbD normative model;
- `satisfies` from implementation-oriented records to a UC Rust ER that defines the project-specific economic constraint.

Example:

```yaml
id: uc-rust:ER-0001
relations:
  - type: realizes_concept
    target: ebd:ECONOMIC-UNIT
    scope: operation invocation cost unit
```

A local ER may instantiate or constrain an EbD model, but it must not claim ownership of the EbD model itself.

## Yukh projection boundary

Yukh may:

- index records and external concepts;
- resolve and cache canonical identifiers;
- derive inverse edges;
- infer candidate relations with method and confidence;
- calculate coverage, impact, freshness and inconsistency findings;
- present navigable graphs and query APIs.

Yukh must not:

- accept, reject, deprecate or supersede a record;
- write inferred edges back as normative relations without source-authority review;
- replace canonical IDs with projection-local node IDs;
- treat cache availability as canonical existence;
- resolve conflicts by mutating source records;
- become required to understand or validate the Git-native UC Rust record set.

A Yukh inference becomes normative only when the authority owning the relation source changes the source record through the normal lifecycle and review process.

## Local cache and snapshot policy

A repository may carry a snapshot for offline review only when the snapshot declares:

- canonical target identifier;
- source authority;
- retrieval timestamp;
- source revision/digest when available;
- cache format version;
- explicit `authoritative: false`.

Snapshots are replaceable artifacts. They cannot satisfy freshness-sensitive acceptance criteria after their declared expiry.

## Canonical examples

### CR-0001

`uc-rust:CR-0001` owns Basket realization semantics. Its `realizes_concept` target remains `uc-bok:CAP-COMMERCE-BASKET`. UC Rust may validate syntax and namespace registration locally; semantic resolution belongs to UC-BoK. A temporary UC-BoK outage produces `unavailable`, not a local Basket concept copy.

### RRR-0001

`uc-rust:RRR-0001` owns Operation Invocation runtime responsibility. It implements `uc-rust:CR-0001`; both source and target are governed by UC Rust. Yukh may derive the inverse `implemented_by` edge, but that inverse is not stored normatively.

### Economic trace

A future `uc-rust:ER-0001` may realize `ebd:ECONOMIC-UNIT` for operation invocation and constrain `uc-rust:RRR-0001`. EbD remains authoritative for the generic economic unit; UC Rust remains authoritative for the project-specific budget and its lifecycle.

## Machine-validatable invariants

The validator and namespace checker must detect:

1. malformed canonical identifiers;
2. unknown namespaces;
3. `authority` values inconsistent with target namespaces;
4. Yukh identifiers used as canonical record or concept identity;
5. local `uc-rust` copies claiming authority over `uc-bok` or `ebd` meaning;
6. cross-repository locators used without canonical targets;
7. resolution identity mismatch;
8. unresolved normative references being silently ignored;
9. projection confidence or inference metadata written as normative relation authority;
10. cache snapshots missing `authoritative: false`, observation time or source identity.

## Completion statement

The responsibility matrix, namespace ownership, resolver contract, concept/economics mappings, Yukh projection boundary, failure behavior and canonical examples are complete for UC Rust 1.0. Any authority transfer or new canonical namespace requires an ADR, registry update, migration plan and validator impact analysis.
