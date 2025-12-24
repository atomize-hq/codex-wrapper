You are starting Workstream I (CLI Parity), Task I16-cli-parity-e2e-followups.

Branch/worktree workflow (follow before coding):
1) Checkout the workstream branch: `git checkout ws/I_cli_parity`.
2) In `workstreams/I_cli_parity/tasks.json`, mark this task as "doing" (edit the JSON) while on the workstream branch.
3) Log session start in `workstreams/I_cli_parity/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/I16-cli-parity-e2e-followups`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-I16 task/I16-cli-parity-e2e-followups` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: Extend the real-binary e2e harness to cover exec/resume/apply/diff once prerequisites are available (usable Codex binary features + credentials), and retire skips by validating execpolicy/feature flag support as the CLI evolves. Exercise the harness against live flows (or sanctioned fixtures) to record expected behaviors for diff/apply/resume, and tighten skip/reporting so gating is explicit (binary path, CODEX_HOME/profile, API key/login assumptions).
Resources: workstreams/I_cli_parity/BRIEF.md, workstreams/I_cli_parity/tasks.json, CLI_MATRIX.md, README.md, crates/codex/tests/cli_e2e.rs, crates/codex/src/lib.rs, crates/codex/examples.
Deliverable: Updated e2e harness/tests (or documented deferral) that probe exec/resume/apply/diff and execpolicy gaps on a real CLI binary, plus any supporting docs/code committed on the task branch; include test results.

Completion steps (in this order):
1) In the worktree on the task branch: finish code/docs/tests, `git status`, `git add ...`, `git commit -m "<msg>"`.
2) Return to the workstream branch: `git checkout ws/I_cli_parity`.
3) Merge the task branch into the workstream branch: `git merge --no-ff task/I16-cli-parity-e2e-followups`.
4) Remove the worktree if you created one: `git worktree remove ../wt-I16` (optional but recommended).
5) Update `workstreams/I_cli_parity/tasks.json` to mark the task "done" or note follow-ups.
6) Update `workstreams/I_cli_parity/SESSION_LOG.md` with end time/outcome.
7) Write the kickoff prompt for the next task in `workstreams/I_cli_parity/kickoff_prompts/<next>.md` (follow this guide); do this while on the workstream branch.
