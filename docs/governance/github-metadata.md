# Declarative GitHub governance

GitHub repository metadata and Project v2 configuration are managed as code.

## Source of truth

`governance/github-manifest.json` is authoritative for:

- repository labels and their visual definitions;
- milestones;
- exact labels assigned to managed issues;
- issue-to-milestone assignments;
- membership of issues in `users/nomed/projects/4`;
- managed Project fields and single-select options;
- Project values for status, priority, type, area, release and size.

`governance/github-structure.json` is authoritative for:

- parent and sub-issue relationships;
- issue dependencies;
- release date ranges used by roadmap fields.

Manual changes are considered drift and may be overwritten.

## Clean-state policy

The ordinary push reconciliation is additive and non-destructive. Destructive cleanup is reserved for explicit manual apply mode.

When destructive cleanup is enabled:

- labels not declared in the manifest are deleted;
- milestones not declared in the manifest are deleted;
- custom Project fields not declared in the manifest are deleted;
- Project items not represented in the manifest are deleted;
- each managed issue receives exactly the declared labels and milestone.

GitHub system Project fields such as `Title`, `Status`, `Labels`, `Milestone` and `Repository` are protected from deletion. Workflow state is written to GitHub's native `Status` field. The synchronizer requires the configured Status options to include `Backlog`, `Ready`, `In progress`, `In review`, `Blocked` and `Done`.

## Taxonomy

Repository labels are intentionally limited to dimensions useful outside the Project:

- `priority:*`;
- `type:*`;
- `area:*`.

The following are not duplicated as labels:

- release: represented by repository milestone and Project `Release`;
- workflow status: represented by native Project `Status`;
- estimate: represented by Project `Size`.

## Workflow modes

The workflow `.github/workflows/sync-governance.yml` supports:

### Validate

Validates manifests and compiles governance scripts without changing GitHub.

### Dry-run

Runs for pull requests or through manual dispatch and reports governance drift.

### Reconcile

Runs automatically after relevant changes reach `main`, and may also be started manually. Before changing anything it verifies that `PROJECT_TOKEN` exists and can read Project #4. It then reconciles:

- labels and milestones;
- issue labels and milestone assignments;
- Project membership;
- native Status and managed Project field values;
- parent/sub-issue relationships and dependencies;
- roadmap date fields.

The preflight is intentionally first: a missing or invalid Project token must not leave repository metadata updated while the Project remains stale.

Native `Status` values are applied on a best-effort basis: if a configured value does not exist in the current Project `Status` options, reconciliation logs a warning and continues while still enforcing strict option matching for manifest-managed custom fields.

### Apply

Runs only through manual dispatch. It requires:

1. selecting `apply`;
2. entering `DELETE_UNMANAGED_GITHUB_METADATA`;
3. approval of the `github-governance` environment, when protection rules are configured.

Apply may perform the destructive cleanup policy described above.

## Required secret

Create a repository Actions secret named `PROJECT_TOKEN`.

Use a dedicated token authorized for Project v2 and able to read/write the `project.number` configured in `governance/github-manifest.json`. The token must also be able to resolve issues from `nomed/uc-rust` when they are added to the Project.

The workflow preflight first validates access using the manifest owner/number pair, then retries with `--owner @me`, and finally retries without `--owner` when the CLI owner resolution behavior differs by token context.

The default `GITHUB_TOKEN` is used for repository labels, milestones, issue metadata and native issue relationships. `PROJECT_TOKEN` is used for Project v2 operations.

The token must not be stored in repository files, examples or logs.

## Recommended environment protection

Create an Actions environment named `github-governance` and require manual approval before destructive apply. Ordinary reconcile remains automatic and non-destructive.

## Change procedure

1. Change the manifests in a pull request.
2. Review the governance diff.
3. Merge only after validation passes.
4. Confirm the reconcile job succeeds on `main`.
5. Review Project fields, hierarchy and roadmap.
6. Use manual apply only when cleanup is intentionally required.

## Adding a new issue

Creating an issue manually is allowed as an initial capture step. Before it becomes part of the managed backlog, add its issue number and complete metadata to `governance/github-manifest.json` and its structural relationships to `governance/github-structure.json` when applicable.

## Scope limitations

Field values, Project items, roadmap dates and issue relationships are automated. GitHub Project visual view configuration remains best-effort because the public automation surface for saved views is less stable than item and field APIs. A Roadmap view may still require one manual creation or adjustment in the GitHub UI, while its underlying date fields remain governed.
