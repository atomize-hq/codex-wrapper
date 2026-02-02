# Codex JSONL Log Parser API (ADR 0005) – Plan

## Purpose
Ship the ADR 0005 offline JSONL parsing API in `crates/codex` so host applications can rehydrate a
saved `--json` JSONL log file into the same typed `ThreadEvent` stream produced during live
streaming, without spawning Codex.

This is Codex-specific, but the API shape is intended to be replicated for future CLI agent wrapper
crates in this repo (Claude Code, Gemini CLI, etc.).

## Guardrails
- Triads only: code / test / integration. No mixed roles.
- Code: production code only; no tests. Required commands: `cargo fmt`; `cargo clippy --workspace --all-targets -- -D warnings`.
- Test: tests/fixtures/harnesses only; no production logic. Required commands: `cargo fmt` + the targeted `cargo test ...` commands specified by the kickoff prompt.
- Integration: merges code+tests, reconciles to spec, and must run `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, relevant tests, and `make preflight`.
- Docs/tasks/session_log edits happen only on the orchestration branch (`feat/codex-jsonl-log-parser-api`), never from worktrees.

## Branch & Worktree Conventions
- Orchestration branch: `feat/codex-jsonl-log-parser-api`.
- Feature prefix: `jp5`.
- Branch naming: use the exact branch names specified in `tasks.json` (do not invent new names).
- Worktrees: `wt/<branch>` (in-repo; ignored by git).

## Triad Overview
- **C0 – Offline JSONL parsing API:** Add the public `codex::jsonl` module + crate-root reexports, refactor normalization for reuse, add fixture-backed tests, and validate with the integration gate.

## Start Checklist (all tasks)
1. `git checkout feat/codex-jsonl-log-parser-api && git pull --ff-only`
2. Read: this plan, `tasks.json`, `session_log.md`, the relevant spec, and your kickoff prompt.
3. Set the task status to `in_progress` in `tasks.json` (orchestration branch only).
4. Add a START entry to `session_log.md`; commit docs (`docs: start <task-id>`).
5. Create the task branch and worktree from `feat/codex-jsonl-log-parser-api`: `git worktree add -b <branch> wt/<branch> feat/codex-jsonl-log-parser-api`.
6. Do **not** edit docs/tasks/session_log from the worktree.

## End Checklist (code/test)
1. Run required commands (code: fmt + clippy; test: fmt + targeted tests) and capture outputs.
2. From inside the worktree, commit task branch changes (no docs/tasks/session_log edits).
3. From outside the worktree, ensure the task branch contains the worktree commit (fast-forward if needed). Do **not** merge into `feat/codex-jsonl-log-parser-api`.
4. Checkout `feat/codex-jsonl-log-parser-api`; update `tasks.json` status; add an END entry to `session_log.md` with commands/results/blockers; commit docs (`docs: finish <task-id>`).
5. Remove the worktree: `git worktree remove wt/<branch>`.

## End Checklist (integration)
1. Merge code/test branches into the integration worktree; reconcile behavior to the spec.
2. Run (capture outputs):
   - `cargo fmt`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - Relevant tests (at minimum, the suites introduced by the triad’s test task)
   - Integration gate: `make preflight`
3. Commit integration changes to the integration branch.
4. Fast-forward merge the integration branch into `feat/codex-jsonl-log-parser-api`; update `tasks.json` and `session_log.md` with the END entry; commit docs (`docs: finish <task-id>`).
5. Remove the worktree.

