# Session 2026-07-18-15 — Durable sync restart and conflicts

## Governing issues

- `nomed/uc-rust#39`
- `nomed/uc-rust#42`

## Objective

Make edge outbox and central inbox state durable across process restart while preventing silent sequence collisions.

## Changes

- added explicit `AcceptResult::SequenceConflict`
- detect conflicts for both applied and buffered edge sequences
- added canonical string accessors for edge and event identifiers
- added `EdgeOutbox::from_pending` for persistence reconstruction
- added `uc-sync-sqlite`
- persist edge outbox events with unique event and per-edge sequence constraints
- persist central applied/buffered inbox state
- transactionally drain consecutive buffered events after the missing sequence arrives
- reload and verify outbox/inbox state after closing and reopening SQLite
- preserve original buffered event when a conflicting event reuses the same edge sequence
- updated architecture dependency enforcement for the sync adapter

## Guarantees implemented

- WAN backlog survives process restart
- acknowledgements survive restart
- exact event redelivery produces no duplicate business effect
- reordered events remain buffered durably
- arrival of the missing event applies the contiguous sequence
- same edge + same sequence + different event identity is an explicit conflict
- a conflict never silently overwrites the original event

## Verification status

The controls and tests exist in the repository but have not been observed passing on a clean runner in this session. The relevant quality rows remain `Implemented`, not `Evidenced`.

## Next work

- run one deliberate quick suite and repair compile/lint/test failures
- add a lightweight central/store-edge runtime harness around the durable store
- model connectivity state and reconnect delivery loop
- measure backlog recovery and expose retail-operational sync health
- remove the temporary verification branch/workflow after evidence collection
