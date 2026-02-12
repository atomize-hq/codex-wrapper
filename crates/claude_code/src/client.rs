use std::{collections::BTreeMap, path::PathBuf, time::Duration};

use tokio::process::Command;

use crate::{
    parse_stream_json_lines, process, ClaudeCodeError, ClaudeOutputFormat, ClaudePrintRequest,
    CommandOutput, StreamJsonLineOutcome,
};

#[derive(Debug, Clone)]
pub struct ClaudeClientBuilder {
    binary: Option<PathBuf>,
    working_dir: Option<PathBuf>,
    env: BTreeMap<String, String>,
    timeout: Option<Duration>,
    mirror_stdout: bool,
    mirror_stderr: bool,
}

impl Default for ClaudeClientBuilder {
    fn default() -> Self {
        Self {
            binary: None,
            working_dir: None,
            env: BTreeMap::new(),
            timeout: Some(Duration::from_secs(120)),
            mirror_stdout: false,
            mirror_stderr: false,
        }
    }
}

impl ClaudeClientBuilder {
    pub fn binary(mut self, binary: impl Into<PathBuf>) -> Self {
        self.binary = Some(binary.into());
        self
    }

    pub fn working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    pub fn timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn mirror_stdout(mut self, enabled: bool) -> Self {
        self.mirror_stdout = enabled;
        self
    }

    pub fn mirror_stderr(mut self, enabled: bool) -> Self {
        self.mirror_stderr = enabled;
        self
    }

    pub fn build(mut self) -> ClaudeClient {
        // Avoid any updater side effects by default; callers may override explicitly.
        self.env
            .entry("DISABLE_AUTOUPDATER".to_string())
            .or_insert_with(|| "1".to_string());

        ClaudeClient {
            binary: self.binary,
            working_dir: self.working_dir,
            env: self.env,
            timeout: self.timeout,
            mirror_stdout: self.mirror_stdout,
            mirror_stderr: self.mirror_stderr,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClaudeClient {
    binary: Option<PathBuf>,
    working_dir: Option<PathBuf>,
    env: BTreeMap<String, String>,
    timeout: Option<Duration>,
    mirror_stdout: bool,
    mirror_stderr: bool,
}

impl ClaudeClient {
    pub fn builder() -> ClaudeClientBuilder {
        ClaudeClientBuilder::default()
    }

    pub async fn print(
        &self,
        request: ClaudePrintRequest,
    ) -> Result<ClaudePrintResult, ClaudeCodeError> {
        if request.prompt.is_none() && request.stdin.is_none() {
            return Err(ClaudeCodeError::InvalidRequest(
                "either prompt or stdin_bytes must be provided".to_string(),
            ));
        }

        let binary = self.resolve_binary();
        let mut cmd = Command::new(&binary);
        cmd.args(request.argv());

        if let Some(dir) = self.working_dir.as_ref() {
            cmd.current_dir(dir);
        }

        process::apply_env(&mut cmd, &self.env);

        let timeout = request.timeout.or(self.timeout);
        let output = process::run_command(
            cmd,
            &binary,
            request.stdin.as_deref(),
            timeout,
            self.mirror_stdout,
            self.mirror_stderr,
        )
        .await?;

        let parsed = match request.output_format {
            ClaudeOutputFormat::Json => {
                let v = serde_json::from_slice(&output.stdout)?;
                Some(ClaudeParsedOutput::Json(v))
            }
            ClaudeOutputFormat::StreamJson => {
                let s = String::from_utf8_lossy(&output.stdout);
                Some(ClaudeParsedOutput::StreamJson(parse_stream_json_lines(&s)))
            }
            ClaudeOutputFormat::Text => None,
        };

        Ok(ClaudePrintResult { output, parsed })
    }

    fn resolve_binary(&self) -> PathBuf {
        if let Some(b) = self.binary.as_ref() {
            return b.clone();
        }
        if let Ok(v) = std::env::var("CLAUDE_BINARY") {
            if !v.trim().is_empty() {
                return PathBuf::from(v);
            }
        }
        PathBuf::from("claude")
    }
}

#[derive(Debug, Clone)]
pub struct ClaudePrintResult {
    pub output: CommandOutput,
    pub parsed: Option<ClaudeParsedOutput>,
}

#[derive(Debug, Clone)]
pub enum ClaudeParsedOutput {
    Json(serde_json::Value),
    StreamJson(Vec<StreamJsonLineOutcome>),
}
