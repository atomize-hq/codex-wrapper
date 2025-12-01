You are starting Workstream I (CLI Parity), Task I15-cli-parity-e2e.

Branch/worktree workflow (follow before coding):
1) Checkout the workstream branch: `git checkout ws/I_cli_parity`.
2) In `workstreams/I_cli_parity/tasks.json`, mark this task as "doing" (edit the JSON) while on the workstream branch.
3) Log session start in `workstreams/I_cli_parity/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/I15-cli-parity-e2e`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-I15 task/I15-cli-parity-e2e` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: Add end-to-end coverage against a real Codex CLI binary for key wrappers (exec/resume/apply/diff, sandbox, codegen/proxy/uds/features/execpolicy) or clearly document why it remains deferred. Build a light harness that can consume a supplied binary path, reuse fixtures where possible, and record gaps/skip conditions.
Resources: workstreams/I_cli_parity/BRIEF.md, workstreams/I_cli_parity/tasks.json, CLI_MATRIX.md, README.md, crates/codex/EXAMPLES.md, crates/codex/src/lib.rs, crates/codex/tests, crates/codex/examples.
Deliverable: E2E test plan/harness (or explicit deferral with rationale) plus any supporting docs/code committed on the task branch; include test results.

Completion steps (in this order):
1) In the worktree on the task branch: finish code/docs/tests, `git status`, `git add ...`, `git commit -m "<msg>"`.
2) Return to the workstream branch: `git checkout ws/I_cli_parity`.
3) Merge the task branch into the workstream branch: `git merge --no-ff task/I15-cli-parity-e2e`.
4) Remove the worktree if you created one: `git worktree remove ../wt-I15` (optional but recommended).
5) Update `workstreams/I_cli_parity/tasks.json` to mark the task "done" or note follow-ups.
6) Update `workstreams/I_cli_parity/SESSION_LOG.md` with end time/outcome.
7) Write the kickoff prompt for the next task in `workstreams/I_cli_parity/kickoff_prompts/<next>.md` (follow this guide); do this while on the workstream branch.
