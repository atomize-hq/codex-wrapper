# Rust Audit Pack

Self-contained audit outputs for this Rust codebase.

## Entry points
- Start here: `audit_pack/README.md`
- Provenance: `audit_pack/meta/commands.log` (timestamp | cwd | command | exit)
- Environment: `audit_pack/meta/environment.txt`, `audit_pack/meta/versions.txt`
- Failures: `audit_pack/failures/failed_steps_summary.md`

## Contents
- `meta/`: timestamps, environment, versions, git info, command log
- `repo/`: structure and config discovery
- `metrics/`: LOC (tokei) + file sizes
- `build/`: cargo check/test logs
- `lint/`: cargo fmt --check + clippy logs
- `deps/`: cargo tree outputs
- `supply_chain/`: cargo audit + cargo deny outputs
- `optional/`: geiger/machete/udeps (best-effort)

## Notes
- Build artifacts redirected to `audit_pack/target` via `CARGO_TARGET_DIR`.

## Tools

No additional Rust CLI tools were installed during this run.

Tool presence summary:
```
cargo-fmt(rustfmt): present
cargo-clippy(clippy): present
tokei: present
cargo-tree: present
cargo-audit: present
cargo-deny: present
cargo-geiger: missing -> attempting cargo install cargo-geiger
cargo-geiger: install failed
cargo-machete: present
cargo-udeps: missing -> attempting cargo install cargo-udeps
cargo-udeps: install failed
```

## Failures/Skips
- cargo install cargo-geiger (exit 101): audit_pack/meta/install_cargo-geiger.txt
- cargo install cargo-udeps (exit 101): audit_pack/meta/install_cargo-udeps.txt
- cargo audit (exit 1): audit_pack/supply_chain/cargo_audit.txt
- cargo deny check advisories (exit 1): audit_pack/supply_chain/cargo_deny_advisories.txt
- cargo deny check licenses (exit 4): audit_pack/supply_chain/cargo_deny_licenses.txt

## Handoff snippet
Use `audit_pack/` as your input; start at `audit_pack/README.md`; refer to `audit_pack/meta/commands.log` for provenance.

