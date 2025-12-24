# Codex Wrapper Backlog

High-priority items to make the Rust wrapper production-ready, cover Codex CLI surface area, and stay version-safe. Grouped into workstreams to enable concurrent progress.

## Current Gaps (Missing / Partial)

### P0 — Production blockers
- [MISSING] Workstream G1: Support external `notify` hook (run program with JSON payload on turn complete).
- [MISSING] Workstream C3: Credential store modes (`File/Keyring/Auto`) for core auth and MCP OAuth.
- [PARTIAL] Workstream E4: Approval elicitation plumbing (surface approvals + cancellations end-to-end for orchestration, not just protocol types).
- [PARTIAL] Workstream C4: Session/history helpers (conversation/history file helpers; durable cross-process resume ergonomics).

### P1 — Important (stability / ops / parity)
- [PARTIAL] Workstream A1: Pinned binary + CODEX_HOME override exists, but “default to bundled binary” is not the default behavior.
- [PARTIAL] Workstream A2: CODEX_HOME layout/path discovery exists; canonical “app-scoped CODEX_HOME path computation” helper is not provided.
- [PARTIAL] Workstream C1/C2: Auth parity is incomplete (device auth + issuer/client overrides; structured `login status` output when/if supported).
- [PARTIAL] Workstream E1: MCP “management” is implemented via config management APIs, but direct CLI `codex mcp ...` wrappers and persistent token materialization are not fully covered.
- [PARTIAL] Workstream F2: Update advisory is supported, but automatic release detection (npm/Homebrew/GitHub mining) is host-supplied rather than built-in.

### P2 — Optional / intentionally unwrapped
- [MISSING] Workstream B2: Interactive/TUI mode wrapper (`codex` with no args).
- [PARTIAL] Workstream G3: Long-running task resume semantics are limited (e.g., `codex-reply` cross-process persistence is not guaranteed).

## Workstream A: Binary + env isolation
1. [PARTIAL] Add builder opts for pinned binary path (default to bundled) and per-invocation `CODEX_HOME` override. (Builder opts exist; default-to-bundled is not the default behavior.)
2. [PARTIAL] Helper to compute app-scoped CODEX_HOME (e.g., `~/.myhub/codex`) and mkdir/log path discovery (`config.toml`, `auth.json`, `.credentials.json`, `history.jsonl`, `conversations/*.jsonl`, `logs/codex-*.log`). (Layout + path helpers exist; canonical “compute app-scoped path” helper is missing.)
3. [DONE] Expose composite env prep (CODEX_HOME + CODEX_BINARY) when spawning any subcommand.

## Workstream B: CLI surface coverage (exec/interactive)
1. [DONE] `exec` flags: `--image/-i`, `--model/-m`, `--oss`, `--sandbox/-s`, `--profile/-p`, `--full-auto`, `--dangerously-bypass-approvals-and-sandbox/--yolo`, `--cd/-C`, `--add-dir`, `--skip-git-repo-check`, `--output-schema`, `--color`, `--json`, `--output-last-message/-o`.
2. [MISSING] Interactive (no subcommand): mirror exec options + optional session resume hints.
3. [DONE] `resume`: resume by ID or `--last`.
4. [DONE] `apply`: apply latest diff; capture stdout/stderr/exit.
5. [DONE] `sandbox`: seatbelt/landlock/windows runners.
6. [DONE] `features`: list/enable/disable.

## Workstream C: Auth + sessions
1. [PARTIAL] `login`/`logout`: ChatGPT OAuth, API key stdin (`--with-api-key`), device auth, issuer/client overrides. (Interactive login + API key login exist; device auth and issuer/client overrides are not covered.)
2. [PARTIAL] Status parsing via `codex login status` (prefer `--json` if/when present). (Text parsing exists; no JSON mode.)
3. [MISSING] Credential store modes (`File/Keyring/Auto`) for core auth and MCP OAuth.
4. [PARTIAL] Session reuse: pass session IDs, `--last`, load conversation files; history helpers for `history.jsonl` and `conversations/*.jsonl`. (Resume selectors exist; conversation/history helpers are not implemented.)

## Workstream D: JSON streaming + logging
1. [DONE] Typed event stream for `--json` output (thread/turn/item lifecycle; item types agent_message, reasoning, command_execution, file_change, mcp_tool_call, web_search, todo_list, errors).
2. [DONE] Real-time streaming: flush per event; idle timeout surfaced as error; expose `--output-last-message` helper and schema path handling.
3. [DONE] Log tee: opt-in mirroring to files; honor `RUST_LOG`; expose log path helper.
4. [DONE] Apply/diff artifacts: return apply exit status/stdout/stderr.

## Workstream E: MCP + app-server
1. [PARTIAL] `mcp` management: `list/get/add/remove/login/logout` with JSON, env injection on add; support `[mcp_servers]` stdio + streamable_http definitions (headers, bearer env var, timeouts, enabled/disabled tools). (Config manager covers list/get/add/remove and token env-var wiring; direct CLI management wrappers and persistent token materialization are not fully covered.)
2. [DONE] Launch helpers for `codex mcp-server` and `codex app-server` (stdio JSON-RPC); lifecycle management (spawn/kill, health check).
3. [DONE] Tool params: `codex` (start session) and `codex-reply` (continue by conversationId) with prompt/cwd/model/sandbox/approval/config map; stream events to caller.
4. [PARTIAL] Approval elicitation plumbing: surface exec/apply approvals and cancellation handling.

## Workstream F: Versioning + feature detection
1. [DONE] Binary version probe (`codex --version`) and `codex features list`; cache supported flags/features per binary.
2. [PARTIAL] Update flow: detect newer releases (npm/Homebrew/github), emit advisory; optional downloader hook (outside crate) but expose hooks to plug in. (Advisory exists; release mining is host-supplied.)
3. [DONE] Capability guards: gate new flags behind detection; graceful degradation for older binaries.

## Workstream G: Notifications + long-running tasks
1. [MISSING] Support external `notify` hook (run program with JSON payload on turn complete).
2. [DONE] Ensure mcp-server/app-server event notifications bubble up to orchestration layer for wake-ups on task completion.
3. [PARTIAL] Long-running task resume: expose `codex-reply` helper for conversationId + prompt; cancellation support.

## Workstream H: Examples + docs
1. [DONE] Add examples for MCP server/app-server usage, JSON streaming, resume/apply, feature toggles, CODEX_HOME override, and bundled binary selection.
2. [DONE] API docs describing env resolution, event schema, approval/sandbox policies.

## Cross-cutting: Auth orchestration
- [DONE] Add orchestration around `auth.json`/`.credentials.json` materialization when `CODEX_HOME` is set, so isolated homes can reuse or copy authenticated state without re-running `codex login`. Provide helpers to detect and copy existing auth files into a target `CODEX_HOME` safely.

## References: neighboring crates
- `codex-helper` (proxy/failover/logging; rewrites ~/.codex/config.toml to point Codex at local proxy; manages multi-upstream, quota-aware routing; sessions/usage diagnostics). Consider ideas for multi-provider routing and config backup/restore, not code.
- `llm-link` (multi-provider proxy with app presets for Codex CLI/Zed/Claude Code, supports OpenAI/Ollama/Anthropic etc., hot reload config, dynamic model discovery). Useful patterns for multi-protocol adapters, but likely out of scope for core Codex wrapper.
