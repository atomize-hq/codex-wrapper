# Next Session Kickoff: Codex CLI Release-Trailing Plan + Workstream Roadmap

Goal: Use `docs/adr/0001-codex-cli-parity-maintenance.md` as the source of truth, expand it with concrete design/ops details, then create a full set of new workstreams (`workstreams/*`) with `BRIEF.md`, `tasks.json`, `SESSION_LOG.md`, and initial kickoff prompts following `workstreams/KICKOFF_GUIDE.md`.

This kickoff is a **planning session**: produce a reviewed roadmap and workstream scaffolding, not a large implementation change set.

Current baseline facts to anchor planning:

- Minimum supported Codex CLI: `0.61.0`
- Latest upstream Codex CLI release (as of now): `0.77.0`
- Platform priority: Linux first, then macOS, then Windows

## 0) Preflight (repo hygiene)

1) Start from a clean working tree:
   - If there are local artifacts (e.g. `events.jsonl`, `last_message.txt`, ad-hoc logs), delete or move them out of the repo.
   - If there are uncommitted code changes, decide whether to:
     - commit them as a “stabilization” batch, or
     - stash/shelve them so planning work is a clean diff.
2) Confirm current CLI binaries available locally:
   - Pinned repo binary: `./codex-x86_64-unknown-linux-musl`
   - System CLI (if installed): `codex` on `PATH`

## 1) Required reading (establish shared context)

Read and extract actionable requirements/assumptions from:

- `docs/adr/0001-codex-cli-parity-maintenance.md` (primary)
- `Audit of the Codex Wrapper and Coverage Gaps.docx.md` (audit + maintenance proposal)
- `CLI_MATRIX.md` (current static snapshot; note it is pegged to older versions)
- `capability_manifest.json` (current static “supports” matrix)
- `crates/codex/README.md` + `README.md` (public contract and “gaps” section)
- Existing workstreams for patterns and scope boundaries:
  - `workstreams/F_versioning_features/*` (capability probes/caching policy patterns)
  - `workstreams/I_cli_parity/*` (parity closure + E2E harness philosophy)
  - `workstreams/J_app_bundle/*` (bundled binary + auth seeding patterns)

## 2) Research to expand ADR into an implementable policy

Perform quick but concrete research so the ADR becomes implementable:

1) **Current Codex CLI surface** (latest you can access locally):
   - Run help/version inventories against both:
     - `./codex-x86_64-unknown-linux-musl`
     - `codex` (PATH), if present
   - Capture deltas vs our static inventories (`CLI_MATRIX.md`, `capability_manifest.json`).
2) **Official reference alignment**
   - Cross-check “commands + flags + config keys” against OpenAI Codex CLI reference docs.
3) **Schema drift reconnaissance**
   - Identify known JSONL differences across versions (events/fields) that already affected us.
   - Record “normalization rules we must maintain” and what should be strict vs lenient.
4) **Support policy proposal**
   - Recommend:
     - minimum supported Codex CLI version
     - latest validated Codex CLI version
     - how frequently we “trail” upstream (e.g., within N days or within N releases)
   - Document what “validated” means (unit tests, examples compile, E2E real-binary smoke, optional live probes).
5) **Release notes + docs signals**
   - Define how we will mine upstream GitHub release notes for candidate flags/commands as “signals to verify”.
   - Decide whether (and how) to incorporate official docs updates as a secondary signal (warn-only), not a source of truth.

## 3) Expand ADR 0001 (deliverable)

Update the ADR (or add ADR 0002 if you prefer a “policy + implementation plan” separation) with:

- Snapshot/manifest format choice (JSON/TOML) and exact schema (fields, required vs optional)
- Where snapshots live in repo (suggested: `cli_manifests/codex/current.json` + `cli_manifests/codex/README.md`)
- Repo pointers for supported versions (text files are acceptable):
  - `cli_manifests/codex/min_supported.txt`
  - `cli_manifests/codex/latest_validated.txt`
