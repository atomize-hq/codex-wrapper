# Workstream K: CLI Snapshot + Diff Tooling

Objective: Generate and diff a structured inventory for a given `codex` binary so “what changed?” is automatic and reviewable.

Scope
- Define a deterministic snapshot format for Codex CLI help/flags/subcommands plus behavior-relevant metadata (version/channel/sha256).
- Build a local tool (prefer `xtask` or a small Rust CLI) that:
  - crawls `codex --help` recursively
  - captures raw help output for debugging
  - emits a structured JSON snapshot suitable for diffs
- Add a small supplement mechanism for known “not in `--help`” cases (explicit, reviewed, minimal).
- Provide a diff helper/report (human-readable) to guide wrapper updates from snapshot changes.

Constraints
- Deterministic output (stable ordering, normalized whitespace) so diffs are meaningful.
- No network calls in the snapshot tool; it operates on a local binary path.
- Linux-first; ensure the schema supports macOS/Windows without baking platform assumptions into the core format.

References
- ADR policy: `docs/adr/0001-codex-cli-parity-maintenance.md`.
- Static inventory (legacy): `CLI_MATRIX.md`, `capability_manifest.json`.
- Prior patterns: `workstreams/F_versioning_features/*` (probes/snapshots), `workstreams/I_cli_parity/*` (CLI surface).

Deliverables
- Snapshot schema + conventions documented under `cli_manifests/codex/README.md`.
- A runnable snapshot generator (tool) that writes `cli_manifests/codex/current.json` for a supplied binary.
- A diff report mode that highlights added/removed commands/flags and likely follow-up actions.

