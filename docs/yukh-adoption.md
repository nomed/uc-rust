# Yukh adoption

UC Rust uses `nomed/yukh@v0.2.2` with Project `nomed#4`.

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

After the first apply, repeat the same run and require zero applied operations, no remaining drift, and no diagnostics. Record the workflow links and any gaps in issue #69.
