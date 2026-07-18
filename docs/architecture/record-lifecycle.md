# UC Rust 1.0 Normative Record Lifecycle

- Status: Accepted design baseline
- Governing issue: #63
- Depends on: #62
- Related: #56, #64, #65, ADR-0022

## Purpose

This document defines the deterministic lifecycle state machine for every UC Rust 1.0 Normative Record. It separates normative governance from implementation conformance, verification freshness, epistemic confidence, release inclusion and deployment state.

The lifecycle answers only one question: **what is the current governance disposition of this record?**

## Orthogonal dimensions

A record has exactly one normative lifecycle state. The following dimensions are independent and cannot implicitly change that state:

- implementation conformance;
- verification freshness;
- evidence confidence;
- release inclusion;
- operational deployment state;
- projection/indexing state.

An Accepted record may be unimplemented, stale, excluded from a release or absent from any deployment. A deployed artifact may implement a Draft or Proposed record and is not thereby accepted.

## Lifecycle states

### Draft

The record is being authored. Its identifier is reserved and immutable, but the content has no formal review disposition.

### Proposed

The record is complete enough for accountable review. It expresses normative intent but is not yet authoritative.

### Accepted

The accountable authority has explicitly approved the record. Acceptance is attributable to at least one authorized human approver. Automated validation may be required evidence but cannot perform acceptance.

### Deprecated

The record remains historically valid or temporarily valid for compatibility, but must not be selected for new work. A deprecation rationale is mandatory. A successor is optional when no replacement exists.

### Superseded

The record has been replaced in whole or in an explicitly identified scope by one or more Accepted records. Its historical identity, accepted content and transition history remain immutable.

### Rejected

The Proposed record was explicitly declined. The identifier remains reserved and the rejection rationale is retained.

### Withdrawn

The author or accountable owner stopped the proposal before acceptance. The identifier remains reserved and the withdrawal rationale is retained.

## State classification

| State | Normative authority | Active for new work | Terminal under same version |
|---|---:|---:|---:|
| Draft | No | No | No |
| Proposed | No | No | No |
| Accepted | Yes | Yes | No |
| Deprecated | Yes, constrained | No | No |
| Superseded | Historical only | No | Yes |
| Rejected | No | No | Yes |
| Withdrawn | No | No | Yes |

Rejected, Withdrawn and Superseded are terminal for the same record version. The subject may be reconsidered only through a new record identifier, or through a new content version when the record family explicitly supports versioned amendment under the rules below.

## Canonical transition graph

```text
Draft ───────────────→ Proposed ───────────────→ Accepted
  │                       │  │                     │   │
  │                       │  ├────────→ Rejected  │   ├────────→ Deprecated
  │                       │  └────────→ Withdrawn │   └────────→ Superseded
  │                       └────────────→ Draft     │
  └──────────────────────────────→ Withdrawn       │
                                                   └────────→ Proposed
                                                        normative amendment only

Deprecated ───────────→ Superseded
Deprecated ───────────→ Proposed
                           normative amendment only
```

No other transitions are valid in the 1.0 baseline.

## Allowed transitions

| From | To | Transition name | Minimum preconditions | Authority |
|---|---|---|---|---|
| Draft | Proposed | propose | Required sections complete; owners identified; known blockers explicit | Author or accountable owner |
| Draft | Withdrawn | withdraw | Rationale recorded | Author or accountable owner |
| Proposed | Draft | return-for-rework | Material incompleteness or unresolved review concern recorded | Author, owner or reviewer |
| Proposed | Accepted | accept | Review complete; required evidence present; validation passes; explicit approval | Authorized human approver |
| Proposed | Rejected | reject | Explicit decision and rationale | Authorized human approver |
| Proposed | Withdrawn | withdraw | Rationale recorded; no acceptance event exists | Author or accountable owner |
| Accepted | Deprecated | deprecate | Reason, impact and migration guidance recorded | Authorized human approver |
| Accepted | Superseded | supersede | Accepted successor identified; replacement scope explicit | Authorized human approver |
| Accepted | Proposed | open-amendment | Normative change required; prior accepted snapshot preserved; new content version created | Accountable owner or authorized approver |
| Deprecated | Proposed | open-amendment | Normative revision may restore active use; prior accepted snapshot preserved | Accountable owner or authorized approver |
| Deprecated | Superseded | supersede | Accepted successor identified; replacement scope explicit | Authorized human approver |