- Update cadence and ownership (who updates when upstream releases ship)
- CI enforcement model:
  - “stale snapshot” detection
  - minimum vs latest validation matrix
- GitHub automation model:
  - nightly “Release Watch” workflow (alerts only)
  - maintainer-triggered “Update Snapshot” workflow (downloads artifacts, records checksums, regenerates snapshots, opens PR, runs CI)
- Explicit “intentionally unwrapped” list and decision criteria for promoting an item into supported wrapper API

## 4) Create new workstreams (deliverable: scaffolding + tasks)

Create **new workstreams** under `workstreams/*` following the existing structure:

- `BRIEF.md` (objective/scope/constraints/deliverables/references)
- `tasks.json` (task list with IDs, dependencies, files, tests; statuses start at `todo`)
- `SESSION_LOG.md` (template + future entries)
- `kickoff_prompts/` with the first task kickoff prompt

Recommended workstream decomposition (adjust if research suggests better boundaries):

### Workstream K: CLI Snapshot + Diff Tooling
Objective: Generate and diff a structured inventory for a given `codex` binary, so “what changed?” is automatic.
Deliverables:
- Local tool (prefer `xtask` or a small Rust CLI) that outputs a snapshot file.
- Diff helper/report (human-readable) to guide wrapper updates.
- Snapshot schema doc and repo location.
Requirements:
- Recursive, exhaustive help crawl (all subcommands).
- Supplement mechanism for “not shown in `--help`” cases.
- Deterministic output for diffs, plus raw help capture for debugging.
Acceptance:
- Running snapshot tool against two binaries yields deterministic output and a clean diff.

### Workstream L: CI Validation Matrix (min vs latest)
Objective: Make “latest validated” real-binary smoke checks repeatable and gating.
Deliverables:
- CI job(s) that run `cargo test -p codex --lib`, `--examples`, and `--test cli_e2e`.
- Mechanism to supply pinned binaries in CI (download or vendored artifact) without polluting the repo.
- Clear CI env knobs for optional live probes.
Also:
- GitHub workflows for “Release Watch” and maintainer-triggered “Update Snapshot”.
Acceptance:
- CI fails on wrapper regressions against pinned “latest validated” binary.

### Workstream M: JSONL & Notification Schema Compatibility
Objective: Keep streaming/MCP/app-server parsing robust across versions.
Deliverables:
- Explicit normalization rules and tests for known drift cases.
- Fixture capture/update workflow (optionally a helper that records JSONL from a real binary into `crates/codex/examples/fixtures/*`).
Acceptance:
- Stream parsing does not collapse on one malformed/legacy event; errors are surfaced but stream continues when possible.

### Workstream N: Ops Playbook (Release Trailing)
Objective: Make updates procedural and low-risk.
Deliverables:
- “Release trailing” checklist: update snapshot, run matrix, update docs/examples, bump crate version, publish notes.
- Decide what stays unwrapped and how to track it.
Acceptance:
- A maintainer can follow the playbook to trail an upstream Codex release with minimal guesswork.

## 5) Planning artifacts to produce (end-of-session outputs)

At the end of the planning session, the repo should contain:

1) Updated ADR(s) with concrete, reviewable decisions.
2) New `workstreams/*` directories with BRIEF/tasks/logs/kickoffs for K/L/M/N (or your final split).
3) A short “Roadmap summary” (can be a section added to `HANDOFF.md` or a new `docs/ROADMAP.md`) listing:
   - the workstreams
   - key milestones
   - what “done” looks like
4) A concrete “trial run” plan to validate the new process by moving from `0.61.0` toward current upstream (`0.77.0`), starting with Linux binaries.

## 6) Validation checklist for the planning session

- No implementation work is required beyond doc/scaffold updates.
- Ensure new workstreams follow `workstreams/KICKOFF_GUIDE.md`.
- Ensure the plan is actionable: each task has clear outputs, files to change, and tests to run.
