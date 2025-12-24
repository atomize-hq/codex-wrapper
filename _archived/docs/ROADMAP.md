# Codex Wrapper Roadmap: Release-Trailing + Parity Maintenance

This roadmap turns `docs/adr/0001-codex-cli-parity-maintenance.md` into an implementable maintenance loop.

## Workstreams (new)

- **Workstream K: CLI Snapshot + Diff Tooling** — generate deterministic snapshots for any `codex` binary and diff them to drive wrapper updates.
- **Workstream L: CI Validation Matrix (min vs latest)** — run smoke/E2E tests against pinned “min supported” and “latest validated” binaries in CI, with automation to update snapshots.
- **Workstream M: JSONL & Notification Schema Compatibility** — keep streaming/app-server/MCP parsing robust across schema drift using normalization rules + fixtures + tests.
- **Workstream N: Ops Playbook (Release Trailing)** — define the procedural checklist and ownership model for trailing upstream Codex releases safely.

## Milestones (“done” checkpoints)

- **M1 (K1/K2):** Snapshot schema + generator produces `cli_manifests/codex/current.json` deterministically for Linux.
- **M2 (L1/L2/L3):** CI runs real-binary smoke tests against `min_supported.txt` and `latest_validated.txt` on Linux.
- **M3 (M1/M2):** Known JSONL/notification drift cases are normalized and covered by fixtures + unit tests.
- **M4 (N1/N2):** Maintainers have an actionable release-trailing playbook (update pointers, regenerate snapshot, run matrix, ship).

## Trial run plan (0.61.0 → 0.77.0 on Linux)

1) Generate snapshots for `0.61.0` (repo-pinned) and `0.77.0` (`codex` on PATH).
2) Review diffs to identify new commands/flags (e.g., `review`) and decide wrap vs explicitly unwrapped.
3) Run the validation matrix against both versions; fix wrapper incompatibilities behind capability guards or normalization.
4) Update `cli_manifests/codex/latest_validated.txt` to the newest passing version and regenerate `current.json`.

