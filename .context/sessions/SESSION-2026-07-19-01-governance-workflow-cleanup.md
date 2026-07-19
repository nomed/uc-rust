# SESSION-2026-07-19-01 — Governance workflow cleanup and project-token preflight hardening

- Agent or operator: Copilot Coding Agent
- Governing issue: #20
- Started: 2026-07-19T06:44:28Z
- Ended: 2026-07-19T07:30:00Z
- Branch or commit: copilot/investigate-cleanup-governance-workflow

## Intent

Fix the failing `sync-governance` workflow run and remove conflicting legacy governance logic while preserving manifest-driven governance behavior.

## Context reviewed

- `.github/workflows/sync-governance.yml`
- `scripts/sync_github_governance.py`
- `scripts/sync_github_structure.py`
- `scripts/sync_project_fields.py` (legacy)
- `.yukh/project.yaml`
- `governance/github-manifest.json`
- `governance/github-structure.json`
- `docs/governance/github-metadata.md`
- `.context/decisions/ADR-0020-yukh-shadow-governance-migration.md`
- GitHub Actions run `29676792001` job `88165542495`

## Actions

- Confirmed failure root cause from job logs: `gh project view ... --owner nomed` returned `unknown owner type` in preflight.
- Added a dedicated preflight script that reads manifest project owner/number and retries without `--owner` when GitHub CLI owner resolution is unsupported.
- Updated governance scripts to use owner-aware fallback consistently for all `gh project` operations.
- Removed duplicate legacy `sync_project_fields.py` path from workflow and deleted the script.
- Removed the temporary custom `Stage` field behavior and aligned field synchronization to native `Status` to match manifest/docs.
- Updated governance workflow script compilation and trigger paths.
- Updated governance documentation to clarify token requirements and owner-flag fallback behavior.

## Outcomes

- `sync-governance` preflight no longer assumes `--owner` always works for project access validation.
- Project synchronization no longer performs conflicting double writes (legacy `Stage` vs native `Status`).
- Workflow remains validate/dry-run/reconcile/apply compatible while using fewer overlapping governance steps.

## Evidence

- Modified files:
  - `.github/workflows/sync-governance.yml`
  - `scripts/sync_github_governance.py`
  - `scripts/sync_github_structure.py`
  - `scripts/preflight_project_token.py`
  - `docs/governance/github-metadata.md`
  - deleted `scripts/sync_project_fields.py`
- Validation commands:
  - `python3 scripts/sync_github_governance.py --mode validate`
  - `python3 -m py_compile scripts/sync_github_governance.py scripts/sync_github_structure.py scripts/preflight_project_token.py`
  - `python3 -m json.tool governance/github-structure.json >/dev/null`
  - `cargo test --workspace --all-features`

## Candidate decisions

- None.

## Failures and discarded approaches

- Discarded single-line workflow fix (`drop --owner`) because scripts still had hard owner assumptions and conflicted with native Status management.

## Open questions

- None.

## Next handoff

- None required.
