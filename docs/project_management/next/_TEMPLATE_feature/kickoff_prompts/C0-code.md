# Kickoff Prompt – C0-code (<TRIAD TITLE>)

## Scope
- Production code only; no tests. Implement the C0-spec.

## Start Checklist
1. `git checkout feat/<feature> && git pull --ff-only`
2. Read: `plan.md`, `tasks.json`, `session_log.md`, `C0-spec.md`, this prompt.
3. Set `C0-code` status to `in_progress` in `tasks.json` (orchestration branch only).
4. Add START entry to `session_log.md`; commit docs (`docs: start C0-code`).
5. Create the task branch and worktree: `git worktree add -b <feature-prefix>-c0-<scope>-code wt/<feature-prefix>-c0-<scope>-code feat/<feature>`.
6. Do **not** edit docs/tasks/session_log.md from the worktree.

## Requirements
- Implement C0 per `C0-spec.md`.
- Protected paths: `.git`, `.substrate-git`, `.substrate`, sockets, device nodes (unless the spec explicitly says otherwise).
- Required commands (before handoff):
  - `cargo fmt`
  - `cargo clippy --workspace --all-targets -- -D warnings`
- Optional sanity checks allowed, but no required tests.

## End Checklist
1. Run the required commands above and capture their outputs.
2. Inside `wt/<feature-prefix>-c0-<scope>-code`, commit C0-code changes (no docs/tasks/session_log.md edits).
3. From outside the worktree, ensure the task branch contains the worktree commit (fast-forward if needed); do **not** merge into `feat/<feature>`.
4. Checkout `feat/<feature>`; update `tasks.json` to `completed`; add an END entry to `session_log.md` with commands/results/blockers; create downstream prompts if missing; commit docs (`docs: finish C0-code`).
5. Remove worktree `wt/<feature-prefix>-c0-<scope>-code`.
