# C0-spec – Offline JSONL parsing API (Codex `ThreadEvent`)

## Scope
- Implement the offline parsing API exactly as specified by:
  - `docs/specs/codex-thread-event-jsonl-parser-contract.md`
  - `docs/specs/codex-thread-event-jsonl-parser-scenarios-v1.md`
  - `crates/codex/JSONL_COMPAT.md` (normative normalization semantics)
- Production code MUST add the `codex::jsonl` module and crate-root reexports required by the contract.
- Production code MUST reuse the same normalization logic as the streaming parser (no “second implementation” of normalization).
- The offline parser MUST be synchronous (`std::io::BufRead`) and line-oriented; no async reader APIs in v1.
- The offline parser MUST be tolerant-only in v1 (no “strict mode” helpers).
- Data minimization MUST hold:
  - `ThreadEventJsonlRecord` MUST NOT include the raw JSONL line on success.
  - Parse errors should retain the original line content via existing `ExecStreamError` variants when possible.

## Acceptance Criteria
- Public API compiles and matches the contract:
  - `codex::JsonlThreadEventParser` exists and has `new`, `reset`, and `parse_line`.
  - `codex::ThreadEventJsonlRecord`, `codex::ThreadEventJsonlReader`, and `codex::ThreadEventJsonlFileReader` exist.
  - `codex::thread_event_jsonl_reader` and `codex::thread_event_jsonl_file` exist.
- Offline parsing behavior matches the scenario catalog:
  - Versioned fixtures parse as required.
  - CRLF tolerance (`\\r`) behavior is locked down.
  - Unknown `type` yields per-line error and continues.
- No doc/planning edits are made from worktrees (enforced by role guardrails).

## Out of Scope
- Any agent-agnostic event schema (this stays Codex-specific).
- Async `tokio::io::AsyncBufRead` parsing APIs (explicitly deferred).
- Adding “strict mode” convenience helpers in the wrapper.
- Building Substrate-side adapters (Substrate owns mapping into its `AgentEvent` bus).

