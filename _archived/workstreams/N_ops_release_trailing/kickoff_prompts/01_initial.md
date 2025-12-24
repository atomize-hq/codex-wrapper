You are starting Workstream N (Ops Playbook), Task N1-release-trailing-checklist.

Branch/worktree workflow (follow before coding, see workstreams/KICKOFF_GUIDE.md):
1) Checkout the workstream branch: `git checkout ws/N_ops_release_trailing` (or create it from main if missing).
2) In `workstreams/N_ops_release_trailing/tasks.json`, mark this task as "doing" (edit JSON) while on the workstream branch.
3) Log session start in `workstreams/N_ops_release_trailing/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/N1-release-trailing-checklist`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-N1 task/N1-release-trailing-checklist` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: write the maintainer playbook that makes trailing upstream Codex CLI releases procedural and low-risk (snapshot diff → CI matrix → wrapper updates → release).
Resources: `docs/adr/0001-codex-cli-parity-maintenance.md`, `docs/ROADMAP.md`, `workstreams/N_ops_release_trailing/BRIEF.md`, workstreams F/I/J patterns.
Deliverable: a step-by-step checklist with decision criteria and a “trial run” checklist for moving from 0.61.0 to 0.77.0 on Linux.

Completion steps (in this order, see workstreams/KICKOFF_GUIDE.md):
1) In the worktree on the task branch: finish docs, `git status`, `git add ...`, `git commit -m "<msg>"`.
2) Return to the workstream branch `ws/N_ops_release_trailing`: `git checkout ws/N_ops_release_trailing`.
3) Merge the task branch: `git merge --no-ff task/N1-release-trailing-checklist`.
4) Remove the worktree: `git worktree remove ../wt-N1` (optional but recommended).
5) In `workstreams/N_ops_release_trailing/tasks.json`, update this task status to "done".
6) Update `workstreams/N_ops_release_trailing/SESSION_LOG.md` with end time/outcome.
7) Write the kickoff prompt for the next task in `workstreams/N_ops_release_trailing/kickoff_prompts/<next>.md` while on the workstream branch.

