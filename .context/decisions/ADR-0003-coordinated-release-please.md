# ADR-0003 — Coordinated monorepo releases with Release Please

- Status: accepted
- Date: 2026-07-18
- Deciders: project owner
- Governing issue: #17
- Supersedes: none
- Superseded by: none

## Context

UC Rust will produce Cargo packages, executable binaries, OCI container images, Helm charts, GitHub releases and supply-chain evidence. Independent manual versioning would create drift between deployable artifacts and make support and rollback ambiguous.

## Decision

We will use Release Please as the sole version authority and release orchestrator for the monorepo.

A coordinated SemVer version will identify the workspace release and all artifacts produced from the same immutable tag: Cargo packages, binaries, containers, Helm charts, changelog, SBOM, provenance and signatures. Agents and humans must not change release versions manually outside the Release Please release pull request.

## Consequences

Every supported artifact can be traced to one source tag and release record. Release automation must update all version-bearing files and verify that artifact publication is complete. Components cannot adopt independent release trains without a superseding RFC. Partial publication must be retried from the same immutable tag or corrected through a new release.

## Alternatives considered

- Independent versions per crate and artifact: rejected for the initial product because operational compatibility would be harder to understand.
- Manual release versions: rejected because they are error-prone and not reproducible.
- Container-only version authority: rejected because Cargo and Helm artifacts also require traceability.

## Evidence

- `release-please-config.json`
- `.release-please-manifest.json`
- `docs/governance/release-packaging.md`

## Compliance

Changes to versioning, registries, signing, artifact immutability or independent component releases require an accepted RFC.