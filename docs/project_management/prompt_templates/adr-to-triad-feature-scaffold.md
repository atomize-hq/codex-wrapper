# Prompt Template: ADR → Triad Feature Scaffold (Docs Only)

Use this prompt when you have an ADR (like `docs/adr/0001-codex-cli-parity-maintenance.md`) and you want an agent to generate a complete triad-based feature scaffold: `plan.md`, `tasks.json`, `session_log.md`, `C*-spec.md`, kickoff prompts, and updates to `docs/project_management/next/sequencing.json`.

## Prompt

You are given an ADR and must convert it into an execution-ready **triad feature scaffold** for this repo.

### Hard rules
1. **Docs/planning only.** Do not implement production code or tests. Do not download binaries. Do not run “live” Codex flows. Only create/update planning documents and templates listed below.
2. **Use this repo’s triad standard** as the source of truth for structure and checklists:
   - `docs/project_management/task-triads-feature-setup-standard.md`
   - `docs/project_management/next/_TEMPLATE_feature/`
3. **Worktree conventions**
   - Orchestration branch: `feat/<feature>`
   - Task worktrees live under `wt/<branch>` (in-repo; ignored by git)
   - **Docs/tasks/session logs are edited only on the orchestration branch** (never from inside worktrees).
4. **Specs are the source of truth.** You must translate the ADR into a set of `C*-spec.md` files that are explicit about scope, acceptance, and out-of-scope.
5. **Context sizing constraint.** Split work so each triad phase fits comfortably in a single agent’s context window (aim ≤ ~40–50% of a 272k window).

### Inputs (provided by the user)
- ADR path: `<ADR_PATH>`
- New feature directory name (slug): `<feature>` (example: `codex-cli-release-trailing`)
- Feature title: `<FEATURE_TITLE>`
- Feature prefix for branches/worktrees: `<feature-prefix>` (example: `ctr`)

### Deliverables (must be created/updated)

Create the new feature directory by copying the template and filling it in:

- Create: `docs/project_management/next/<feature>/`
  - Required:
    - `docs/project_management/next/<feature>/plan.md`
    - `docs/project_management/next/<feature>/tasks.json`
    - `docs/project_management/next/<feature>/session_log.md`
    - `docs/project_management/next/<feature>/kickoff_prompts/`
    - `docs/project_management/next/<feature>/C0-spec.md`
    - `docs/project_management/next/<feature>/C1-spec.md`
    - … additional `C*-spec.md` as needed
  - Required kickoff prompts per triad:
    - `docs/project_management/next/<feature>/kickoff_prompts/C0-code.md`
    - `docs/project_management/next/<feature>/kickoff_prompts/C0-test.md`
    - `docs/project_management/next/<feature>/kickoff_prompts/C0-integ.md`
    - Repeat for C1/C2/…

Update sequencing:
- Update: `docs/project_management/next/sequencing.json`
  - Add a new track for `<feature>` (or add phases to an existing track if the ADR extends one).
  - Add a `sequence` entry for each phase (`C0`, `C1`, …) with explicit dependencies and blocking.

### What to extract from the ADR
From `<ADR_PATH>`, extract and restate (in your feature docs/specs/tasks):
- The decision(s) and the “source of truth” statements.
- Required artifacts/formats/schemas (files to create, where they live, required fields).
- Operational processes and “definition of done” (validation steps, CI gating, pointers like `min_supported`/`latest_validated`).
- Constraints (Linux-first, no network in crate runtime, safety rules, etc.).
- Explicitly unwrapped / out-of-scope surfaces and promotion criteria (if present).

### How to structure triads
1. Identify distinct deliverables that can be shipped independently.
2. For each deliverable, create a phase `C<N>` with three tasks:
   - `C<N>-code`: production code only (later executed by a code agent)
   - `C<N>-test`: tests/fixtures only (later executed by a test agent)
   - `C<N>-integ`: merge + reconcile to spec + run integration gate (later executed by an integration agent)
3. Keep each `C<N>-spec.md` small and explicit:
   - **Scope:** required behaviors, defaults, error handling, platform guards, and any protected paths
   - **Acceptance Criteria:** observable outcomes (what must be true / what outputs exist)
   - **Out of Scope:** anything intentionally deferred

### `plan.md` requirements
In `docs/project_management/next/<feature>/plan.md`:
- Summarize purpose and guardrails (triads-only, roles, doc-edit rules).
- Add a **Triad Overview** listing each `C<N>` with 1–2 sentences.
- Include the standard Start/End checklists and the integration gate (`make preflight`).
- Include context sizing guidance (when/how to split phases).

### `tasks.json` requirements
In `docs/project_management/next/<feature>/tasks.json`:
- Create one entry per role for each phase: `C0-code`, `C0-test`, `C0-integ`, then `C1-*`, etc.
- For each task, fill in **all required fields**:
  - `id`, `name`, `type`, `phase`, `status`, `description`
  - `references` (must include `<ADR_PATH>`, feature `plan.md`, and the relevant `C*-spec.md`)
  - `acceptance_criteria` (concrete bullets)
  - `start_checklist` and `end_checklist` (copy/paste from the standard, but fully instantiated for this feature)
  - `worktree` (use `wt/<feature-prefix>-cN-<scope>-<role>`)
  - `integration_task`
  - `kickoff_prompt`
  - `depends_on` / `concurrent_with`
- Statuses must start as `pending`.

### Kickoff prompt requirements
For each kickoff prompt:
- Start with a one-paragraph scope summary referencing the spec.
- Include explicit role boundaries:
  - Code: production code only; no tests; run `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`.
  - Test: tests only; run `cargo fmt` + targeted `cargo test ...`.
  - Integration: merge code+test; reconcile to spec; run `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings` + relevant tests + `make preflight`.
- Include the standard Start Checklist and End Checklist, fully instantiated with:
  - `feat/<feature>` branch name
  - branch name
  - worktree path `wt/<branch>`
  - exact required commands for the role
- Note: kickoff prompts are documentation; they do not execute anything themselves.

### Sequencing (`sequencing.json`) requirements
Update `docs/project_management/next/sequencing.json`:
- Add a track entry:
  - `id`: `<feature>` (or a stable snake_case id)
  - `name`: `<FEATURE_TITLE>`
  - `source`: `<ADR_PATH>`
  - `phases`: one per `C<N>` with `id`, `name`, and `spec` path
- Add `sequence` items with:
  - `order`: increments of 10
  - `id`: `<feature>:C<N>` or `<feature>:C<N>`-like stable identifier
  - `depends_on`: enforce C0 → C1 → C2 ordering unless the ADR explicitly allows parallelism
  - `blocks`: list later phases that should not start early

### Output quality bar
- A fresh agent should be able to pick up any triad from the docs and execute it without additional discovery.
- Every spec must be unambiguous about what is being built and how it will be validated.
- Every task must include copy/paste-able checklists and commands.

