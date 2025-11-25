# Workstream I7 — Sandbox Command Wrapper Design

Goal: expose `codex sandbox`/`codex debug` from the Rust wrapper, covering platform subcommands, flag mapping, and post-run behavior so callers can launch arbitrary commands inside Codex-provided sandboxes.

## CLI surface recap
- Subcommands: `macos` (alias `seatbelt`), `linux` (alias `landlock`), `windows`. Top-level `sandbox` accepts only `--config/--enable/--disable` plus help; platform subcommands add `--full-auto` and (macOS only) `--log-denials`. Other shared Codex flags (profile, approval, sandbox, cd, search, etc.) are rejected.
- Args: trailing `[COMMAND]...` is required; the CLI panics when omitted. `--full-auto` maps to workspace-write sandbox mode with cwd/TMP writable and network disabled; otherwise sandbox mode defaults to read-only for the helper.
- Behavior: `Config::load_with_cli_overrides` sets sandbox policy + cwd (defaulting to process cwd). Linux uses the bundled `codex-linux-sandbox` binary; Windows runs via `run_windows_sandbox_capture` and exits the parent with the captured status; macOS can stream denials via `--log-denials` after the child exits. Stdout/stderr inherit.
- Exit: the CLI mirrors the inner command’s exit code (non-zero exits are not wrapped) and prints denial logs on macOS when requested. There is no built-in post-run hook beyond the macOS denial summary.

## Proposed Rust surface
- New request/response types:
  - `SandboxPlatform { Macos, Linux, Windows }` → subcommand/alias mapping.
  - `SandboxCommandRequest { platform, command: Vec<OsString>, full_auto: bool, log_denials: bool, config_overrides: Vec<ConfigOverride>, feature_toggles: FeatureToggles, working_dir: Option<PathBuf> }`, where `FeatureToggles { enable: Vec<String>, disable: Vec<String> }` maps to `--enable/--disable` (or callers can use `config_overrides` with `features.<name>=bool` for consistency with existing helpers).
  - `SandboxRun { status: ExitStatus, stdout: String, stderr: String }`.
- `impl CodexClient { pub async fn run_sandbox(&self, request: SandboxCommandRequest) -> Result<SandboxRun, CodexError>; }` returning captured output + status (non-zero statuses are surfaced in the result, not turned into `NonZeroExit` errors).
- No builder-level defaults beyond existing env/binary/codex_home/timeout/mirroring; per-call request drives platform/flags. `log_denials` is only honored on macOS (no-op elsewhere).

## Flag/arg mapping & execution
- Platform → subcommand (`macos`/`seatbelt`, `linux`/`landlock`, `windows`). If the host OS lacks support, the CLI will exit non-zero; we propagate the status and stderr.
- `command` → trailing argv (no shell interpolation; require non-empty to avoid the CLI panic).
- `full_auto` → `--full-auto` (let the CLI set the workspace-write sandbox preset). `log_denials` → `--log-denials` only for macOS.
- Config/feature toggles: emit repeated `--config key=value`, `--enable`, and `--disable` from the request; do **not** reuse `CliOverrides` safety/profile/search/approval plumbing because the CLI rejects those flags on `sandbox`.
- CWD: set the spawned process cwd to `request.working_dir` or builder `working_dir` if provided; otherwise use `std::env::current_dir()` (align with CLI default instead of the exec/resume tempdir helper).
- Env/timeouts/output: reuse `CommandEnvironment::apply` (CODEX_HOME/BINARY/RUST_LOG) and builder `timeout`/`mirror_stdout`/`quiet` for stdout/stderr handling similar to `apply_or_diff`. No `--skip-git-repo-check` or model flags are attached.
- Post-run: wrapper simply returns `SandboxRun`; callers can run any follow-up/cleanup script after awaiting the result. macOS denial logs are just forwarded output (no structured parsing in the wrapper).

## Testing plan
- Add unit tests with the fake codex binary harness to assert argv mapping per platform (macos/linux/windows labels + aliases), `--full-auto`, `--log-denials` passthrough, and config/feature toggles. Validate we refuse empty commands.
- Cover cwd resolution (defaulting to process cwd when builder/request unset) and that non-zero exit statuses are returned without being converted into errors.
- Keep platform-specific expectations loose (skip macOS/log-denials or Windows when the host cannot execute the real sandbox binary; rely on the fake binary for arg capture).

## Docs plan
- Update `README.md`/`CLI_MATRIX.md` to describe the new `run_sandbox` API, platform enums, flag coverage, and exit/status behavior (including macOS denial logging and Windows experimental notes).
- Note that only `--config`/`--enable`/`--disable`/`--full-auto`/`--log-denials` are forwarded; other CLI overrides remain scoped to exec/resume/apply/diff.
- Add a short example showing `SandboxPlatform::Linux` with `full_auto` and a trailing command vector.

## Open questions / limits
- Post-run hooks: CLI offers no hook beyond macOS denial logging; if a built-in “on finish run script” helper is desired, we’d need to add a wrapper-level callback to run after the sandbox child exits.
- Windows sandboxing is experimental and requires the bundled helper; tests likely stay fake-binary-only. macOS denial logging cannot be exercised in CI on non-mac hosts.
- `FeatureToggles` ergonomics: OK to rely solely on `--config features.<name>=bool` instead of explicit enable/disable helpers?
