You are starting Workstream L (CI Validation Matrix), Task L1-define-validation-matrix.

Branch/worktree workflow (follow before coding, see workstreams/KICKOFF_GUIDE.md):
1) Checkout the workstream branch: `git checkout ws/L_ci_validation_matrix` (or create it from main if missing).
2) In `workstreams/L_ci_validation_matrix/tasks.json`, mark this task as "doing" (edit JSON) while on the workstream branch.
3) Log session start in `workstreams/L_ci_validation_matrix/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/L1-define-validation-matrix`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-L1 task/L1-define-validation-matrix` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: define the CI validation matrix for Codex CLI parity (min supported vs latest validated), including which tests are gating and which probes are opt-in.
Resources: `docs/adr/0001-codex-cli-parity-maintenance.md`, `workstreams/L_ci_validation_matrix/BRIEF.md`, `cli_manifests/codex/min_supported.txt`, `cli_manifests/codex/latest_validated.txt`, `crates/codex/tests/cli_e2e.rs`.
Deliverable: documented matrix rules and environment knobs that Workstream L will implement in workflows.

Completion steps (in this order, see workstreams/KICKOFF_GUIDE.md):
1) In the worktree on the task branch: finish docs/design, `git status`, `git add ...`, `git commit -m "<msg>"`.
2) Return to the workstream branch `ws/L_ci_validation_matrix`: `git checkout ws/L_ci_validation_matrix`.
3) Merge the task branch: `git merge --no-ff task/L1-define-validation-matrix`.
4) Remove the worktree: `git worktree remove ../wt-L1` (optional but recommended).
5) In `workstreams/L_ci_validation_matrix/tasks.json`, update this task status to "done".
6) Update `workstreams/L_ci_validation_matrix/SESSION_LOG.md` with end time/outcome.
7) Write the kickoff prompt for the next task in `workstreams/L_ci_validation_matrix/kickoff_prompts/<next>.md` while on the workstream branch.

