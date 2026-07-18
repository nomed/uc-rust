# Record Lifecycle

- Status: Draft
- Governing issue: to be assigned

## States

```text
Draft -> In Review -> Accepted -> Implemented -> Verified
                    \-> Rejected
Accepted/Implemented/Verified -> Deprecated -> Superseded
```

## Semantics

- **Draft**: incomplete proposal; not normative.
- **In Review**: review scope frozen except for review amendments.
- **Accepted**: normative project intent; implementation may not yet exist.
- **Implemented**: conforming implementation evidence exists.
- **Verified**: required acceptance evidence and quality gates pass.
- **Rejected**: reviewed and not adopted; retained for provenance.
- **Deprecated**: still valid for a bounded compatibility period but should not be used for new work.
- **Superseded**: replaced by an explicitly linked record.

## Transition rules

- Acceptance requires accountable owner and recorded review disposition.
- Implementation requires evidence links; a merged PR alone is insufficient when runtime evidence is required.
- Verification requires the record-specific acceptance criteria and evidence bundle.
- A record cannot return from Superseded to Accepted; a new version or new record is created.
- Material semantic changes after acceptance increment the version and trigger review.
- Broken or stale evidence may move a record from Verified back to Implemented, with an audit event explaining why.

## Orthogonal attributes

Lifecycle state is separate from:

- epistemic status: asserted, inferred, observed, measured;
- conformance: unknown, conforming, deviating, waived;
- freshness: current, review-due, stale;
- release inclusion: planned, committed, shipped, removed.

These dimensions must not be collapsed into one status field.

## Review due policy

Records declare either a review date or a review trigger such as major release, contract change, incident, cost regression, provider change or UC-BoK revision.