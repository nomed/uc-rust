# ADR-0017 — Selective CI execution and useful failure evidence

- Status: Accepted
- Date: 2026-07-18
- Governing issues: #17, #28, #35, #37, #42

## Context

Always-on GitHub Actions were running on every pull request and every push to `main`, installing heavyweight Rust quality tools and executing full coverage even while the quality baseline was still being assembled. Runs were frequently failing without concise diagnostics, creating noise and consuming Actions capacity without producing accepted evidence.

## Decision

1. GitHub Actions quality execution is manual through `workflow_dispatch` while the Project Ready enforcement baseline is being stabilized.
2. The workflow exposes two suites:
   - `quick`: format, lint, tests, documentation and repository validation;
   - `full`: the quick suite plus complete line/branch coverage and formal evidence artifacts.
3. Contributors and agents run the same versioned commands locally through `just` before requesting an Actions run.
4. Full CI is requested only for milestone evidence, readiness review, release preparation, changes to enforcement infrastructure or investigation of environment-specific behavior.
5. Failed runs must publish readable step summaries and diagnostic log artifacts. A failing run without retrievable diagnostics cannot count as quality evidence.
6. Automatic pull-request execution may be reintroduced only after the quick suite is stable, fast and consistently diagnostic. Reintroduction requires an explicit change to this policy or a superseding ADR.
7. Expensive suites such as coverage, integration, partition, performance, soak and fleet rollout are never triggered by every documentation or governance-only commit.

## Consequences

- Repository commits no longer automatically consume Actions capacity.
- Enforcement remains versioned and reproducible locally.
- Formal evidence requires an intentional manual run until automatic triggers are approved again.
- Branch protection cannot depend on the manual workflow until a stable automatic quick gate is reintroduced.
