# Session — Yukh shadow governance migration

Date: 2026-07-18

## Outcome

UC Rust will continue product implementation while governance-engine development moves to `nomed/yukh`.

## Operating model

- Stream A: Yukh implements UC Rust compatibility in read-only shadow mode, then sandbox reconciliation and migration gate.
- Stream B: UC Rust continues issue #44 for reproducible Economic by Design reports.
- Embedded governance scripts remain the apply baseline and are frozen against new generic features.

## Records

- ADR-0020 records the accepted migration strategy.
- UC Rust issue #20 is the migration gate.
- Yukh contains the compatibility roadmap and proving-ground contract.
