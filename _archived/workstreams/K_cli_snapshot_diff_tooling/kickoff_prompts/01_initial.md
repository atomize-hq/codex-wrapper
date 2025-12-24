You are starting Workstream K (CLI Snapshot + Diff Tooling), Task K1-snapshot-schema-and-layout.

Branch/worktree workflow (follow before coding, see workstreams/KICKOFF_GUIDE.md):
1) Checkout the workstream branch: `git checkout ws/K_cli_snapshot_diff_tooling` (or create it from main if missing).
2) In `workstreams/K_cli_snapshot_diff_tooling/tasks.json`, mark this task as "doing" (edit JSON) while on the workstream branch.
3) Log session start in `workstreams/K_cli_snapshot_diff_tooling/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/K1-snapshot-schema-and-layout`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-K1 task/K1-snapshot-schema-and-layout` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: finalize the Codex CLI snapshot schema and file layout so snapshots are deterministic, reviewable, and can be used as the primary drift signal.
Resources: `docs/adr/0001-codex-cli-parity-maintenance.md`, `cli_manifests/codex/README.md`, `CLI_MATRIX.md`, `capability_manifest.json`.
Deliverable: updated schema/layout docs plus any small repo scaffolding needed to make Workstream K implementable.

Completion steps (in this order, see workstreams/KICKOFF_GUIDE.md):
1) In the worktree on the task branch: finish design/docs, `git status`, `git add ...`, `git commit -m "<msg>"`.
2) Return to the workstream branch `ws/K_cli_snapshot_diff_tooling`: `git checkout ws/K_cli_snapshot_diff_tooling`.
3) Merge the task branch: `git merge --no-ff task/K1-snapshot-schema-and-layout`.
4) Remove the worktree: `git worktree remove ../wt-K1` (optional but recommended).
5) In `workstreams/K_cli_snapshot_diff_tooling/tasks.json`, update this task status to "done".
6) Update `workstreams/K_cli_snapshot_diff_tooling/SESSION_LOG.md` with end time/outcome.
7) Write the kickoff prompt for the next task in `workstreams/K_cli_snapshot_diff_tooling/kickoff_prompts/<next>.md` while on the workstream branch.

