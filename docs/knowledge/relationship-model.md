# Typed Relationship Model

- Status: Complete
- Governing issue: #58
- Governing architecture work: #64, #67

## Purpose

Relationships are semantic, directional architecture assertions rather than generic hyperlinks. Their authoritative vocabulary, source/target constraints, temporal semantics, provenance and inverse policy are defined in `docs/architecture/relationship-model.md`.

This document is the knowledge-foundation entry point and binds that controlled vocabulary to repository-level graph integrity and query requirements.

## Canonical relation authority

A relation is authoritative only when declared in the source record's canonical envelope. The source namespace owns the assertion. Incoming relations do not imply endorsement by the target authority.

Derived reverse edges, confidence scores, transitive closure and reconciliation findings are projections. Yukh may compute them but cannot silently write them back as normative relations.

## Controlled vocabulary

The UC Rust 1.0 vocabulary is:

- `references`
- `realizes_concept`
- `depends_on`
- `constrains`
- `implements`
- `exposes`
- `uses_data`
- `satisfies`
- `governed_by`
- `included_in`
- `supersedes`
- `deprecates`
- `conflicts_with`
- `waives`

Generic `related_to`, `link`, `association`, `see_also` and hand-authored inverse names are forbidden.

The definitive semantics and family matrix remain in `docs/architecture/relationship-model.md`; this file must not create a competing vocabulary.

## Integrity rules

Repository-level graph validation enforces:

- unique canonical identifiers;
- resolvable local `uc-rust:` targets;
- declared external namespace authorities;
- no self-relations or duplicate edges;
- acyclic `supersedes` relations;
- acyclic `depends_on` relations unless explicitly justified under the governed exception rule;
- no projection-only inverse relations in normative records;
- explicit findings for isolated or orphaned records;
- no inferred edge satisfying a required normative trace.

The complete rules, resolution classes and result contract are defined in `docs/knowledge/graph-views-and-integrity.md`.

## Required graph views

The model supports the following deterministic projections:

- decision impact;
- capability realization;
- runtime dependency;
- verification coverage;
- release composition;
- UC-BoK traceability;
- economic attribution;
- stale, unresolved and orphaned knowledge.

Each query identifies the authoritative nodes and declared edges used, distinguishes inferred edges, exposes incomplete external resolution and carries a validity timestamp.

## Canonical questions

- What does a decision affect?
- What realizes a capability?
- Which interface and data contracts support a runtime responsibility?
- Which evidence verifies a claim and is it still fresh?
- Which records enter a release?
- Which records are stale, isolated, unresolved or orphaned?

These questions are answered from controlled relations and evidence metadata rather than free-text interpretation.

## Tooling

- `scripts/validate_records.py` validates one record and its relation shapes.
- `scripts/validate_record_graph.py` validates cross-record identity, resolution, cycles and orphan/isolation rules.
- Yukh may add cross-repository projection and visualization but remains non-authoritative.

## Completion

Issue #58 is complete when the controlled model from #64, authority model from #67, graph integrity contract and repository-local graph validator are all in place. These artifacts now provide the required semantics without making a graph database mandatory.