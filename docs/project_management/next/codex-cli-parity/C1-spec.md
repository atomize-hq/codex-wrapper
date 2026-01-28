# C1-spec – Version policy + CI workflows (real-binary validation)

Source: `docs/adr/0001-codex-cli-parity-maintenance.md`

## Decisions (no ambiguity)
- GitHub Actions is the CI/workflow runner for this repo.
- Workflow files (exact paths):
  - Release Watch: `.github/workflows/codex-cli-release-watch.yml`
  - Update Snapshot: `.github/workflows/codex-cli-update-snapshot.yml`
  - CI validation (Linux gate): `.github/workflows/ci.yml`
- Upstream release source: GitHub releases for `openai/codex`.
  - Upstream tag naming convention: `rust-v<semver>` (example: `rust-v0.77.0`).
  - The workflow input `version` is the bare semver (example: `0.77.0`); workflows must map it to `rust-v<version>` when fetching the release.
  - Stability policy (release selection): we only “land” (promote `latest_validated.txt` / snapshots) for **non-prerelease, non-draft** upstream releases.
    - Release Watch must ignore prereleases/drafts.
    - Update Snapshot must fail fast if the requested `version` resolves to a prerelease or draft release in `openai/codex`.
- Supported platform for automated downloads in C1: Linux `x86_64` musl only.
  - The downloaded asset is an archive. Use this exact asset name:
    - `codex-x86_64-unknown-linux-musl.tar.gz`
  - The extracted executable is named `codex`. Place it at this conventional path in the repo workspace (gitignored) before running tests:
    - `./codex-x86_64-unknown-linux-musl`
  - Local multi-version convention (recommended):
    - Store binaries under `./.codex-bins/<version>/codex-x86_64-unknown-linux-musl` (gitignored), and symlink the active one into `./codex-x86_64-unknown-linux-musl`.
  - macOS support is explicitly deferred; Windows is treated as WSL/Linux per ADR.
- Checksum/traceability lockfile (exact path + schema for downloaded artifacts):
  - `cli_manifests/codex/artifacts.lock.json`
  - JSON schema (v1):
    - `version` (int): must be `1`
    - `upstream_repo` (string): must be `openai/codex`
    - `artifacts` (array): stable-sorted by `codex_version` (semver ascending), then `os`, then `arch`
      - `codex_version` (string): semver (example: `0.77.0`)
      - `os` (string): must be `linux`
      - `arch` (string): must be `x86_64`
      - `variant` (string): must be `musl`
      - `asset_name` (string): must be `codex-x86_64-unknown-linux-musl.tar.gz`
      - `download_url` (string)
      - `sha256` (string): checksum of the downloaded archive bytes
      - `size_bytes` (int): size of the downloaded archive bytes

## Task Breakdown (no ambiguity)
- `C1-code` (non-test changes):
  - Create `.github/workflows/ci.yml`, `.github/workflows/codex-cli-release-watch.yml`, `.github/workflows/codex-cli-update-snapshot.yml`.
  - Create `cli_manifests/codex/artifacts.lock.json` (schema v1) and wire Update Snapshot workflow to update it.
- `C1-test` (tests only):
  - Expected no-op unless C1 introduces new testable Rust logic.
- `C1-integ`:
  - Merge `C1-code` + `C1-test`, reconcile to this spec, and run the ADR “validated” commands (with isolated home) plus `make preflight`.

## Workflow Permissions & Implementation Checklist
This section is intentionally copy/paste-friendly to reduce CI ambiguity.

### Minimal `permissions:` blocks (exact)
- `.github/workflows/ci.yml`
  - ```yml
    permissions:
      contents: read
    ```
- `.github/workflows/codex-cli-release-watch.yml`
  - ```yml
    permissions:
      contents: read
      issues: write
    ```
- `.github/workflows/codex-cli-update-snapshot.yml`
  - ```yml
    permissions:
      contents: write
      pull-requests: write
    ```

### Implementation checklist (recommended)
- Release Watch (`codex-cli-release-watch.yml`)
  - Fetch releases from `openai/codex`, filter `prerelease==false` and `draft==false`, parse semver from `tag_name` by stripping `rust-v`.
  - Determine `latest_stable` and `candidate` (= stable-minus-one, or stable if only one exists).
  - Read `cli_manifests/codex/latest_validated.txt`; if `candidate != latest_validated`, create-or-update an issue titled `Codex CLI release watch: candidate <candidate-version>`.
  - Make updates idempotent (search existing open issues by exact title; update body if present, else create).
