use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaudeOutputFormat {
    Text,
    Json,
    StreamJson,
}

impl ClaudeOutputFormat {
    pub(crate) fn as_arg_value(&self) -> &'static str {
        match self {
            ClaudeOutputFormat::Text => "text",
            ClaudeOutputFormat::Json => "json",
            ClaudeOutputFormat::StreamJson => "stream-json",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaudeInputFormat {
    Text,
    StreamJson,
}

impl ClaudeInputFormat {
    pub(crate) fn as_arg_value(&self) -> &'static str {
        match self {
            ClaudeInputFormat::Text => "text",
            ClaudeInputFormat::StreamJson => "stream-json",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClaudePrintRequest {
    pub(crate) prompt: Option<String>,
    pub(crate) stdin: Option<Vec<u8>>,
    pub(crate) output_format: ClaudeOutputFormat,
    pub(crate) input_format: Option<ClaudeInputFormat>,
    pub(crate) json_schema: Option<String>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) extra_args: Vec<String>,
}

impl ClaudePrintRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: Some(prompt.into()),
            stdin: None,
            output_format: ClaudeOutputFormat::Text,
            input_format: None,
            json_schema: None,
            timeout: None,
            extra_args: Vec::new(),
        }
    }

    pub fn output_format(mut self, format: ClaudeOutputFormat) -> Self {
        self.output_format = format;
        self
    }

    pub fn input_format(mut self, format: ClaudeInputFormat) -> Self {
        self.input_format = Some(format);
        self
    }

    pub fn json_schema(mut self, schema: impl Into<String>) -> Self {
        self.json_schema = Some(schema.into());
        self
    }

    pub fn stdin_bytes(mut self, bytes: Vec<u8>) -> Self {
        self.stdin = Some(bytes);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn extra_args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.extra_args = args.into_iter().map(Into::into).collect();
        self
    }

    pub fn argv(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        out.push("--print".to_string());
        out.push("--output-format".to_string());
        out.push(self.output_format.as_arg_value().to_string());

        if let Some(input_format) = self.input_format {
            out.push("--input-format".to_string());
            out.push(input_format.as_arg_value().to_string());
        }

        if let Some(schema) = self.json_schema.as_ref() {
            out.push("--json-schema".to_string());
            out.push(schema.clone());
        }

        out.extend(self.extra_args.iter().cloned());

        if let Some(prompt) = self.prompt.as_ref() {
            out.push(prompt.clone());
        }

        out
    }
}
