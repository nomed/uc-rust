# RFC-0001 — UC Rust agentic operating system

- Status: draft
- Authors: project owner and orchestration agent
- Created: 2026-07-18
- Governing issue: #13
- Decision owner: project owner
- Target milestone: M0 — Project Ready

## Summary

Define a repository-governed multi-agent operating system for analysing, planning, implementing, verifying, securing, documenting and releasing UC Rust.

## Motivation

The project is expected to span many sessions and specialist tasks. Unstructured agents can duplicate work, alter architecture implicitly, lose decisions and perform writes outside their authority. We need explicit roles, context-loading rules, structured handoffs, permission boundaries, evidence requirements and human approval gates.

## Detailed design

The system will use an Orchestrator that selects specialist roles and controls task boundaries. Candidate specialists are Product and UC-BoK Analyst, Domain Architect, Rust Implementation Agent, Verification Agent, Security and Dependency Agent, Documentation and Knowledge Agent, and Release Agent.

Each role must define purpose, required context, allowed tools, writable paths, structured output, verification obligations and escalation conditions. Repository work is issue-driven. Material design changes require ADR or RFC records. Handoffs use `.context/templates/handoff.md`; sessions use `.context/templates/session.md`.

Orchestration may mix deterministic code-driven sequencing with model-directed delegation. Destructive actions, decision acceptance, scope changes, security exceptions and releases require explicit human authority.

## Agentic impact

This RFC establishes the agent model itself. Agents must be replaceable: authoritative state lives in Git, not inside one provider. `AGENTS.md` supplies local instructions, while `.context/manifest.yaml` supplies durable precedence and role-loading rules.

## Security and privacy

Agents receive least-privilege access. Secrets must not enter prompts, session records or commits. Production and destructive credentials are isolated behind protected environments and explicit approvals. Tool calls and handoffs must be auditable.

## Release and migration

The operating model will be introduced during M0. Existing work is treated as pre-model history and consolidated into initial ADRs and session records. Implementation issues remain blocked until the Project Ready gate approves the model.

## Drawbacks

The process introduces documentation and handoff overhead. Poorly designed records could become bureaucracy or stale context. Automation and periodic consolidation will be required.

## Alternatives

- One general-purpose autonomous agent: simpler but weak on authority, review and separation of concerns.
- Human-only orchestration without structured records: insufficient for repeatability across sessions.
- Provider-specific memory as source of truth: not portable or reliably auditable.

## Unresolved questions

- Exact role count and whether some roles should be merged.
- Which flows are code-orchestrated versus model-routed.
- How session and context drift validation should run in CI.
- Whether agent definitions should be YAML, Markdown or executable manifests.
- Required approval matrix by change category.

## Acceptance criteria

- Roles, ownership and escalation paths are unambiguous.
- Every role has required inputs and structured outputs.
- Permission boundaries follow least privilege.
- Handoffs preserve evidence and unresolved risks.
- Architecture and scope cannot be silently changed by an agent.
- The model supports both single-agent and multi-agent execution.
- Human approval points are explicit and testable.