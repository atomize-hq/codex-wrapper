# Codex Rust Wrapper

Async wrapper around the Codex CLI focused on the headless `codex exec` flow. The client shells out to the bundled or system Codex binary, mirrors stdout/stderr when asked, and keeps the parent process environment untouched.

## Binary and `CODEX_HOME` isolation

- Point the wrapper at a bundled Codex binary via [`CodexClientBuilder::binary`]; if unset, it honors `CODEX_BINARY` or falls back to `codex` on `PATH`.
- Apply an app-scoped home with [`CodexClientBuilder::codex_home`]. The resolved binary is mirrored into `CODEX_BINARY`, and the provided home is exported as `CODEX_HOME` for every spawn site (exec/login/status/logout). The parent environment is never mutated.
- Use [`CodexClientBuilder::create_home_dirs`] to control whether `CODEX_HOME`, `conversations/`, and `logs/` are created up front (defaults to `true` when a home is set). `RUST_LOG` defaults to `error` if you have not set it.

```rust
use codex::{CodexClient, CodexHomeLayout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary = "/opt/myapp/bin/codex";
    let codex_home = "/var/lib/myapp/codex";

    // Discover (and optionally create) the CODEX_HOME layout.
    let layout = CodexHomeLayout::new(codex_home);
    layout.materialize(true)?;
    println!("Logs live at {}", layout.logs_dir().display());

    let client = CodexClient::builder()
        .binary(binary)
        .codex_home(codex_home)
        .create_home_dirs(true)
        .mirror_stdout(false)
        .quiet(true)
        .build();

    let reply = client.send_prompt("Health check").await?;
    println!("{reply}");
    Ok(())
}
```

## `CODEX_HOME` layout helper

`CodexHomeLayout` documents where Codex stores state under an app-scoped home:

- `config.toml`
- `auth.json`
- `.credentials.json`
- `history.jsonl`
- `conversations/` for transcript JSONL files
- `logs/` for `codex-*.log` files

Call [`CodexHomeLayout::materialize`] to create the root, `conversations/`, and `logs/` directories before spawning Codex.

## Examples

See `crates/codex/EXAMPLES.md` for one-to-one CLI parity examples, including `bundled_binary_home` to run Codex from an embedded binary with isolated state.
