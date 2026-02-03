# Documentation style guide

This is a lightweight set of conventions to keep this repository’s documentation consistent.

## Status labeling

- `docs/specs/**` documents are **Normative** contracts and should explicitly say so near the top.
- `docs/adr/**` are decision records; their `Status` should reflect the decision lifecycle
  (`Proposed`, `Accepted`, `Rejected`, etc.).
- Other docs (guides, runbooks) are **Informative** by default.

## “Source of truth” rules

- If an ADR and a spec conflict, the spec should win (and ADRs should link to specs).
- For time-sensitive “current versions”, prefer pointing at the pointer files under
  `cli_manifests/codex/` instead of hardcoding version numbers in prose.

