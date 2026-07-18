# ADR-0023 — Record Lifecycle and Knowledge Validity

- Status: Proposed
- Date: 2026-07-18
- Governing issue: #57
- Parent epic: #55
- Related: ADR-0022

## Context

A single progression such as `Draft -> Accepted -> Implemented -> Verified` conflates governance decisions with delivery and evidence. An accepted architecture may not yet be implemented; a shipped implementation may deviate; verification may expire while the governing intent remains valid.

The knowledge model must preserve this distinction and support auditable changes, freshness checks, waivers and supersession across UC Rust, UC-BoK, Yukh and Economics by Design.

## Decision

UC Rust adopts a multi-axis record state model.

### Normative lifecycle

The only normative lifecycle states are:

- Draft;
- In Review;
- Accepted;
- Rejected;
- Deprecated;
- Superseded.

### Orthogonal dimensions

Implementation, conformance, verification, freshness, release inclusion and epistemic basis are represented independently.

No state in one dimension is inferred automatically from another. In particular:

- Accepted does not mean Implemented;
- Implemented does not mean Conforming;
- Shipped does not mean Verified;
- stale evidence does not automatically revoke an accepted decision.

### Transition events

All normative transitions are auditable events with actor, timestamp, rationale and supporting references. Automation may project evidence-derived dimensions but may not accept, reject, deprecate or supersede normative records.

### Freshness

Every accepted record declares a date-based policy, event triggers, or both. Evidence validity is scoped to implementation, environment and time. Expired or failed evidence changes verification/freshness projections and may block release gates without rewriting history.

### Supersession

Accepted records are never deleted. Replacement uses explicit directional relationships and a migration or compatibility disposition. Superseded records remain queryable as historical knowledge.

## Consequences

### Positive

- Governance, delivery and evidence become unambiguous.
- Yukh can compute stale, unverified or non-conforming records without becoming the source of truth.
- Release gates can evaluate real evidence rather than issue closure.
- UC-BoK knowledge remains historically traceable.

### Costs

- More fields and validation rules are required.
- Owners must maintain review triggers and evidence validity.
- GitHub Status cannot represent the full model and remains only a delivery projection.

## Rejected alternatives

- **One linear lifecycle through Verified:** semantically incorrect and difficult to regress safely.
- **Issue status as record status:** confuses work management with normative state.
- **Automatic revocation of accepted records when evidence expires:** destroys the distinction between intent and proof.
- **Mutable replacement in place:** loses historical identity and impact traceability.

## Required evidence before acceptance

- lifecycle schema and transition validator;
- canonical examples showing accepted/unimplemented, shipped/unverified and verified/stale states;
- waiver representation;
- supersession example;
- Yukh projection mapping;
- release gate rules consuming the independent dimensions.
