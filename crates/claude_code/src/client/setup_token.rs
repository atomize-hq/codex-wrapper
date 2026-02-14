use std::{sync::Arc, time::Duration};

use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::Command,
    sync::{oneshot, Mutex},
    task, time,
};

use super::ClaudeClient;
use crate::{
    commands::setup_token::ClaudeSetupTokenRequest,
    process::{self, ConsoleTarget},
    ClaudeCodeError, CommandOutput,
};

#[derive(Debug)]
pub struct ClaudeSetupTokenSession {
    url: String,
    url_rx: Option<oneshot::Receiver<String>>,
    child: tokio::process::Child,
    stdin: Option<tokio::process::ChildStdin>,
    stdout_buf: Arc<Mutex<Vec<u8>>>,
    stderr_buf: Arc<Mutex<Vec<u8>>>,
    stdout_task: Option<tokio::task::JoinHandle<Result<(), ClaudeCodeError>>>,
    stderr_task: Option<tokio::task::JoinHandle<Result<(), ClaudeCodeError>>>,
    timeout: Option<Duration>,
}

impl ClaudeSetupTokenSession {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub async fn wait_for_url(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<&str>, ClaudeCodeError> {
        if !self.url.is_empty() {
            return Ok(Some(self.url.as_str()));
        }

        let Some(rx) = self.url_rx.as_mut() else {
            return Ok(None);
        };

        match time::timeout(timeout, rx).await {
            Ok(Ok(url)) => {
                self.url = url;
                self.url_rx = None;
                Ok(Some(self.url.as_str()))
            }
            Ok(Err(_closed)) => {
                self.url_rx = None;
                Ok(None)
            }
            Err(_timeout) => Ok(None),
        }
    }

    pub async fn submit_code(mut self, code: &str) -> Result<CommandOutput, ClaudeCodeError> {
        if let Some(mut stdin) = self.stdin.take() {
            let mut bytes = code.as_bytes().to_vec();
            if !bytes.ends_with(b"\n") {
                bytes.push(b'\n');
            }
            stdin
                .write_all(&bytes)
                .await
                .map_err(ClaudeCodeError::StdinWrite)?;
        }
        self.wait().await
    }

    pub async fn wait(mut self) -> Result<CommandOutput, ClaudeCodeError> {
        let wait_fut = self.child.wait();
        let status = if let Some(dur) = self.timeout {
            time::timeout(dur, wait_fut)
                .await
                .map_err(|_| ClaudeCodeError::Timeout { timeout: dur })?
                .map_err(ClaudeCodeError::Wait)?
        } else {
            wait_fut.await.map_err(ClaudeCodeError::Wait)?
        };

        if let Some(task) = self.stdout_task.take() {
            task.await
                .map_err(|e| ClaudeCodeError::Join(e.to_string()))??;
        }
        if let Some(task) = self.stderr_task.take() {
            task.await
                .map_err(|e| ClaudeCodeError::Join(e.to_string()))??;
        }

        let stdout = self.stdout_buf.lock().await.clone();
        let stderr = self.stderr_buf.lock().await.clone();

        Ok(CommandOutput {
            status,
            stdout,
            stderr,
        })
    }
}

impl Drop for ClaudeSetupTokenSession {
    fn drop(&mut self) {
        // Best-effort cleanup; if the session is dropped before completion, avoid leaving
        // an interactive `claude setup-token` process running.
        if self.child.id().is_some() {
            let _ = self.child.start_kill();
        }
    }
}

impl ClaudeClient {
    pub async fn setup_token_start(&self) -> Result<ClaudeSetupTokenSession, ClaudeCodeError> {
        self.setup_token_start_with(ClaudeSetupTokenRequest::new())
            .await
    }

