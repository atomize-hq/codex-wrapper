# Workstream L: CI Validation Matrix (min vs latest)

Objective: Make “latest validated” real-binary smoke checks repeatable and gating on Linux, then expand to macOS/Windows.

Scope
- Define the validation matrix: `min_supported.txt` vs `latest_validated.txt`.
- Add CI jobs to run:
  - `cargo test -p codex --lib`
  - `cargo test -p codex --examples`
  - `cargo test -p codex --test cli_e2e` against pinned binaries
- Add GitHub automation:
  - nightly “Release Watch” workflow (alerts only)
  - maintainer-triggered “Update Snapshot” workflow (download artifacts, record checksums, regenerate snapshots, open PR, run CI)

Constraints
- Avoid supply-chain/network behavior in the core crate; downloading binaries is a CI/workflow concern only.
- Prefer reproducibility: pin by version + checksum; avoid “latest” URLs.
- Linux-first; macOS/Windows come after the Linux matrix is stable.

References
- ADR policy: `docs/adr/0001-codex-cli-parity-maintenance.md`.
- Existing harness: `crates/codex/tests/cli_e2e.rs`.
- Version pointers: `cli_manifests/codex/min_supported.txt`, `cli_manifests/codex/latest_validated.txt`.

Deliverables
- CI jobs that validate the wrapper against pinned binaries (gating).
- Workflows for “Release Watch” and “Update Snapshot”.

