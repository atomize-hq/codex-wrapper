mod codex_snapshot;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "Project automation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate a Codex CLI snapshot manifest under `cli_manifests/codex/`.
    CodexSnapshot(codex_snapshot::Args),
}

fn main() -> Result<(), codex_snapshot::Error> {
    let cli = Cli::parse();
    match cli.command {
        Command::CodexSnapshot(args) => codex_snapshot::run(args),
    }
}
