#![forbid(unsafe_code)]
//! Async helper around the Claude Code CLI (`claude`) focused on the headless `--print` flow.
//!
//! This crate intentionally does **not** attempt to wrap interactive default mode (no `--print`)
//! as a parity target. It shells out to a locally installed/pinned `claude` binary.

mod client;
mod error;
mod process;
mod request;
mod stream_json;
pub mod wrapper_coverage_manifest;

pub use client::{ClaudeClient, ClaudeClientBuilder};
pub use error::{ClaudeCodeError, StreamJsonLineError};
pub use request::{ClaudeInputFormat, ClaudeOutputFormat, ClaudePrintRequest};
pub use stream_json::{parse_stream_json_lines, StreamJsonLine, StreamJsonLineOutcome};

pub use process::CommandOutput;
