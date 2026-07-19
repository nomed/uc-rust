# Yukh adoption

UC Rust consumes the supported Yukh v0 release channel through `nomed/yukh@v0` with Project `nomed#4`.

The moving `v0` tag is the normal operational reference. For audit, incident analysis, or rollback, record and use the exact resolved release such as `v0.x.y` or its commit SHA.

Configuration:

- variable `YUKH_PROJECT_NUMBER=4`;
- secret `YUKH_PROJECT_TOKEN`;
- policy `.yukh/project.yaml`;
- workflow `.github/workflows/sync-governance.yml`.

Issue events run Yukh in dry-run mode. For apply, run **Sync GitHub governance**, select mode `yukh-apply`, enter the issue number, and confirm the mutation.

A governed issue begins with:

```markdown
<!-- yukh
schema: 1
kind: feature
area: platform
priority: P1
size: M
-->
```

After the first apply, repeat the same run and require zero applied operations, no remaining drift, and no diagnostics. Record the workflow links, exact resolved Yukh release, and any gaps in issue #69.
