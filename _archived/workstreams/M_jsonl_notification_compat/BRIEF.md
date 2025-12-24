# Workstream M: JSONL & Notification Schema Compatibility

Objective: Keep streaming (`--json`) and server notifications (MCP/app-server) robust across Codex CLI version drift.

Scope
- Document “normalization rules” and strict-vs-lenient behavior for streaming events and notifications.
- Add tests for known drift cases (missing IDs, alternate field names, partial events, unknown items).
- Establish a fixture capture/update workflow (optionally a helper that records JSONL from a real binary into fixtures for review).

Constraints
- Do not collapse the entire stream on a single malformed or legacy event; surface errors but continue when possible.
- Prefer additive compatibility (unknown field capture) over breaking parsing changes.
- Keep live/credentialed capture opt-in; fixtures must be reviewable and sanitized.

References
- ADR policy: `docs/adr/0001-codex-cli-parity-maintenance.md`.
- Existing normalization patterns: `crates/codex/src/lib.rs` (stream event normalization), `crates/codex/src/mcp.rs` (notification parsing).
- Existing fixtures: `crates/codex/examples/fixtures/*` (if present).

Deliverables
- Documented normalization/compatibility policy.
- Fixture-backed tests for streaming and notifications.
- A contributor-friendly process to refresh fixtures when Codex changes.