    pub async fn setup_token_start_with(
        &self,
        request: ClaudeSetupTokenRequest,
    ) -> Result<ClaudeSetupTokenSession, ClaudeCodeError> {
        let requested_timeout = request.timeout;
        let binary = self.resolve_binary();
        let mut cmd = Command::new(&binary);
        cmd.args(request.into_command().argv());

        if let Some(dir) = self.working_dir.as_ref() {
            cmd.current_dir(dir);
        }

        process::apply_env(&mut cmd, &self.env);

        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        let mut child = process::spawn_with_retry(&mut cmd, &binary)?;
        let stdin = child.stdin.take();
        let stdout = child.stdout.take().ok_or(ClaudeCodeError::MissingStdout)?;
        let stderr = child.stderr.take().ok_or(ClaudeCodeError::MissingStderr)?;

        let stdout_buf = Arc::new(Mutex::new(Vec::<u8>::new()));
        let stderr_buf = Arc::new(Mutex::new(Vec::<u8>::new()));

        let (url_tx, url_rx) = oneshot::channel::<String>();
        let url_tx = Arc::new(Mutex::new(Some(url_tx)));
        let url_state = Arc::new(Mutex::new(UrlCapture::default()));

        let stdout_task = spawn_capture_task(
            stdout,
            ConsoleTarget::Stdout,
            self.mirror_stdout,
            stdout_buf.clone(),
            url_state.clone(),
            url_tx.clone(),
        );
        let stderr_task = spawn_capture_task(
            stderr,
            ConsoleTarget::Stderr,
            self.mirror_stderr,
            stderr_buf.clone(),
            url_state.clone(),
            url_tx.clone(),
        );

        Ok(ClaudeSetupTokenSession {
            url: String::new(),
            url_rx: Some(url_rx),
            child,
            stdin,
            stdout_buf,
            stderr_buf,
            stdout_task: Some(stdout_task),
            stderr_task: Some(stderr_task),
            timeout: requested_timeout.or(self.timeout),
        })
    }
}

#[derive(Debug, Default)]
struct UrlCapture {
    buffer: String,
    found: Option<String>,
}

impl UrlCapture {
    fn push_text(&mut self, chunk: &str) -> Option<String> {
        if self.found.is_some() {
            return None;
        }

        if self.buffer.len() > 64 * 1024 {
            // Avoid unbounded growth in the unlikely event the command is chatty before printing
            // the URL.
            self.buffer = self
                .buffer
                .chars()
                .rev()
                .take(16 * 1024)
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>();
        }

        self.buffer.push_str(chunk);
        if let Some(url) = extract_oauth_url(&self.buffer) {
            self.found = Some(url.clone());
            return Some(url);
        }
        None
    }
}

fn extract_oauth_url(text: &str) -> Option<String> {
    let start = text.find("https://claude.ai/oauth/authorize?")?;
    let tail = &text[start..];

    let mut stop = tail.len();
    if let Some(idx) = tail.find("\n\n") {
        stop = stop.min(idx);
    }
    if let Some(idx) = tail.find("Paste code") {
        stop = stop.min(idx);
    }
    if let Some(idx) = tail.find("Paste") {
        stop = stop.min(idx);
    }

    let raw = &tail[..stop];
    let flattened: String = raw.split_whitespace().collect();
    flattened
        .starts_with("https://claude.ai/oauth/authorize?")
        .then_some(flattened)
}

fn spawn_capture_task<R>(
    reader: R,
    target: ConsoleTarget,
    mirror_console: bool,
    out: Arc<Mutex<Vec<u8>>>,
    url_state: Arc<Mutex<UrlCapture>>,
    url_tx: Arc<Mutex<Option<oneshot::Sender<String>>>>,
) -> tokio::task::JoinHandle<Result<(), ClaudeCodeError>>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut reader = reader;
        let mut chunk = [0u8; 4096];
        loop {
            let n = reader.read(&mut chunk).await.map_err(|e| match target {
                ConsoleTarget::Stdout => ClaudeCodeError::StdoutRead(e),
                ConsoleTarget::Stderr => ClaudeCodeError::StderrRead(e),
            })?;
            if n == 0 {
                break;
            }

            if mirror_console {
                task::block_in_place(|| {
                    let mut w: Box<dyn std::io::Write> = match target {
                        ConsoleTarget::Stdout => Box::new(std::io::stdout()),
                        ConsoleTarget::Stderr => Box::new(std::io::stderr()),
                    };
                    w.write_all(&chunk[..n])?;
                    w.flush()
                })
                .map_err(|e| match target {
                    ConsoleTarget::Stdout => ClaudeCodeError::StdoutRead(e),
                    ConsoleTarget::Stderr => ClaudeCodeError::StderrRead(e),
                })?;
            }

            out.lock().await.extend_from_slice(&chunk[..n]);

            let text = String::from_utf8_lossy(&chunk[..n]);
            if let Some(url) = url_state.lock().await.push_text(&text) {
                if let Some(tx) = url_tx.lock().await.take() {
                    let _ = tx.send(url);
                }
            }
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::extract_oauth_url;

    #[test]
    fn extracts_wrapped_url_from_setup_token_output() {
        let text = r#"
Browser didn't open? Use the url below to sign in (c to copy)

https://claude.ai/oauth/authorize?code=true&client_id=abc&response_type=c
ode&redirect_uri=https%3A%2F%2Fplatform.claude.com%2Foauth%2Fcode%2Fcallback&scope=user%3Ainference

Paste code here if prompted >
"#;

        let url = extract_oauth_url(text).expect("url");
        assert!(url.starts_with("https://claude.ai/oauth/authorize?"));
        assert!(url.contains("client_id=abc"));
        assert!(url.contains("response_type=code"));
    }
}
