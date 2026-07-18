# SESSION-2026-07-18-07 — Complete quality model and identity architecture

- Date: 2026-07-18
- Governing epic: #10
- Related issues: #19, #28–#37

## Objective

Close the remaining project-definition gaps, make completeness measurable, and define authentication and Zanzibar-style authorization before implementation resumes.

## Actions completed

- Created issues #30–#35 for contract evolution, consistency, operability/DR, configuration, data governance and reproducible developer experience.
- Created issue #36 for OIDC authentication and Zanzibar-style authorization.
- Selected SpiceDB as the initial Zanzibar-compatible authorization adapter while keeping an application-owned provider-neutral authorization port.
- Created ADR-0013 documenting the identity and authorization decision.
- Created issue #37 and `.context/quality-attributes/system-quality-model.md` as the authoritative P0 completeness matrix.
- Updated `.context/manifest.yaml` to include the quality model in precedence, role loading and write policy.
- Updated `AGENTS.md` with reliability, performance, compatibility, consistency, operations, data, configuration and identity constraints.
- Updated `governance/github-manifest.json` to manage issues #28–#37 and remove the previous declarative-governance gap.

## Decision summary

Authentication is based on OAuth 2.0/OpenID Connect and remains identity-provider neutral. Authorization uses a Zanzibar-style relationship model. SpiceDB is the initial adapter because its explicit consistency levels, ZedTokens, gRPC API and Kubernetes Operator align with UC Rust requirements for causal safety, performance and operability.

## Evidence

- ADR-0013
- system quality model
- issues #30–#37
- updated AGENTS.md
- updated context and GitHub manifests

## Remaining work

The quality model currently records planned enforcement. Issue #37 must drive every P0 row to operational evidence before Project Ready #19 can be approved. Product charter and target architecture must define the concrete identity/resource hierarchy and initial performance/SLO budgets.
