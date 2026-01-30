# C1 - Full scenario catalog (v1) + parity exclusions (Spec)

## Purpose
Complete ADR 0003 by implementing the full Scenario Catalog v1 and locking behavior down with tests so:
- `cli_manifests/codex/wrapper_coverage.json` is comprehensive (per v1 catalog), deterministic, and offline,
- reports classify excluded TUI-only identities under `excluded_*` deltas (not `missing_*`),
- the generator never claims excluded identities (validator-enforced), and
- parity deltas become actionable for new upstream releases.

## Normative references (must drive implementation)
- `docs/adr/0003-wrapper-coverage-auto-generation.md`
- `docs/specs/codex-wrapper-coverage-generator-contract.md`
- `docs/specs/codex-wrapper-coverage-scenarios-v1.md` (Scenarios 0-12)
- `cli_manifests/codex/RULES.json` (especially `parity_exclusions`)
- `cli_manifests/codex/SCHEMA.json` (CoverageReportV1 `excluded_*` deltas)
- `cli_manifests/codex/VALIDATOR_SPEC.md` (parity exclusions checks)

## Scope

### 1) Implement Scenario Catalog v1 (Scenarios 3-12)
File: `crates/codex/src/wrapper_coverage_manifest.rs`

Extend `wrapper_coverage_manifest()` to include (in addition to C0 coverage):
- Scenario 3: `["resume"]` (flags + args as specified; includes `SESSION_ID` and `PROMPT`)
- Scenario 4: `["apply"]`, `["diff"]`
- Scenario 5: `["login"]`, `["login","status"]`, `["logout"]` with capability-guarded `--mcp` note policy
- Scenario 6: `["features","list"]` with `--json`
- Scenario 7: `["app-server","generate-ts"]`, `["app-server","generate-json-schema"]` with `--out`, plus `--prettier` only under `["app-server","generate-ts"]`
- Scenario 8: `["responses-api-proxy"]` with required flags
- Scenario 9: `["stdio-to-uds"]` with `SOCKET_PATH` arg
- Scenario 10: `["sandbox","macos"]`, `["sandbox","linux"]`, `["sandbox","windows"]` with `--log-denials` on macOS only and `COMMAND` arg
- Scenario 11: `["execpolicy","check"]` with flags + `COMMAND` arg
- Scenario 12: `["mcp-server"]` and `["app-server"]` (server-mode)

Exactness requirements (normative; tests must enforce):
- For every command path listed in the catalog, emit exactly one command entry with `level: explicit`.
- For each command path, emitted flags/args MUST equal the union of flags/args listed across all scenarios that reference that path.
- Must omit any flag/arg not listed for that path by the catalog.

v1 restrictions (must hold for all emitted units):
- No scope fields anywhere.
- Note policy:
  - `note: "capability-guarded"` only for the exact capability-guarded units listed in the catalog.
  - `intentionally_unsupported` requires a non-empty rationale note (not expected in v1 catalog output, but enforcement must remain).
  - Otherwise omit `note`.

### 2) Enforce parity exclusions (TUI policy)
Files:
- `cli_manifests/codex/RULES.json` (read-only input)
- `crates/xtask/src/codex_wrapper_coverage.rs` (generation-time enforcement)
- `crates/xtask/src/codex_report.rs` (already classifies excluded deltas; verify behavior)

Requirements:
- The generator MUST NOT emit any identity listed in `RULES.json.parity_exclusions.units[]`.
  - If the wrapper-derived manifest contains an excluded identity, `xtask codex-wrapper-coverage` MUST fail with a deterministic error.
- Reports MUST classify excluded identities under:
  - `deltas.excluded_commands`
  - `deltas.excluded_flags`
  - `deltas.excluded_args`
  ...and MUST NOT include excluded identities under the `missing_*` deltas (validator-enforced).

### 3) Refresh committed artifacts for meaningful parity
File:
- `cli_manifests/codex/wrapper_coverage.json`

Integration MUST regenerate and commit an updated `cli_manifests/codex/wrapper_coverage.json` using the new generator (with a deterministic `SOURCE_DATE_EPOCH`).

## Acceptance Criteria

### Catalog completeness and restrictions
- `SOURCE_DATE_EPOCH=0 cargo run -p xtask -- codex-wrapper-coverage --out cli_manifests/codex/wrapper_coverage.json` succeeds and produces a non-empty file that matches the v1 Scenario Catalog exactly (paths + flags + args + notes).
- `cargo run -p xtask -- codex-validate --root cli_manifests/codex` succeeds (includes parity exclusions checks).

### Report semantics (excluded vs missing)
- Let `VERSION="$(tr -d '\\n' < cli_manifests/codex/latest_validated.txt)"`. Running:
  - `SOURCE_DATE_EPOCH=0 cargo run -p xtask -- codex-report --version "$VERSION" --root cli_manifests/codex`
  - `SOURCE_DATE_EPOCH=0 cargo run -p xtask -- codex-validate --root cli_manifests/codex`
  produces reports where identities listed in `RULES.json.parity_exclusions` appear only under `excluded_*` deltas (not under `missing_*`).

### Tests
- Tests under `crates/xtask/tests/` must lock down:
  - Scenario Catalog v1 completeness and exactness against generated wrapper coverage.
  - v1 scope omission and note restriction invariants.
  - parity exclusions: excluded identities are rejected by wrapper coverage generation and never appear as `missing_*` in reports.

## Out of Scope
- Adding new scenarios beyond v1.
- Introducing/using scope fields in wrapper coverage (v1 forbids; requires a new contract).
- Any interactive/TUI support work (explicitly excluded from parity deltas).
