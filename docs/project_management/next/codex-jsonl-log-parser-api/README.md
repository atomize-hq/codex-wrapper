# CODEX_JSONL_LOG_PARSER_API (ADR 0005)

Implements a small, public, offline parsing API for Codex `--json` JSONL logs that reuses the
wrapper’s normalization rules and yields typed `ThreadEvent` values with per-line outcomes.

Source ADR: `docs/adr/0005-codex-jsonl-log-parser-api.md`.

This feature is executed via triads (code/test/integration) per:
`docs/project_management/task-triads-feature-setup-standard.md`.