- Update Snapshot (`codex-cli-update-snapshot.yml`)
  - Trigger: `workflow_dispatch` with inputs `version` and `update_min_supported`.
  - Use `tag = rust-v${{ inputs.version }}` and asset `codex-x86_64-unknown-linux-musl.tar.gz`.
  - Guard: fetch the upstream release and assert `prerelease==false` and `draft==false` before downloading or updating any pointers.
  - Download the archive, compute `sha256` and `size_bytes`, and upsert the entry in `cli_manifests/codex/artifacts.lock.json` for `(version, linux, x86_64, musl)`.
  - Extract `codex` and install it to `./codex-x86_64-unknown-linux-musl` (gitignored), then run the snapshot generator (C0) with `--capture-raw-help`.
  - Update `cli_manifests/codex/latest_validated.txt` to `version` (and `min_supported.txt` only when `update_min_supported==true`).
  - Open a PR (preferred: `peter-evans/create-pull-request`) that commits only `cli_manifests/codex/**` changes (never commit the binary).
  - Add `concurrency:` to prevent overlapping snapshot-update runs on the same repo.
- CI (`ci.yml`)
  - Determine the validated version from `cli_manifests/codex/latest_validated.txt` and require a matching entry in `cli_manifests/codex/artifacts.lock.json` (fail with a clear message if missing).
  - Download/verify/extract to `./codex-x86_64-unknown-linux-musl`, then run the ADR “validated” tests with `CODEX_E2E_HOME` + `CODEX_HOME` isolated.

## Scope
- Enforce the ADR’s version support policy pointers:
  - `cli_manifests/codex/min_supported.txt` is the minimum supported Codex CLI version (single semver line).
  - `cli_manifests/codex/latest_validated.txt` is the newest Codex CLI version validated by our Linux gate (single semver line).
  - These two files are the only authoritative “policy pointers”; `cli_manifests/codex/current.json` is generated for the `latest_validated.txt` version.
- Define “validated” exactly as ADR 0001:
  - Linux gating must include:
    - `cargo test -p codex`
    - `cargo test -p codex --examples`
    - `cargo test -p codex --test cli_e2e` using a supplied real `codex` binary path and a fully isolated `CODEX_HOME`.
  - Optional, non-gating (must remain opt-in via env vars):
    - Live/credentialed probes for `exec`/`resume`/`diff`/`apply`
    - macOS smoke coverage (incremental after Linux baseline)
    - Windows treated as WSL/Linux; native Windows CI can be deferred
- Add CI/workflow automation that **does not** introduce auto-download/auto-update behavior in the crate runtime:
  - Nightly “Release Watch” workflow: read-only upstream check, issue/alert creation, no downloads at runtime.
- Maintainer-triggered “Update Snapshot” workflow (`workflow_dispatch`):
    - Inputs (exact):
      - `version` (string, required): exact Codex CLI semver to validate (example: `0.77.0`)
      - `update_min_supported` (boolean, default `false`): when true, also update `cli_manifests/codex/min_supported.txt` to `version`
    - Downloads the specified Linux release artifact in CI:
      - Tag: `rust-v<version>`
      - Asset name: `codex-x86_64-unknown-linux-musl.tar.gz`
      - Extract executable `codex` and place it at `./codex-x86_64-unknown-linux-musl` for subsequent steps.
    - Records checksums (`sha256` + `size_bytes`) for the downloaded archive in `cli_manifests/codex/artifacts.lock.json` and commits the update in the PR.
    - Runs the snapshot generator (C0) to update:
      - `cli_manifests/codex/current.json`
      - `cli_manifests/codex/raw_help/<version>/**` (enabled in workflow)
    - Updates `cli_manifests/codex/latest_validated.txt` to `version` in the same PR.
    - Opens a PR with snapshot diffs and runs real-binary validations as PR checks (see “Definition of validated”).
    - Implementation note: see “Workflow Permissions & Implementation Checklist” above.
- Document the end-to-end “release watch → snapshot diff → update” process from an operator perspective (high-level; detailed runbook belongs in C3).

### Release Watch workflow details (no ambiguity)
- Schedule: nightly.
- Candidate selection:
  - Fetch latest GitHub releases for `openai/codex`.
  - Filter out prereleases.
  - Sort by semver (descending), parsing semver from tags like `rust-v0.77.0`.
  - Candidate = the *second* newest stable release (stable-minus-one). If only one stable release is available, candidate = newest stable.
