# Kickoff Prompt – C0-integ (Offline JSONL parsing API)

## Scope
- Merge `C0-code` + `C0-test`, resolve drift against `C0-spec.md`, and ensure the triad is green. Integration owns aligning code/tests to the spec.

## Start Checklist
1. `git checkout feat/codex-jsonl-log-parser-api && git pull --ff-only`
2. Read: `docs/project_management/next/codex-jsonl-log-parser-api/plan.md`, `docs/project_management/next/codex-jsonl-log-parser-api/tasks.json`, `docs/project_management/next/codex-jsonl-log-parser-api/session_log.md`, `docs/project_management/next/codex-jsonl-log-parser-api/C0-spec.md`, this prompt.
3. Set `C0-integ` status to `in_progress` in `tasks.json` (orchestration branch only).
4. Add START entry to `session_log.md`; commit docs (`docs: start C0-integ`).
5. Create the integration branch and worktree: `git worktree add -b jp5-c0-jsonl-parser-api-integ wt/jp5-c0-jsonl-parser-api-integ feat/codex-jsonl-log-parser-api`.
6. Do **not** edit docs/tasks/session_log.md from the worktree.

## Requirements
- Merge the upstream code/test branches for C0, reconcile behavior to:
  - `docs/project_management/next/codex-jsonl-log-parser-api/C0-spec.md`
  - `docs/specs/codex-thread-event-jsonl-parser-contract.md`
  - `docs/specs/codex-thread-event-jsonl-parser-scenarios-v1.md`
  - `crates/codex/JSONL_COMPAT.md`
- Run required commands (capture outputs in END log):
  - `cargo fmt`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test -p codex --test jsonl_compat -- --nocapture`
  - `cargo test -p codex --test jsonl_parser_api -- --nocapture`
  - `make preflight`

## End Checklist
1. Merge the upstream C0 code/test branches into the integration worktree and reconcile behavior to the spec.
2. Run required commands (fmt/clippy/tests/preflight) and capture outputs.
3. Commit integration changes on branch `jp5-c0-jsonl-parser-api-integ`.
4. Fast-forward merge the integration branch into `feat/codex-jsonl-log-parser-api`; update `tasks.json` to `completed`; add an END entry to `session_log.md` with commands/results/blockers; commit docs (`docs: finish C0-integ`).
5. Remove worktree `wt/jp5-c0-jsonl-parser-api-integ`.

