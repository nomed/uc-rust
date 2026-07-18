# ADR-0023 — Record Lifecycle and Knowledge Validity

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #57
- Parent epic: #55
- Related: ADR-0022, #63
- Review readiness: Ready for accountable human acceptance

## Context

A single progression such as `Draft -> Accepted -> Implemented -> Verified` conflates governance decisions with delivery and evidence. An accepted architecture may not yet be implemented; a shipped implementation may deviate; verification may expire while the governing intent remains valid.

The knowledge model must preserve this distinction and support auditable changes, freshness checks, waivers and supersession across UC Rust, UC-BoK, Yukh and Economics by Design.

## Decision

UC Rust adopts a multi-axis record state model.

### Normative lifecycle

The normative lifecycle states are:

- Draft;
- Proposed;
- Accepted;
- Deprecated;
- Superseded;
- Rejected;
- Withdrawn.

The deterministic transition graph, authorities, amendment rules and terminal-state behavior are defined in `docs/architecture/record-lifecycle.md`.

### Orthogonal dimensions

Implementation conformance, verification freshness, evidence confidence, release inclusion and operational deployment are represented independently.

No state in one dimension is inferred automatically from another. In particular:

- Accepted does not mean Implemented;
- Implemented does not mean Conforming;
- Shipped does not mean Verified;
- stale evidence does not automatically revoke an accepted record;
- release inclusion does not imply acceptance or deployment.

### Transition events

All normative transitions are append-only, auditable events with source state, target state, actor, timestamp, rationale, content version and supporting references.

Automation may validate preconditions and project evidence-derived dimensions but may not accept, reject, deprecate or supersede normative records.

### Review and freshness

Every Accepted or Deprecated record declares an accountable review policy using one or more of:

- date-based review;
- release-gate review;
- event-triggered review;
- stable review with explicit triggers.

Freshness is projected as `current`, `review_due`, `stale` or `unknown` according to `docs/knowledge/review-and-waiver-policy.md`.

Freshness never mutates normative lifecycle automatically. A stale record may remain Accepted historically while being forbidden as release-readiness evidence.

### Waivers

A waiver is a temporary governed exception to a conformance, freshness or release-gate finding. It is not a lifecycle state and cannot alter record meaning.

Every waiver is scoped, risk-assessed, independently approved, linked to remediation and time-bounded. Permanent and self-approved waivers are forbidden. Expired or revoked waivers reopen the underlying finding.

### Supersession and deprecation

Accepted records are never deleted or silently rewritten. Replacement uses explicit directional relationships, lifecycle transition events and a migration or compatibility disposition.

Superseded records remain queryable as historical knowledge. Deprecation discourages new reliance but preserves the record until supersession or explicit historical retention.

### Ownership loss

Loss of accountable ownership immediately makes a record review-due. If ownership is not restored within the configured grace period, the record becomes stale and cannot satisfy release readiness.

## Consequences

### Positive

- Governance, delivery and evidence become unambiguous.
- Yukh can compute stale, unverified or non-conforming records without becoming source of truth.
- Release gates can evaluate real evidence rather than issue closure.
- UC-BoK knowledge remains historically traceable.
- Temporary exceptions become explicit, bounded and auditable.

### Costs

- More fields and validation rules are required.
- Owners must maintain review triggers and evidence validity.
- GitHub issue status cannot represent the full model and remains only a delivery projection.
- Release automation must understand stale records and waiver validity.

## Rejected alternatives

- **One linear lifecycle through Verified:** semantically incorrect and difficult to regress safely.
- **Issue status as record status:** confuses work management with normative state.
- **Automatic revocation of accepted records when evidence expires:** destroys the distinction between intent and proof.
- **Mutable replacement in place:** loses historical identity and impact traceability.
- **Permanent waivers:** convert exceptions into undocumented policy changes.
- **Automated acceptance by passing CI or absence of objection:** removes accountable human governance.

## Validation evidence produced

- `docs/architecture/record-lifecycle.md`;
- `docs/knowledge/review-and-waiver-policy.md`;
- common-envelope lifecycle and review metadata;
- schema and validator support from #65;
- canonical CR-0001 and RRR-0001 examples;
- typed supersession and waiver relation rules from #64.

## Acceptance readiness

The technical prerequisites for acceptance are complete:

- [x] deterministic lifecycle and transition rules;
- [x] explicit transition authorities;
- [x] lifecycle separated from implementation, verification and release dimensions;
- [x] freshness calculation and review triggers defined;
- [x] waiver policy defined;
- [x] deprecation and supersession handling defined;
- [x] machine-detectable invariants specified.

The remaining action is an explicit attributable human acceptance decision.