# Codex CLI Manifests (`cli_manifests/codex`)

This directory is the home for **Codex CLI snapshot artifacts** used to maintain wrapper parity over time.

Source of truth for the policy is `docs/adr/0001-codex-cli-parity-maintenance.md`.

## Files

- `min_supported.txt` — minimum supported Codex CLI version (single semver line).
- `latest_validated.txt` — latest Codex CLI version that passed the validation matrix (single semver line).
- `current.json` — generated snapshot for `latest_validated.txt` (added by Workstream K tooling).

Optional/generated:

- `raw_help/<version>/**` — raw `--help` captures (for debugging help parser drift).
- `supplement/**` — small, explicit hand-maintained supplements for known “help omissions”.

## Conventions

- Keep JSON deterministic: stable sort order, avoid timestamps in fields that would churn diffs unnecessarily (use `collected_at` only).
- Treat `min_supported.txt` and `latest_validated.txt` as the only authoritative pointers.
- Avoid large “history dumps” unless we have a concrete need; prefer keeping only the current snapshot plus raw help for the validated version.

