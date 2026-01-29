# ADR 0003: Deterministic Auto‑Generation of Wrapper Coverage (No Handwritten Mapping)

Date: 2026-01-29  
Status: Proposed

## Context

ADR 0002 (“Snapshot → Coverage → Work Queue”) established a parity workflow:

1. CI runs upstream `codex` binaries to generate per-target snapshots and a merged union snapshot (`cli_manifests/codex/snapshots/<version>/union.json`).
2. CI generates a wrapper coverage inventory (`cli_manifests/codex/wrapper_coverage.json`) describing what `crates/codex` supports at the command/flag/arg level.
3. CI compares upstream snapshot(s) to wrapper coverage to produce deterministic coverage reports (`cli_manifests/codex/reports/<version>/coverage.*.json`) that become an actionable work queue.

The upstream side is working end-to-end: CI can download upstream binaries, generate snapshots, merge a union, generate reports, and validate artifact invariants.

However, wrapper coverage is currently not meaningful:

- `xtask codex-wrapper-coverage` generates `cli_manifests/codex/wrapper_coverage.json`.
- The generator’s only input is `codex::wrapper_coverage_manifest::wrapper_coverage_manifest()` from `crates/codex/src/wrapper_coverage_manifest.rs`.
- That function currently returns `coverage: Vec::new()`, so the generated JSON contains `"coverage": []`.
- Under `cli_manifests/codex/RULES.json`, missing wrapper entries are treated as `unknown`, which causes reports for new upstream versions to show nearly everything as missing/unknown even when the wrapper already supports many surfaces.

Clarification: `cli_manifests/codex/current.json` is generated from the upstream `codex` binary (it must match `snapshots/<latest_validated>/union.json`), not from the wrapper. Wrapper support must be reflected via wrapper coverage artifacts, not via `current.json`.

This contradicts the operational goal: CI should highlight *delta work* for a new upstream release (new/changed surfaces), not rediscover the entire CLI as “unsupported”.

## Problem

We need a deterministic mechanism to generate accurate wrapper coverage automatically from the wrapper implementation signals, without requiring humans to maintain a handwritten command/flag/arg inventory (in JSON or ad hoc mapping tables).

## Decision

Implement **deterministic auto-generation of wrapper coverage** such that:

- `cli_manifests/codex/wrapper_coverage.json` is produced mechanically from `crates/codex` implementation signals.
- The output is deterministic (stable ordering; timestamps controlled via `SOURCE_DATE_EPOCH`; no nondeterministic discovery).
- The generator is offline (no network access; no runtime upstream binary downloads).
- The output validates against existing contracts:
  - shape validation per `cli_manifests/codex/SCHEMA.json` (`WrapperCoverageV1`)
  - scope semantics and resolution per `cli_manifests/codex/RULES.json.wrapper_coverage`
  - invariants per `cli_manifests/codex/VALIDATOR_SPEC.md` (including rationale note requirements for `intentionally_unsupported`)

The auto-generation approach may be implemented via:
- deterministic static analysis of wrapper source, and/or
- deterministic wrapper “probe”/introspection outputs, and/or
- a hybrid approach,

but it must not rely on manual hand-curation of the command/flag/arg inventory as a long-term source of truth.

## What Is Already Specified (No Contract Changes Required)

The following are already sufficient to specify artifact shapes and comparison semantics:

- `cli_manifests/codex/SCHEMA.json` (snapshots, wrapper coverage, reports)
- `cli_manifests/codex/RULES.json` (scope resolution, report semantics, “supported” policy)
- `cli_manifests/codex/VERSION_METADATA_SCHEMA.json` (version metadata shape)
- `docs/adr/0002-codex-cli-parity-coverage-mapping.md` (system intent and constraints)

This ADR does not require changes to these contracts by default.

## What Is Not Yet Specified (Requires a Generator Contract)

To implement auto-generation safely and deterministically, we must define:

- The exact derivation algorithm(s) (static analysis vs probe vs hybrid).
- How to map wrapper implementation signals to upstream identities:
  - command `path` (`[]`, `["exec"]`, `["login","status"]`, …)
  - flag identity `key` (canonical `--long` or `-s`)
  - arg identity `name` (help-derived positional arg name)
- How to classify `explicit` vs `passthrough` automatically (what proof/signal constitutes each).
- How to handle root/global flags and command-local flags without over/under counting.
- How to model platform/target scoping (`scope.platforms` / `scope.target_triples`) where necessary.
- How to handle feature-gated or intentionally unsupported surfaces deterministically (including note text stability).

## Consequences

### Benefits

- Coverage reports become actionable deltas for new upstream versions rather than “everything missing”.
- CI can reliably produce a work queue to add support, deprecate/adjust old surfaces, or explicitly waive surfaces with policy rationale.

### Tradeoffs / Risks

- Auto-derivation is non-trivial; static analysis can be brittle, probes can be incomplete, and hybrids add complexity.
- False positives/negatives can mislead the work queue; tests must lock behavior down.

## Follow-ups

- Write a “Wrapper Coverage Generator Contract” spec (inputs, algorithm, determinism guarantees, acceptance tests).
- Implement the generator and accompanying tests under `crates/xtask/tests/`.
- Optionally update ADR 0002 with a concrete generator contract section once the approach is selected and stabilized.

