# Workstream I: CLI Parity (flags/config) and Session Helpers

Objective: Bring the Rust wrapper to full parity with Codex CLI 0.61 flags/options/config knobs and add missing session helpers.

Scope
- Add builder/request fields to pass all missing flags: `--config` overrides, `--ask-for-approval`, `--sandbox`, `--full-auto`, `--dangerously-bypass-approvals-and-sandbox`, `--cd`, `--local-provider`, `--search`, `--last`, `--all`.
- Surface config profile selection (`--profile <CONFIG_PROFILE>`) alongside other CLI parity fields.
- Expose config override support (`--config key=value`) and/or targeted setters for reasoning/verbosity/summaries; wire through per-command.
- Ensure CODEX_HOME env prep remains applied per spawn.
- (Stretch) Add a higher-level auth/session helper that can check login status and prompt login as needed (optional if time).

Constraints
- Preserve backward compatibility: defaults unchanged unless new options are set.
- Apply env overrides per Command; do not mutate parent process env.
- Keep behavior consistent across exec/resume and other subcommands we wrap.
- Tests and docs/examples must be updated.

References
- CLI inventory: `CLI_MATRIX.md`.
- Current client: `crates/codex/src/lib.rs`.
- Workstream tasks live in `workstreams/I_cli_parity/tasks.json`; kickoff prompts in `workstreams/I_cli_parity/kickoff_prompts/`.

Deliverables
- Builder/request API covering missing flags and config overrides.
- Code changes wired to spawn commands with the new options.
- Tests covering new flags/config paths.
- Docs/EXAMPLES refreshed to show the new APIs.

## CLI Parity Closeout Notes
- Verified CLI parity docs/examples after the app-server codegen additions (I10b); clarified codegen error handling (non-zero exits raise `CodexError::NonZeroExit`).
- Added a real-binary E2E harness (`crates/codex/tests/cli_e2e.rs`) gated by `CODEX_E2E_BINARY`/`CODEX_E2E_HOME`. Covers `features list` (text-only), app-server codegen, `sandbox` (Linux), `responses-api-proxy`, and `stdio-to-uds`, and records CLI gaps for `diff`/`apply` (expects task/TTY) and absent `execpolicy check`.
- Live probe for exec/resume/diff/apply is gated by `CODEX_E2E_LIVE`; the current 0.61.0 CLI against `~/.codex` fails due to an invalid execpolicy file in CODEX_HOME (parse error) and sparse JSON events (`turn.started` missing thread_id), so the test records skips instead of failures.
- Remaining gaps: `codex cloud exec`/shell completion stay unwrapped (experimental/setup-time), and exec/resume/streamed prompts still require live sessions/API access so automation remains opt-in via the new harness.
