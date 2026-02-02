# Kickoff Prompt – C0-code (Offline JSONL parsing API)

## Scope
- Production code only; no tests. Implement `docs/project_management/next/codex-jsonl-log-parser-api/C0-spec.md`.

## Start Checklist
1. `git checkout feat/codex-jsonl-log-parser-api && git pull --ff-only`
2. Read: `docs/project_management/next/codex-jsonl-log-parser-api/plan.md`, `docs/project_management/next/codex-jsonl-log-parser-api/tasks.json`, `docs/project_management/next/codex-jsonl-log-parser-api/session_log.md`, `docs/project_management/next/codex-jsonl-log-parser-api/C0-spec.md`, this prompt.
3. Set `C0-code` status to `in_progress` in `tasks.json` (orchestration branch only).
4. Add START entry to `session_log.md`; commit docs (`docs: start C0-code`).
5. Create the task branch and worktree: `git worktree add -b jp5-c0-jsonl-parser-api-code wt/jp5-c0-jsonl-parser-api-code feat/codex-jsonl-log-parser-api`.
6. Do **not** edit docs/tasks/session_log.md from the worktree.

## Requirements
- Implement the offline parsing API per the normative contract:
  - `docs/specs/codex-thread-event-jsonl-parser-contract.md`
  - `crates/codex/JSONL_COMPAT.md` (normalization semantics; must be shared with streaming)
- Do not add or modify any tests.
- Required commands (before handoff):
  - `cargo fmt`
  - `cargo clippy --workspace --all-targets -- -D warnings`

## End Checklist
1. Run the required commands above and capture their outputs.
2. Inside `wt/jp5-c0-jsonl-parser-api-code`, commit C0-code changes (no docs/tasks/session_log.md edits).
3. From outside the worktree, ensure the task branch contains the worktree commit (fast-forward if needed); do **not** merge into `feat/codex-jsonl-log-parser-api`.
4. Checkout `feat/codex-jsonl-log-parser-api`; update `tasks.json` to `completed`; add an END entry to `session_log.md` with commands/results/blockers; commit docs (`docs: finish C0-code`).
5. Remove worktree `wt/jp5-c0-jsonl-parser-api-code`.

