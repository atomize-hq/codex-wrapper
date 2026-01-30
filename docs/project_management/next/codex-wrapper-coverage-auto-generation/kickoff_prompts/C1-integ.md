# Kickoff - C1-integ (CODEX_WRAPPER_COVERAGE_AUTO_GENERATION)

You are the **integration agent** for C1 (merge code+tests, reconcile to spec, run gates).

Scope source of truth: `docs/project_management/next/codex-wrapper-coverage-auto-generation/C1-spec.md`.

## Role boundaries (hard)
- You own reconciling implementation to spec and getting a clean, green result.
- Do not edit planning-pack docs (`docs/project_management/next/codex-wrapper-coverage-auto-generation/**`) from the worktree.
- Do not download binaries or run live Codex flows.

## Start checklist
1. `git checkout feat/codex-wrapper-coverage-auto-generation && git pull --ff-only`
2. Read: `docs/project_management/next/codex-wrapper-coverage-auto-generation/plan.md`, `docs/project_management/next/codex-wrapper-coverage-auto-generation/tasks.json`, `docs/project_management/next/codex-wrapper-coverage-auto-generation/session_log.md`, `docs/project_management/next/codex-wrapper-coverage-auto-generation/C1-spec.md`, this prompt.
3. Update `docs/project_management/next/codex-wrapper-coverage-auto-generation/tasks.json`: set `C1-integ` to `in_progress`.
4. Add START entry to `docs/project_management/next/codex-wrapper-coverage-auto-generation/session_log.md`; commit docs with `docs: start C1-integ`.
5. Create worktree: `git worktree add -b wcg-c1-scenario-catalog-integ wt/wcg-c1-scenario-catalog-integ feat/codex-wrapper-coverage-auto-generation`
6. Work only inside `wt/wcg-c1-scenario-catalog-integ` for integration changes.

## Integration requirements (C1)
- Merge branches:
  - `wcg-c1-scenario-catalog-code`
  - `wcg-c1-scenario-catalog-test`
- Reconcile to `C1-spec.md`.
- Refresh committed wrapper coverage artifact:
  - `SOURCE_DATE_EPOCH=0 cargo run -p xtask -- codex-wrapper-coverage --out cli_manifests/codex/wrapper_coverage.json`
- Run report + validate checks against the existing committed snapshots:
  - `VERSION="$(tr -d '\\n' < cli_manifests/codex/latest_validated.txt)"`
  - `SOURCE_DATE_EPOCH=0 cargo run -p xtask -- codex-report --version "$VERSION" --root cli_manifests/codex`
  - `SOURCE_DATE_EPOCH=0 cargo run -p xtask -- codex-validate --root cli_manifests/codex`

## Required commands (integration role)
- `cargo fmt`
- `cargo clippy --workspace --all-targets -- -D warnings`
- Relevant tests (at minimum): `cargo test -p xtask`
- Integration gate: `make preflight`

## End checklist
1. Merge `wcg-c1-scenario-catalog-code` and `wcg-c1-scenario-catalog-test` into `wt/wcg-c1-scenario-catalog-integ` and reconcile behavior to `C1-spec.md`.
2. Run required commands (capture outputs): `cargo fmt`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test -p xtask`; `make preflight`.
3. Commit integration changes on branch `wcg-c1-scenario-catalog-integ`.
4. Fast-forward merge `wcg-c1-scenario-catalog-integ` into `feat/codex-wrapper-coverage-auto-generation`; set `C1-integ` to `completed` in `docs/project_management/next/codex-wrapper-coverage-auto-generation/tasks.json`; add END entry to `docs/project_management/next/codex-wrapper-coverage-auto-generation/session_log.md`; commit docs with `docs: finish C1-integ`.
5. Remove worktree: `git worktree remove wt/wcg-c1-scenario-catalog-integ`.
