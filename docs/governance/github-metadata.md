# Declarative GitHub governance

GitHub repository metadata and Project v2 configuration are managed as code.

## Source of truth

`governance/github-manifest.json` is authoritative for:

- repository labels and their visual definitions;
- milestones;
- exact labels assigned to managed issues;
- issue-to-milestone assignments;
- membership of issues in `users/nomed/projects/4`;
- custom Project fields and single-select options;
- Project values for stage, priority, type, area, release and size.

Manual changes are considered drift and may be overwritten or deleted.

## Clean-state policy

Apply mode is intentionally convergent and destructive:

- labels not declared in the manifest are deleted;
- milestones not declared in the manifest are deleted;
- custom Project fields not declared in the manifest are deleted;
- Project items not represented in the manifest are deleted;
- each managed issue receives exactly the declared labels and milestone.

GitHub system Project fields such as `Title`, `Status`, `Labels`, `Milestone` and `Repository` are protected. Operational workflow state is stored in the managed custom field `Stage`, because GitHub's built-in `Status` options are not safely replaceable through the CLI.

## Taxonomy

Repository labels are intentionally limited to dimensions useful outside the Project:

- `priority:*`
- `type:*`
- `area:*`

The following are not labels:

- release: represented by repository milestone and Project `Release`;
- workflow status: represented by Project `Stage`;
- estimate: represented by Project `Size`.

This avoids duplicated and contradictory metadata.

## Workflow modes

The workflow `.github/workflows/sync-governance.yml` supports:

### Validate

Runs on pull requests. It validates the manifest without accessing or changing GitHub metadata.

### Dry-run

Runs after relevant changes reach `main`, and may also be started manually. It reports drift without changing GitHub.

### Apply

Runs only through manual dispatch. It requires:

1. selecting `apply`;
2. entering `DELETE_UNMANAGED_GITHUB_METADATA`;
3. approval of the `github-governance` environment, when protection rules are configured.

Apply performs the cleanup policy described above.

## Required secret

Create a repository Actions secret named `PROJECT_TOKEN`.

For a user-owned Project, use a dedicated personal access token authorized for:

- the `project` scope required by `gh project`;
- access to `nomed/uc-rust` issues and repository metadata.

The default `GITHUB_TOKEN` is used for repository labels, milestones and issue metadata. `PROJECT_TOKEN` is used only for Project v2 operations.

The token must not be stored in repository files, examples or logs.

## Recommended environment protection

Create an Actions environment named `github-governance` and require manual approval before deployment. The apply job uses this environment so destructive reconciliation cannot happen from an ordinary push.

## Change procedure

1. Change `governance/github-manifest.json` in a pull request.
2. Review the manifest diff as a governance change.
3. Merge only after validation passes.
4. Review the dry-run output on `main`.
5. Manually run apply with the required confirmation.
6. Review the Project and repository issue metadata.

## Adding a new issue

Creating an issue manually is allowed only as an initial capture step. Before it becomes part of the managed backlog, add its issue number and complete metadata to `governance/github-manifest.json`.

An unmanaged issue is removed from Project #4 during apply, but the issue itself is not deleted.

## Scope limitations

GitHub Project views and visual layouts are not currently managed by the synchronization script. Fields, options, values and items are managed. Views should be treated as presentation-only until GitHub exposes a stable automation interface suitable for this repository.
