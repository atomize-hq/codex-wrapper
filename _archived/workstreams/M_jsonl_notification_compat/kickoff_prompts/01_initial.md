You are starting Workstream M (JSONL & Notification Schema Compatibility), Task M1-normalization-policy.

Branch/worktree workflow (follow before coding, see workstreams/KICKOFF_GUIDE.md):
1) Checkout the workstream branch: `git checkout ws/M_jsonl_notification_compat` (or create it from main if missing).
2) In `workstreams/M_jsonl_notification_compat/tasks.json`, mark this task as "doing" (edit JSON) while on the workstream branch.
3) Log session start in `workstreams/M_jsonl_notification_compat/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/M1-normalization-policy`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-M1 task/M1-normalization-policy` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: document our normalization/compatibility rules for JSONL streams and MCP/app-server notifications so future drift is handled consistently.
Resources: `docs/adr/0001-codex-cli-parity-maintenance.md`, `workstreams/M_jsonl_notification_compat/BRIEF.md`, `crates/codex/src/lib.rs`, `crates/codex/src/mcp.rs`, `crates/codex/tests/cli_e2e.rs`.
Deliverable: a clear policy doc (and optionally design notes) that Workstream M can turn into fixtures + tests.

Completion steps (in this order, see workstreams/KICKOFF_GUIDE.md):
1) In the worktree on the task branch: finish docs/design, `git status`, `git add ...`, `git commit -m "<msg>"`.
2) Return to the workstream branch `ws/M_jsonl_notification_compat`: `git checkout ws/M_jsonl_notification_compat`.
3) Merge the task branch: `git merge --no-ff task/M1-normalization-policy`.
4) Remove the worktree: `git worktree remove ../wt-M1` (optional but recommended).
5) In `workstreams/M_jsonl_notification_compat/tasks.json`, update this task status to "done".
6) Update `workstreams/M_jsonl_notification_compat/SESSION_LOG.md` with end time/outcome.
7) Write the kickoff prompt for the next task in `workstreams/M_jsonl_notification_compat/kickoff_prompts/<next>.md` while on the workstream branch.

