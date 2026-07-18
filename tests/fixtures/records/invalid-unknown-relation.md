---
id: uc-rust:CR-9001
type: CR
schema_version: 1
content_version: 0.1.0
title: Invalid relation fixture
summary: Deliberately invalid record used to verify diagnostics.
status: Proposed
owners: [{role: domain-architecture}]
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Validator fixture only.
provenance:
  - {kind: issue, locator: https://github.com/nomed/uc-rust/issues/65}
relations:
  - {type: related_to, target: uc-rust:RRR-0001}
review:
  required_roles: [domain-architecture]
  reviewers: []
  disposition: pending
---

# Invalid relation fixture

## Purpose
Exercise invalid relation diagnostics.

## Invariants
The fixture must fail validation.

## Authority
This file has no normative authority.
