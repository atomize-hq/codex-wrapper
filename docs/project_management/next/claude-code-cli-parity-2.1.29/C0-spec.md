# C0-spec – Parity update for `claude_code` `2.1.29`

## Scope
- Use the generated coverage report as the work queue.
- Report: `cli_manifests/claude_code/reports/2.1.29/coverage.any.json`
- Implement wrapper support or explicitly waive with `intentionally_unsupported` notes.
- Regenerate artifacts and pass `codex-validate` for the parity root.

### Missing commands
- `mcp`
- `mcp list`
- `mcp reset-project-choices`
- `plugin`
- `plugin manifest`
- `plugin manifest marketplace`
- `plugin marketplace`
- `plugin marketplace repo`
- `setup-token`

### Missing flags
- `<root> --add-dir`
- `<root> --agent`
- `<root> --agents`
- `<root> --allow-dangerously-skip-permissions`
- `<root> --allowedTools`
- `<root> --append-system-prompt`
- `<root> --betas`
- `<root> --chrome`
- `<root> --continue`
- `<root> --dangerously-skip-permissions`
- `<root> --debug`
- `<root> --debug-file`
- `<root> --disable-slash-commands`
- `<root> --disallowedTools`
- `<root> --fallback-model`
- `<root> --file`
- `<root> --fork-session`
- `<root> --from-pr`
- `<root> --ide`
- `<root> --include-partial-messages`
- `<root> --max-budget-usd`
- `<root> --mcp-config`
- `<root> --mcp-debug`
- `<root> --model`
- `<root> --no-chrome`
- `<root> --no-session-persistence`
- `<root> --permission-mode`
- `<root> --plugin-dir`
- `<root> --replay-user-messages`
- `<root> --resume`
- `<root> --session-id`
- `<root> --setting-sources`
- `<root> --settings`
- `<root> --strict-mcp-config`
- `<root> --system-prompt`
- `<root> --tools`
- `<root> --verbose`

### Missing args
- (none)

## Acceptance Criteria
- Wrapper changes address C0 scope.
- Artifacts regenerated deterministically.
- `cargo run -p xtask -- codex-validate --root <root>` passes.

## Out of Scope
- Promotion (pointer/current.json updates) unless explicitly requested.
