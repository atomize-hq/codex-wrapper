# Agent Wrappers (Rust)

This repository provides Rust wrappers around coding-agent CLIs, starting with:
- OpenAI Codex CLI (`codex`)
- Anthropic Claude Code (`claude`)

Codex wrapper highlights:
- Typed streaming of `codex exec --json` events (`ThreadEvent`)
- Offline parsing of saved JSONL logs into the same `ThreadEvent` model
- Capability probing and compatibility shims for upstream drift
- Tooling and artifacts for release-trailing parity maintenance (`cli_manifests/codex/`)

Claude Code wrapper highlights (v1):
- Non-interactive `--print` execution wrapper
- Tolerant parsing of `--output-format=stream-json` (NDJSON)
- Release-trailing parity lane (`cli_manifests/claude_code/`)

## Start here

- Wrapper API docs: `crates/codex/README.md`
- Examples index: `crates/codex/EXAMPLES.md`
- Documentation index: `docs/README.md`
- Contributing: `CONTRIBUTING.md`

## Repo map

- `crates/codex/` — Rust wrapper crate
- `crates/claude_code/` — Rust wrapper crate
- `docs/` — ADRs, specs, integration notes, project management
- `cli_manifests/codex/` — Codex CLI parity artifacts + ops docs
- `cli_manifests/claude_code/` — Claude Code parity artifacts + ops docs

## Operations / parity maintenance

- Ops playbook: `cli_manifests/codex/OPS_PLAYBOOK.md`
- CLI snapshot artifacts: `cli_manifests/codex/README.md`
- Decisions (ADRs): `docs/adr/`
- Normative contracts: `docs/specs/`
