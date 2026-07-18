# Record Lifecycle and Knowledge Validity

- Status: Proposed
- Governing issue: #57
- Parent epic: #55
- Related: ADR-0022

## Principle

A record has one normative lifecycle, but implementation, conformance, verification, freshness and release inclusion are separate state dimensions. They must never be collapsed into one status field.

## Normative lifecycle

```text
Draft -> In Review -> Accepted
                 \-> Rejected
Accepted -> Deprecated -> Superseded
```

### States

- **Draft** — incomplete and non-normative.
- **In Review** — proposed content is under formal review; material amendments restart review.
- **Accepted** — normative project knowledge.
- **Rejected** — reviewed but not adopted; retained for provenance.
- **Deprecated** — still applicable only within an explicit compatibility or migration window.
- **Superseded** — replaced by one or more explicitly linked records.

`Implemented` and `Verified` are not lifecycle states. They describe evidence about realization of an accepted record.

## Orthogonal state dimensions

### Implementation

- `not_started`
- `in_progress`
- `implemented`
- `not_applicable`

### Conformance

- `unknown`
- `conforming`
- `partially_conforming`
- `deviating`
- `waived`

A waiver requires owner, rationale, scope, expiry or review trigger, and compensating controls where applicable.

### Verification

- `not_verified`
- `verification_pending`
- `verified`
- `verification_failed`
- `not_applicable`

### Freshness

- `current`
- `review_due`
- `stale`
- `unknown`

### Release inclusion

- `not_planned`
- `planned`
- `committed`
- `shipped`
- `removed`

### Epistemic basis

- `asserted`
- `inferred`
- `observed`
- `measured`

Epistemic basis describes how a claim is known, not whether the record is accepted.

## Transition authority

- Draft authors may move a record to `In Review` when required fields and reviewers are present.
- Accountable reviewers may accept or reject.
- Owners may propose deprecation; acceptance authority approves it.
- Supersession requires accepted replacement records and explicit typed links.
- Evidence automation may update implementation, verification and freshness projections, but may not accept or supersede normative knowledge.

Every transition records actor, timestamp, rationale and supporting references.

## Acceptance rules

Acceptance requires:

- stable identity and accountable owner;
- bounded scope and non-goals;
- required type-specific content;
- review disposition;
- unresolved objections recorded;
- required relations valid;
- review policy declared.

Acceptance does not require implementation.

## Freshness policy

Every accepted record declares at least one of:

- `review_due_at`;
- a maximum review interval;
- one or more event triggers.

Typical triggers include:

- major release boundary;
- public contract or schema change;
- production incident;
- security finding;
- cost or performance regression;
- provider or technology replacement;
- UC-BoK revision;
- changed legal, fiscal or compliance requirement;
- failed or expired verification evidence.

`review_due` means review is required but the record is not automatically invalid. `stale` means the project can no longer safely rely on the record without explicit review or waiver.

## Evidence invalidation

Evidence has its own validity period and applicability scope. When evidence expires, fails, or no longer matches the implementation/environment:

- lifecycle remains unchanged;
- verification may move from `verified` to `verification_pending` or `verification_failed`;
- freshness may become `review_due` or `stale`;
- release gates may fail.

History is append-only; previous verification is not erased.

## Material change policy

After acceptance:

- editorial corrections do not require re-acceptance;
- compatible clarifications increment the content version and require owner review;
- material semantic changes return the record to `In Review` or create a successor;
- changes that invalidate consumers require a new record or major content version with migration impact.

## Deprecation and supersession

Deprecation declares:

- reason;
- replacement or migration path, when available;
- compatibility window;
- affected releases and consumers;
- removal criteria.

Supersession requires directional links:

```text
old_record superseded_by new_record
new_record supersedes old_record
```

A record may be superseded by multiple records only when replacement responsibilities are explicitly partitioned.

## Invariants

- Rejected records cannot become Accepted without a new review event.
- Superseded records cannot return to Accepted.
- Accepted records cannot be deleted from history.
- `verified` requires valid evidence and acceptable conformance.
- `shipped` does not imply `verified`.
- `accepted` does not imply `implemented`.
- `deprecated` does not imply immediate runtime removal.
