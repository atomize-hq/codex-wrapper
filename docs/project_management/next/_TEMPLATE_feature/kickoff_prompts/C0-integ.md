# Kickoff Prompt – C0-integ (<TRIAD TITLE>)

## Scope
- Merge `C0-code` + `C0-test`, resolve drift against `C0-spec.md`, and ensure the triad is green. Integration owns aligning code/tests to the spec.

## Start Checklist
1. `git checkout feat/<feature> && git pull --ff-only`
2. Read: `plan.md`, `tasks.json`, `session_log.md`, `C0-spec.md`, this prompt.
3. Set `C0-integ` status to `in_progress` in `tasks.json` (orchestration branch only).
4. Add START entry to `session_log.md`; commit docs (`docs: start C0-integ`).
5. Create the integration branch and worktree: `git worktree add -b <feature-prefix>-c0-<scope>-integ wt/<feature-prefix>-c0-<scope>-integ feat/<feature>`.
6. Do **not** edit docs/tasks/session_log.md from the worktree.

## Requirements
- Merge the upstream code/test branches for C0, reconcile behavior to `C0-spec.md`.
- Run required commands (capture outputs in END log):
  - `cargo fmt`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - Relevant tests (at minimum, the suites introduced by C0-test)
  - Integration gate: `make preflight`

## End Checklist
1. Merge the upstream C0 code/test branches into the integration worktree and reconcile behavior to the spec.
2. Run required commands (fmt/clippy/tests/integration gate) and capture outputs.
3. Commit integration changes on branch `<feature-prefix>-c0-<scope>-integ`.
4. Fast-forward merge the integration branch into `feat/<feature>`; update `tasks.json` to `completed`; add an END entry to `session_log.md` with commands/results/blockers; commit docs (`docs: finish C0-integ`).
5. Remove worktree `wt/<feature-prefix>-c0-<scope>-integ`.
