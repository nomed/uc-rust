# ADR-0001 — Repository-owned agentic context system

- Status: accepted
- Date: 2026-07-18
- Deciders: project owner
- Governing issue: #14
- Supersedes: none
- Superseded by: none

## Context

UC Rust will be developed by humans and multiple coding agents across separate sessions. Chat history and agent-local memory are not durable enough to govern architecture, scope, permissions and delivery. The project needs an auditable source of truth that survives tool and model changes.

## Decision

We will maintain the durable agentic operating memory inside the repository under `.context/`.

Accepted decisions and RFCs are authoritative. Sessions and handoffs preserve continuity and evidence but cannot silently change architecture. The precedence and loading rules are defined by `.context/manifest.yaml`. Agents may draft decisions and RFCs, but material acceptance requires human approval.

## Consequences

The repository becomes portable across agent products and sessions. Decisions, evidence and unresolved work remain inspectable through Git history. Agents must spend time loading relevant context and recording meaningful work. Context drift must be checked in CI as the system matures.

## Alternatives considered

- Rely only on chat memory: rejected because it is not repository-scoped, portable or reliably auditable.
- Store all context in one large document: rejected because large documents become stale and create merge conflicts.
- Depend on a proprietary agent-memory service: rejected as the authoritative store, though such services may cache repository context.

## Evidence

- Issue #14 defines the memory lifecycle.
- `AGENTS.md` provides agent-facing instructions.
- `.context/manifest.yaml` defines precedence and role loading.

## Compliance

All agents and workflows must preserve accepted records and may not rewrite accepted history. Superseding decisions require a new numbered ADR.