You are starting Workstream I (CLI Parity), Task I8-sandbox-command-impl.

Branch/worktree workflow (follow before coding):
1) Checkout the workstream branch: `git checkout ws/I_cli_parity`.
2) In `workstreams/I_cli_parity/tasks.json`, mark this task as "doing" (edit the JSON) while on the workstream branch.
3) Log session start in `workstreams/I_cli_parity/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/I8-sandbox-command-impl`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-I8 task/I8-sandbox-command-impl` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: Implement the `codex sandbox` wrapper per the I7 design: API surface + request structs/enums, CLI wiring (platform aliases, flags, working-dir handling, post-run stance), tests, and docs/examples.
Resources: workstreams/I_cli_parity/BRIEF.md, workstreams/I_cli_parity/tasks.json, workstreams/I_cli_parity/design_notes/I7-sandbox-command-design.md, CLI_MATRIX.md, crates/codex/src/lib.rs, README.md, Codex CLI help.
Deliverable: Sandbox command support committed (code/tests/docs), reflecting the agreed design and noting any gaps.

Completion steps (in this order):
1) In the worktree on the task branch: finish code/docs/tests, `git status`, `git add ...`, `git commit -m "<msg>"`.
2) Return to the workstream branch: `git checkout ws/I_cli_parity`.
3) Merge the task branch into the workstream branch: `git merge --no-ff task/I8-sandbox-command-impl`.
4) Remove the worktree if you created one: `git worktree remove ../wt-I8` (optional but recommended).
5) Update `workstreams/I_cli_parity/tasks.json` to reflect the task status or add follow-ons.
6) Update `workstreams/I_cli_parity/SESSION_LOG.md` with end time/outcome.
7) Write the kickoff prompt for the next task in `workstreams/I_cli_parity/kickoff_prompts/<next>.md` (follow this guide); do this while on the workstream branch.
