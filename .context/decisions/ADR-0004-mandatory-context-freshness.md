# ADR-0004 — Mandatory context freshness and decision crystallization

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #21
- Decision owner: Human project owner

## Context

UC Rust is intended to be developed by humans and multiple agents over a long period. The project will fail operationally if implementation, architecture, process or schema choices advance without being consolidated into durable repository context. Session history and chat history are not sufficient because they are partial, tool-specific and may be unavailable to future contributors.

## Decision

Maintaining `.context/` is part of the work itself, not optional documentation after the work.

Every meaningful pull request or direct change must declare its context impact and do one of the following:

1. update the relevant durable context records;
2. add a session record and, when work continues elsewhere, a handoff;
3. promote a material decision into an ADR or RFC;
4. explicitly state `no context impact` with a concrete justification.

Material changes include, at minimum:

- architecture or module boundaries;
- domain concepts, invariants or ownership;
- public contracts and events;
- security or privacy behavior;
- release, packaging or deployment behavior;
- database schemas and migration policy;
- delivery workflow, governance or agent permissions;
- accepted assumptions, risks or non-goals.

A change is not complete when the code works but the authoritative memory is stale.

## Enforcement

- Pull-request templates must contain a context-impact section.
- CI must validate record naming, numbering, statuses and links.
- CI must require context changes for protected change categories unless a reviewed no-impact justification is present.
- Agents must write session records for meaningful work and handoffs for incomplete or delegated work.
- Reviewers must reject changes whose implementation and context disagree.
- The Project Ready gate cannot pass until these controls are active.

## Consequences

### Positive

- Decisions remain traceable and discoverable.
- New agents and contributors can reconstruct project intent.
- Temporary reasoning cannot silently become architecture.
- Architectural drift becomes visible during review.

### Negative

- Every meaningful change has a documentation cost.
- Small changes require classification to avoid unnecessary records.
- CI rules must avoid encouraging empty or low-value context updates.

## Alternatives rejected

- Rely on GitHub issues and PR descriptions only: they do not provide a stable, ordered decision system.
- Rely on chat/session memory: it is not repository-owned or reliably available.
- Update context periodically: this creates windows in which code and project memory disagree.
