# Contributing

## Repo map (where things live)

- Wrapper library: `crates/codex/`
  - Main guide: `crates/codex/README.md`
  - Examples index: `crates/codex/EXAMPLES.md`
  - Normative JSONL normalization notes: `crates/codex/JSONL_COMPAT.md`
- Decisions/specs:
  - ADRs: `docs/adr/`
  - Normative contracts: `docs/specs/`
  - Docs index: `docs/README.md`
- CLI parity artifacts + ops docs: `cli_manifests/codex/`
- Triad planning/process: `docs/project_management/`
  - Feature directories: `docs/project_management/next/`

## Development

### Requirements

- Rust toolchain (stable)
- `make` (optional, but recommended for the project’s preflight gate)

### Common commands

- Format: `make fmt`
- Lint: `make clippy`
- Test: `make test`
- Preflight (integration gate): `make preflight`

## Repository hygiene rules

This repo intentionally does not commit:

- Worktrees: `wt/`
- Build output: `target/`
- Download/extract scratch: `_download/`, `_extract/`
- Raw help captures: `cli_manifests/codex/raw_help/`
- Ad-hoc logs at repo root (for example `codex-stream.log`, `error.log`)

`make preflight` runs a repo hygiene check to prevent accidentally committing these artifacts.

## Triads/worktrees (project management)

Feature work is planned as triads (code / test / integration) with checklists and prompts under the
feature directory in `docs/project_management/next/<feature>/`.

Conventions:
- Task worktrees live under `wt/<branch>` (in-repo).
- Do not edit `docs/project_management/**` from inside a worktree.

See `docs/project_management/task-triads-feature-setup-standard.md`.

## Branch promotion (staging -> main)

This repo uses a promotion flow where changes land in `staging` first, then are promoted to `main`.
PRs targeting `main` are restricted and must originate from `staging`.

When opening a `staging` -> `main` PR, you must apply **exactly one** purpose label:

- `purpose=codex_release`
- `purpose=claude_code_release`
- `purpose=ops`

This is enforced by the GitHub Actions workflow:
- `.github/workflows/only-staging-to-main.yml`
