# GitHub Agent Instructions

These instructions apply to every GitHub coding agent, Copilot agent, automation, and delegated implementation working in this repository. They complement the root `AGENTS.md` and do not weaken any architecture, TDD, coverage, documentation, context, or governance requirement.

## Mandatory pre-commit sequence

Before **every commit**, the agent must run formatting in write mode and then verify that the repository is cleanly formatted:

```bash
cargo fmt --all
cargo fmt --all -- --check
```

The agent must inspect the resulting diff before committing. It must not commit code that still produces a formatting diff, and it must not rely on CI to format code after the fact.

After formatting, run the applicable validation suite. For Rust workspace changes, the minimum is:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Runtime Foundation changes must additionally run the repository documentation checks, Rustdoc with warnings denied, architecture checks, semantic-conformance tests, and absolute coverage gates defined by the Runtime Foundation workflow and `docs/governance/test-driven-development-and-documentation.md`.

## Commit discipline

- Formatting is part of implementation, not a later cleanup step.
- A commit must contain the formatted form of every file it changes.
- The agent must not mix unrelated repository-wide formatting debt into a functional commit unless that cleanup is explicitly scoped and reviewed.
- When pre-existing formatting debt prevents a workspace-wide check from passing, the agent must still format all touched files, document the unrelated baseline failure, and open or reference a dedicated remediation issue.
- A failed formatter, Clippy, test, documentation, coverage, or architecture gate blocks the commit unless the commit is an explicitly identified TDD **red** commit whose expected failure is documented in the pull request.
- Even a TDD red commit must be formatted before it is created.

## Evidence

The pull request must state which validation commands were run and their outcomes. CI is independent verification; it is not a substitute for local pre-commit validation.