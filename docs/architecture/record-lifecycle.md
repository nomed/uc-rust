# UC Rust 1.0 Normative Record Lifecycle

- Status: Draft
- Governing issue: #63
- Depends on: #62
- Related: #56, #64, #65, ADR-0022

## Purpose

This document defines the lifecycle state machine for UC Rust Normative Records. It deliberately separates normative lifecycle from implementation conformance, verification freshness, epistemic confidence and release inclusion.

## Lifecycle dimensions

A record has one normative lifecycle state. Other dimensions are tracked independently:

- implementation conformance;
- verification freshness;
- evidence confidence;
- release inclusion;
- operational deployment state.

None of those dimensions can implicitly change the normative lifecycle state.

## Baseline states

### Draft

The record is being authored and has no formal review expectation. Its identifier is reserved and cannot be reused.

### Proposed

The record is sufficiently complete for accountable review. The proposal is normative in intent but is not accepted.

### Accepted

The accountable authority has explicitly accepted the record. Acceptance requires an attributable human decision unless a future governance ADR defines a narrower delegated mechanism.

### Deprecated

The record remains valid for historical or compatibility purposes but should not be used for new work. Deprecation must identify a reason and, where applicable, a preferred successor.

### Superseded

The record has been replaced by one or more identified accepted records. Historical identity and content remain immutable.

### Rejected

The proposal was explicitly declined. The identifier remains reserved and the rejection rationale is retained.

### Withdrawn

The proposer withdrew the record before acceptance. The identifier remains reserved and the withdrawal rationale is retained.

## Initial transition graph

```text
Draft ───────────────→ Proposed
  │                       │  │
  └────────→ Withdrawn    │  ├────────→ Rejected
                          │  └────────→ Draft
                          ↓
                       Accepted
                          │  │
                          │  ├────────→ Deprecated
                          │  └────────→ Superseded
                          ↓
                    review/amendment
```

The amendment/reopening semantics for Accepted records are intentionally not finalized in this initial implementation. They must preserve accepted history and distinguish editorial corrections from normative change.

## Transition event requirements

Every lifecycle transition must record:

- source state;
- target state;
- transition timestamp;
- accountable actor;
- rationale;
- required evidence references;
- content version involved;
- previous transition reference when available.

A Git commit alone is not a lifecycle transition.

## Initial transition rules

| From | To | Minimum preconditions | Authority |
|---|---|---|---|
| Draft | Proposed | Required sections complete; owners identified; unresolved blockers explicit | Author or accountable owner |
| Draft | Withdrawn | Withdrawal rationale | Author or accountable owner |
| Proposed | Draft | Review found material incompleteness; rationale recorded | Author, owner or reviewer |
| Proposed | Accepted | Review complete; required evidence satisfied; explicit accountable approval | Authorized human approver |
| Proposed | Rejected | Explicit decision and rationale | Authorized human approver |
| Proposed | Withdrawn | Withdrawal rationale; no acceptance recorded | Author or accountable owner |
| Accepted | Deprecated | Explicit reason; impact and successor guidance where applicable | Authorized human approver |
| Accepted | Superseded | Identified accepted successor; replacement scope explicit | Authorized human approver |
| Deprecated | Superseded | Identified accepted successor; replacement scope explicit | Authorized human approver |

## Prohibited implicit transitions

The following must never change lifecycle state by themselves:

- merge or commit;
- passing CI;
- deployment success;
- accumulated evidence;
- lack of objections;
- elapsed review time;
- graph inference;
- release inclusion;
- implementation completion.

## Acceptance principle

Acceptance is an explicit attributable governance act. Silence, inactivity, automated validation or implementation progress cannot be interpreted as acceptance.

## Type-specific work still required

The following must be resolved before #63 is complete:

1. amendment and reopening semantics for Accepted records;
2. whether Rejected and Withdrawn are terminal or may return to Draft under the same identifier;
3. editorial correction policy after acceptance;
4. ADR- and RFC-specific deviations;
5. transition evidence requirements by record family;
6. machine-validatable transition invariants and canonical examples.
