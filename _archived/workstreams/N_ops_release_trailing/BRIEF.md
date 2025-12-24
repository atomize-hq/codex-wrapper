# Workstream N: Ops Playbook (Release Trailing)

Objective: Make Codex CLI release trailing procedural, low-risk, and repeatable.

Scope
- Write a maintainer-facing playbook for trailing upstream Codex releases:
  - how to choose a candidate version
  - how to regenerate snapshots and review diffs
  - how to run the validation matrix
  - how to decide wrap vs intentionally unwrapped
  - how to ship a wrapper release (docs/examples version bumps, release notes)
- Define ownership and cadence (how quickly we trail; alerting expectations).
- Define “validated” vs “supported” vs “upstream seen” terminology.

Constraints
- Avoid adding operational complexity inside the core crate; keep automation in CI/workflows/docs.
- Linux-first; document macOS/Windows as phased follow-ups.

References
- ADR policy: `docs/adr/0001-codex-cli-parity-maintenance.md`.
- Existing workstreams: `workstreams/F_versioning_features/*`, `workstreams/I_cli_parity/*`, `workstreams/J_app_bundle/*`.

Deliverables
- A clear, step-by-step release trailing checklist and decision rubric.
- A “trial run” checklist for moving from 0.61.0 to 0.77.0 on Linux.

