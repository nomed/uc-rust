# Unified release and packaging policy

## Status

Proposed project baseline. This policy must be approved through the Project Ready gate before implementation work resumes.

## Decision

UC Rust uses **Release Please as the single release orchestrator** for every versioned deliverable produced by this repository.

A repository release is coordinated around one canonical semantic version:

```text
vMAJOR.MINOR.PATCH
```

The same version identifies:

- the Cargo workspace release;
- publishable Rust crates;
- application binaries;
- OCI container images;
- Helm charts;
- the GitHub Release;
- SBOM, provenance and signatures;
- release notes and changelog entries.

There must not be independent manually managed versions for Cargo packages, containers or Helm charts unless a future RFC explicitly introduces a multi-version release model.

## Release authority

Release Please owns:

- semantic-version calculation from Conventional Commits;
- release pull-request creation and maintenance;
- changelog generation;
- version updates in every declared package surface;
- Git tag and GitHub Release creation.

Publishing workflows run only after Release Please creates the GitHub Release. Publishing workflows never calculate or mutate the release version independently.

## Canonical version surfaces

The release pull request must update all applicable surfaces atomically:

- root Cargo workspace package version;
- publishable `Cargo.toml` package versions;
- internal Cargo dependency versions where required;
- `charts/uc-rust/Chart.yaml` `version`;
- `charts/uc-rust/Chart.yaml` `appVersion`;
- generated or maintained changelog files;
- release metadata manifest used by packaging workflows.

Container image tags are derived from the Git tag and are not stored as mutable source versions.

## Cargo packaging

The workspace follows a coordinated version policy:

- all releasable workspace crates use the repository release version;
- private implementation-only crates may remain `publish = false` but still carry the coordinated version;
- crates are packaged and verified with `cargo package` before publication;
- crates intended for a registry are published in dependency order;
- immutable registry artifacts are never overwritten;
- publication is skipped for private crates.

The initial registry decision is deferred to the Project Ready process. Supported targets may include crates.io or an approved private Cargo registry.

## Container packaging

Each deployable application produces an OCI image tagged with:

- `vMAJOR.MINOR.PATCH` as the immutable release tag;
- the full Git commit SHA;
- optionally `MAJOR.MINOR` and `MAJOR` moving aliases;
- optionally `latest` only for the default stable channel.

Published images must include OCI labels for version, revision, source and creation time.

Images must be:

- built from the released Git tag;
- reproducible as far as practicable;
- non-root at runtime;
- scanned before publication or promotion;
- accompanied by an SBOM;
- signed with keyless or approved signing infrastructure;
- published to the approved OCI registry.

No environment-specific configuration is baked into the image.

## Helm packaging

The primary chart lives under:

```text
charts/uc-rust/
```

For each coordinated release:

- chart `version` equals `MAJOR.MINOR.PATCH` without the `v` prefix;
- chart `appVersion` equals `MAJOR.MINOR.PATCH`;
- default image tag resolves to the same release version;
- the chart is linted and rendered before publication;
- the packaged chart is published as an OCI artifact;
- the chart artifact is signed or attested according to the supply-chain policy.

Chart and application versions therefore remain aligned for the initial product lifecycle.

## Release workflow

```text
Conventional commits merged to main
        ↓
Release Please opens or updates release PR
        ↓
Release PR updates all version surfaces and changelogs
        ↓
Required CI, packaging dry-runs and policy checks pass
        ↓
Human approval and merge of release PR
        ↓
Release Please creates tag and GitHub Release
        ↓
Publish workflow builds from the tag
        ↓
Cargo packages, OCI images and Helm charts are published
        ↓
SBOM, provenance, signatures and checksums are attached
        ↓
Post-publication verification completes
```

## Release Please configuration

The repository will contain:

```text
release-please-config.json
.release-please-manifest.json
```

The configuration must declare every file whose version is managed by the release process. Custom updaters may be used for files not natively supported by Release Please.

The release type will be selected to support a Rust workspace while preserving one coordinated repository version. The final configuration and any custom updater strategy must be tested in dry-run fixtures before enabling production publication.

## Conventional Commit policy

Release calculation follows Conventional Commits:

- `fix:` produces a patch candidate;
- `feat:` produces a minor candidate;
- `BREAKING CHANGE:` or `!` produces a major candidate;
- documentation, test, chore and refactor changes do not force a release unless configured or explicitly requested.

Pull-request titles or squash commit messages must satisfy this policy.

## Quality gates before release

A release cannot publish unless all applicable checks pass:

- formatting, linting and workspace tests;
- architecture and dependency policy;
- `cargo package` verification;
- container build and vulnerability scan;
- Helm lint and template validation;
- license and vulnerability checks;
- SBOM generation;
- artifact provenance generation;
- release consistency check across Cargo, chart and tag versions.

## Atomicity and failure handling

GitHub Release creation is the release trigger, but each artifact publication is independently observable.

If publication partially fails:

- immutable artifacts already published are not deleted or overwritten;
- the failed workflow is retried from the same tag;
- a release is not considered complete until post-publication verification passes;
- unrecoverable inconsistency requires a new patch release, never retagging an existing version.

## Environments and promotion

Build once, promote the same immutable artifacts.

Environment promotion must reference immutable image digests and chart versions. Staging and production do not rebuild application artifacts from different commits.

## Required follow-up decisions

Before enabling publication, the project must approve:

- Cargo registry;
- OCI registry and repository names;
- Helm OCI registry location;
- signing and provenance mechanism;
- whether moving image aliases such as `latest` are allowed;
- release approval roles;
- pre-release channels and version syntax;
- retention and rollback policies.
