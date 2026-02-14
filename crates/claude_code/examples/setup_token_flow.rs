//! Demonstrates the multi-step `claude setup-token` flow via `ClaudeSetupTokenSession`.
//!
//! Usage:
//! - `CLAUDE_EXAMPLE_LIVE=1 cargo run -p claude_code --example setup_token_flow`
//! - Provide the code via env: `CLAUDE_SETUP_TOKEN_CODE=...`
//! - Or paste the code when prompted.
//!
//! Notes:
//! - This example uses the real `claude` binary and may require network/auth.
//! - Optional isolation: `CLAUDE_EXAMPLE_ISOLATED_HOME=1`

use std::{env, error::Error, io};

use claude_code::ClaudeSetupTokenRequest;

#[path = "support/real_cli.rs"]
mod real_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !real_cli::live_enabled() {
        real_cli::require_live("setup_token_flow")?;
        return Ok(());
    }

    let client = real_cli::maybe_isolated_client("setup_token_flow")?;
    let session = client
        .setup_token_start_with(ClaudeSetupTokenRequest::new().timeout(None))
        .await?;

    println!("Open this URL to authenticate:\n{}", session.url());
    let code = read_code()?;
    let out = session.submit_code(&code).await?;
    println!("exit: {}", out.status);
    print!("{}", String::from_utf8_lossy(&out.stdout));
    eprint!("{}", String::from_utf8_lossy(&out.stderr));
    Ok(())
}

fn read_code() -> Result<String, Box<dyn Error>> {
    if let Ok(code) = env::var("CLAUDE_SETUP_TOKEN_CODE") {
        if !code.trim().is_empty() {
            return Ok(code);
        }
    }

    println!("Paste code here and press Enter:");
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let code = line.trim().to_string();
    if code.is_empty() {
        return Err("Empty code".into());
    }
    Ok(code)
}
