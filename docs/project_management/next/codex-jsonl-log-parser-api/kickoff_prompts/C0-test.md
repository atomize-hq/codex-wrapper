# Kickoff Prompt – C0-test (Offline JSONL parsing API)

## Scope
- Tests/fixtures only; no production code changes. Cover `docs/project_management/next/codex-jsonl-log-parser-api/C0-spec.md` and the scenario catalog.

## Start Checklist
1. `git checkout feat/codex-jsonl-log-parser-api && git pull --ff-only`
2. Read: `docs/project_management/next/codex-jsonl-log-parser-api/plan.md`, `docs/project_management/next/codex-jsonl-log-parser-api/tasks.json`, `docs/project_management/next/codex-jsonl-log-parser-api/session_log.md`, `docs/project_management/next/codex-jsonl-log-parser-api/C0-spec.md`, this prompt.
3. Set `C0-test` status to `in_progress` in `tasks.json` (orchestration branch only).
4. Add START entry to `session_log.md`; commit docs (`docs: start C0-test`).
5. Create the task branch and worktree: `git worktree add -b jp5-c0-jsonl-parser-api-test wt/jp5-c0-jsonl-parser-api-test feat/codex-jsonl-log-parser-api`.
6. Do **not** edit docs/tasks/session_log.md from the worktree.

## Requirements
- Implement tests that cover the scenario catalog:
  - `docs/specs/codex-thread-event-jsonl-parser-scenarios-v1.md`
- Tests MUST use the new public offline API (`codex::thread_event_jsonl_file` / `codex::thread_event_jsonl_reader` / `codex::JsonlThreadEventParser`) rather than reaching into internals.
- Create a dedicated integration test file at `crates/codex/tests/jsonl_parser_api.rs` (tests only).
- Required commands:
  - `cargo fmt`
  - `cargo test -p codex --test jsonl_compat -- --nocapture`
  - `cargo test -p codex --test jsonl_parser_api -- --nocapture`

## End Checklist
1. Run the required commands above and capture their outputs.
2. Inside `wt/jp5-c0-jsonl-parser-api-test`, commit C0-test changes (no docs/tasks/session_log.md edits).
3. From outside the worktree, ensure the task branch contains the worktree commit (fast-forward if needed); do **not** merge into `feat/codex-jsonl-log-parser-api`.
4. Checkout `feat/codex-jsonl-log-parser-api`; update `tasks.json` to `completed`; add an END entry to `session_log.md` with commands/results/blockers; commit docs (`docs: finish C0-test`).
5. Remove worktree `wt/jp5-c0-jsonl-parser-api-test`.