## Forbidden transitions

The following transitions are invalid and must be rejected by tooling:

- Draft → Accepted;
- Draft → Rejected;
- Accepted → Draft;
- Accepted → Rejected;
- Accepted → Withdrawn;
- Deprecated → Accepted directly;
- Deprecated → Draft;
- Superseded → any state;
- Rejected → any state;
- Withdrawn → any state.

A rejected or withdrawn idea must not be revived by rewriting history. A new proposal receives a new identifier and may relate to the previous record through the controlled vocabulary defined by #64.

## Transition event model

Every lifecycle transition produces an append-only event with at least:

```yaml
transition:
  id: uc-rust:transition:<record-id>:<sequence>
  record: uc-rust:<record-id>
  from: Draft
  to: Proposed
  action: propose
  occurred_at: 2026-07-18T17:00:00Z
  actor: github:nomed
  authority_basis: accountable-owner
  rationale: "Ready for architecture review"
  content_version: 1.0.0
  content_digest: sha256:<digest>
  evidence: []
  previous_transition: null
```

Required invariants:

1. `from` equals the record state immediately before the event.
2. `to` is allowed by the canonical transition table.
3. `content_version` identifies the exact content under disposition.
4. `content_digest` binds the event to immutable content.
5. `actor` is attributable.
6. `authority_basis` is valid for the transition.
7. event sequence is strictly increasing per record.
8. transition history is append-only.
9. the current state equals the `to` state of the latest valid event.
10. a Git commit, merge, issue close or CI result is not itself a transition event.

## Authorization model

### Author

May edit Draft content, propose it, withdraw it before acceptance and respond to review. Authorship does not imply acceptance authority.

### Accountable owner

Owns the record's governance path. May propose, return for rework, withdraw before acceptance and open a normative amendment. The owner cannot self-accept unless repository governance explicitly grants that authority.

### Reviewer

May record findings and return a Proposed record to Draft. A reviewer cannot accept unless separately authorized as an approver.

### Authorized human approver

May accept, reject, deprecate or supersede. Acceptance must always include an explicit approval event. Agents and automation cannot hold this role in the 1.0 baseline.

### Agent or automation

May author drafts, run validation, assemble evidence, suggest transitions and create review requests. It may not accept, reject, deprecate or supersede records.

## Amendment and reopening semantics

### Editorial correction

An editorial correction changes no normative meaning. Examples include spelling, formatting, broken links, metadata normalization and clarification that cannot alter a conforming implementation or governance decision.

Rules:

- lifecycle state remains unchanged;
- content version increments at patch level;
- correction rationale and diff are recorded;
- the accepted semantic baseline remains the same;
- an authorized owner confirms the change is non-normative;
- tooling may require renewed digest and verification references.

### Normative amendment

A normative amendment changes intent, constraints, obligations, acceptance criteria, compatibility or externally observable semantics.

Rules:

1. preserve the previously accepted content snapshot and transition history;
2. increment the content version at minor or major level;
3. transition Accepted or Deprecated → Proposed using `open-amendment`;
4. perform a complete review of the changed normative scope;
5. require a new explicit acceptance event;
6. until re-accepted, the last Accepted version remains authoritative unless an approver explicitly suspends it through deprecation or supersession.

A record is never moved from Accepted to Draft. Normative amendment reopens it directly as Proposed because an accepted historical baseline already exists.

### New record instead of amendment

A new identifier is required when:

- the semantic responsibility changes;
- the change creates a separately deployable or governable concern;
- historical consumers must reference both old and new meanings;
- replacement scope cannot be expressed unambiguously within the existing identity;
- the original record is Rejected, Withdrawn or Superseded.

## Supersession semantics

Supersession is explicit and directional.

- the successor must already be Accepted;
- the superseding relation identifies full or partial replacement scope;
- partial supersession must name the unaffected scope retained by the old record;
- the superseded record is immutable except for editorial metadata and backlinks;
- implementation and release records may continue to reference it for historical compatibility;
- supersession does not delete evidence or transition history.

