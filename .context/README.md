# UC Rust Context System

`.context/` is the durable operating memory for humans and agents working on UC Rust.

## Source precedence

When sources conflict, apply this order:

1. explicit human instruction in the current task;
2. accepted decisions in `.context/decisions/`;
3. accepted RFCs in `.context/rfcs/`;
4. `PROJECT_CHARTER.md` and target architecture documents;
5. root and nearest `AGENTS.md`;
6. current issue and pull request;
7. latest accepted handoff;
8. session records;
9. temporary notes.

A session record or handoff never changes architecture by itself. Material decisions must be promoted into a decision record or RFC.

## Structure

- `manifest.yaml`: machine-readable context index and loading rules.
- `decisions/`: immutable decision records. Superseded records are retained.
- `rfcs/`: proposals for substantial or costly changes.
- `handoffs/`: structured transfer of work between agents or sessions.
- `sessions/`: chronological work records and evidence, not authoritative decisions.
- `templates/`: canonical templates for all record types.

## Required agent workflow

Before acting, an agent must:

1. read `.context/manifest.yaml`;
2. read the root and nearest `AGENTS.md`;
3. load records required for its role;
4. identify the governing issue and accepted decisions;
5. declare unresolved conflicts rather than guessing.

At the end of meaningful work, the agent must:

1. write or update a session record;
2. create a handoff when another agent or session must continue;
3. propose an ADR/RFC when a material decision was discovered;
4. list evidence, changed files, tests and unresolved risks.

## Record lifecycle

- **Proposed**: under discussion and not authoritative.
- **Accepted**: binding until superseded.
- **Rejected**: retained with rationale.
- **Superseded**: replaced by another numbered record.
- **Deprecated**: still present but should no longer guide new work.

Numbers are monotonic and never reused.