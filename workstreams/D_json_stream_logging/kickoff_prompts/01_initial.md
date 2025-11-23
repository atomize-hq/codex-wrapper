You are starting Workstream D (JSON Streaming + Logging), Task D1-design-stream-types.

Branch/worktree workflow (follow before coding):
1) Checkout the workstream branch: `git checkout ws/D_json_stream_logging`.
2) In `workstreams/D_json_stream_logging/tasks.json`, mark this task as \"doing\" (edit the JSON) while on the workstream branch.
3) Log session start in `workstreams/D_json_stream_logging/SESSION_LOG.md`.
4) Create the task branch from the workstream branch: `git checkout -b task/D1-design-stream-types`.
5) Create a task worktree from that branch (example): `git worktree add ../wt-D1 task/D1-design-stream-types` and do all code in the worktree. Do **not** edit docs/logs inside the worktree.

Task goal: design typed JSONL event API and streaming surface for `codex exec --json`, covering thread/turn/item lifecycle and item variants.
Resources: workstreams/D_json_stream_logging/BRIEF.md, workstreams/D_json_stream_logging/tasks.json, crates/codex/src/lib.rs, DeepWiki notes in BACKLOG.md.
Deliverable: event type definitions and API sketch (doc comments or design note) committed to the repo.

Completion steps (in this order):
1) Return to the workstream branch `ws/D_json_stream_logging` (if you were in the worktree/task branch).
2) In `workstreams/D_json_stream_logging/tasks.json`, update this task status to \"done\" (or equivalent).
3) Update `workstreams/D_json_stream_logging/SESSION_LOG.md` with end time/outcome.
4) Write the kickoff prompt for the next task in `workstreams/D_json_stream_logging/kickoff_prompts/<next>.md` (create the file) while on the workstream branch.