## Deprecation semantics

Deprecation signals that an Accepted record must not be selected for new work.

Required metadata:

- reason;
- effective date;
- impact;
- migration or containment guidance;
- successor, when one exists;
- known exceptions, if any.

Deprecation is reversible only through a normative amendment: Deprecated → Proposed → Accepted. Direct Deprecated → Accepted is forbidden.

## ADR and RFC rules

All record families use the same baseline lifecycle. ADR and RFC add the following constraints.

### ADR

- `Accepted` means the decision is currently authoritative.
- material changes require normative amendment or a new ADR that supersedes the old one.
- an ADR cannot be Withdrawn after acceptance.
- implementation failure does not automatically reject or deprecate the ADR.

### RFC

- an RFC may be Rejected without producing an ADR.
- an Accepted RFC remains a normative design specification when the repository explicitly uses RFCs that way.
- when an RFC exists only to solicit discussion and culminates in an ADR, the RFC must be Superseded by or linked to the resulting Accepted ADR according to #64.
- closure of the discussion thread does not imply acceptance.

CR, RRR, QAR, ER, SR, IR, DR and RR require no lifecycle deviations in the 1.0 baseline.

## Evidence requirements by transition class

| Transition class | Required evidence |
|---|---|
| propose | structural completeness and schema validation |
| accept | completed review, resolved blockers, required type-specific evidence, explicit approval |
| reject | decision rationale and material review findings |
| withdraw | attributable owner rationale |
| deprecate | impact assessment and migration/containment guidance |
| supersede | accepted successor and replacement scope |
| open-amendment | change rationale, prior accepted snapshot and semantic diff |

Record-family schemas may require additional evidence but cannot weaken these minimums.

## Canonical examples

### CR-0001 initial acceptance

```text
Draft --propose--> Proposed --accept--> Accepted
```

Passing capability tests before the `accept` event changes conformance evidence only. It does not change lifecycle state.

### RRR-0001 normative amendment

```text
Accepted(v1.0.0) --open-amendment--> Proposed(v1.1.0) --accept--> Accepted(v1.1.0)
```

The v1.0.0 accepted snapshot and approval remain immutable and queryable.

### ADR replacement

```text
ADR-0010 Accepted
ADR-0025 Draft --propose--> Proposed --accept--> Accepted
ADR-0010 Accepted --supersede by ADR-0025--> Superseded
```

The old ADR cannot be marked Superseded before the successor is Accepted.

### Withdrawn RFC reconsidered

```text
RFC-0007 Proposed --withdraw--> Withdrawn
RFC-0012 Draft --propose--> Proposed
```

RFC-0007 is not reopened. RFC-0012 may reference it as prior exploration.

## Machine-validatable rules for #65

The validator must detect at least:

1. unknown lifecycle states;
2. transitions absent from the allowed table;
3. missing transition actor, rationale, timestamp, version or digest;
4. acceptance without an authorized human approver;
5. non-monotonic transition sequence;
6. mismatch between current state and latest transition;
7. mutation of a terminal record's normative content;
8. supersession without an Accepted successor;
9. direct Deprecated → Accepted transitions;
10. accepted normative changes recorded as editorial corrections;
11. reused identifiers after Rejected, Withdrawn or Superseded disposition;
12. implicit acceptance inferred from CI, merge, elapsed time or issue state.

## Governance invariants

1. Lifecycle state is singular and deterministic.
2. Acceptance is explicit, attributable and human.
3. Silence and absence of objection never imply acceptance.
4. Accepted history is immutable.
5. Normative amendment creates a new review disposition without erasing the prior Accepted version.
6. Terminal records never return to active states under the same version.
7. Evidence informs transitions but does not perform them.
8. Implementation, verification, release and deployment state remain orthogonal.
9. Every transition is auditable without Yukh or external graph infrastructure.
10. Invalid transitions are mechanically detectable from repository data.

## Completion

This lifecycle applies to every UC Rust 1.0 Normative Record family and provides the lifecycle primitives required by #64 and #65. Record-family-specific schemas may add stricter preconditions, but they may not introduce new states or weaken these invariants without a governing ADR.