- Compare candidate vs the contents of `cli_manifests/codex/latest_validated.txt`.
- If different, open or update a GitHub issue with title:
  - `Codex CLI release watch: candidate <candidate-version>`
- Issue body must include:
  - `latest_validated` (current pointer value)
  - `latest_stable` (newest stable release)
  - `candidate` (stable-minus-one)
  - release URLs for `latest_stable` and `candidate`
  - a short checklist linking to the Update Snapshot workflow
  - Implementation note: see “Workflow Permissions & Implementation Checklist” above.

### CI validation workflow details (no ambiguity)
- `ci.yml` must include a Linux job that runs the ADR “validated” checks against a local binary at `./codex-x86_64-unknown-linux-musl` by setting:
  - `CODEX_E2E_BINARY=./codex-x86_64-unknown-linux-musl`
- The CI job must create a fully isolated home for the run:
  - `CODEX_E2E_HOME=$(mktemp -d)` and `CODEX_HOME=$CODEX_E2E_HOME`
- The CI job must ensure `./codex-x86_64-unknown-linux-musl` exists by downloading and extracting the `latest_validated.txt` version using `cli_manifests/codex/artifacts.lock.json` (do not commit the binary; it is a gitignored workspace artifact).
- The job must run:
  - `cargo test -p codex`
  - `cargo test -p codex --examples`
  - `CODEX_E2E_BINARY=./codex-x86_64-unknown-linux-musl cargo test -p codex --test cli_e2e -- --nocapture`

### Binary acquisition (copy/paste bash for CI/workflows)
Given a bare semver like `0.77.0`, the workflow should:
1. Fetch the `download_url` from `cli_manifests/codex/artifacts.lock.json` for `codex_version==<version>`.
2. Download and verify the archive checksum + size.
3. Extract to `./codex-x86_64-unknown-linux-musl` and `chmod +x`.

Example (Linux; assumes `jq`, `curl`, `sha256sum`, `tar` are available):
- `VERSION=0.77.0`
- `URL=$(jq -r --arg v "$VERSION" '.artifacts[] | select(.codex_version==$v and .os=="linux" and .arch=="x86_64" and .variant=="musl") | .download_url' cli_manifests/codex/artifacts.lock.json)`
- `SHA=$(jq -r --arg v "$VERSION" '.artifacts[] | select(.codex_version==$v and .os=="linux" and .arch=="x86_64" and .variant=="musl") | .sha256' cli_manifests/codex/artifacts.lock.json)`
- `SIZE=$(jq -r --arg v "$VERSION" '.artifacts[] | select(.codex_version==$v and .os=="linux" and .arch=="x86_64" and .variant=="musl") | .size_bytes' cli_manifests/codex/artifacts.lock.json)`
- `curl -fsSL -o codex.tgz "$URL"`
- `test "$(wc -c < codex.tgz)" = "$SIZE"`
- `echo "$SHA  codex.tgz" | sha256sum -c -`
- `tar -xzf codex.tgz codex`
- `install -m 0755 codex ./codex-x86_64-unknown-linux-musl`
- `./codex-x86_64-unknown-linux-musl --version`

## Acceptance Criteria
- `cli_manifests/codex/min_supported.txt` and `cli_manifests/codex/latest_validated.txt` exist and are treated as the only authoritative policy pointers in docs and workflows.
- CI has a Linux gate that runs the ADR “validated” test set against at least the pinned “latest validated” binary.
- Optional CI support exists (or is explicitly deferred with rationale in docs/spec) for:
  - validating the “minimum supported” binary, and
  - macOS smoke checks.
- Two workflows exist at the exact paths specified in “Decisions (no ambiguity)”:
  - `.github/workflows/codex-cli-release-watch.yml`
  - `.github/workflows/codex-cli-update-snapshot.yml`
- `cli_manifests/codex/artifacts.lock.json` exists and is updated by Update Snapshot PRs.

## Out of Scope
- Implementing new wrapper surfaces for newly discovered commands/flags (that work follows snapshot diffs and is handled in separate triads).
- Any crate-runtime behavior that downloads, installs, or updates `codex` binaries.
- Full native Windows CI parity (WSL/Linux is sufficient for initial policy).
