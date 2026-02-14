use claude_code::{ClaudeStreamJsonEvent, ClaudeStreamJsonParseError, ClaudeStreamJsonParser};

use crate::error::{AdapterErrorCode, CapturedRaw};
use crate::line_parser::{ClassifiedParserError, LineInput, LineParser};
use crate::normalized::{
    NormalizationContext, NormalizedEventKind, NormalizedEvents, NormalizedWrapperEvent,
    WrapperAgentKind,
};

#[derive(Debug, Clone, Default)]
pub struct ClaudeCodeLineParser {
    parser: ClaudeStreamJsonParser,
}

impl ClaudeCodeLineParser {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{redacted}")]
pub struct ClaudeCodeLineParserError {
    code: AdapterErrorCode,
    redacted: String,
    details: String,
}

impl ClassifiedParserError for ClaudeCodeLineParserError {
    fn code(&self) -> AdapterErrorCode {
        self.code
    }

    fn redacted_summary(&self) -> String {
        self.redacted.clone()
    }

    fn full_details(&self) -> String {
        self.details.clone()
    }
}

impl LineParser for ClaudeCodeLineParser {
    type Event = ClaudeStreamJsonEvent;
    type Error = ClaudeCodeLineParserError;

    fn reset(&mut self) {
        self.parser.reset();
    }

    fn parse_line(&mut self, input: LineInput<'_>) -> Result<Option<Self::Event>, Self::Error> {
        self.parser
            .parse_line(input.line)
            .map_err(|err| claude_err(err))
    }
}

fn claude_err(err: ClaudeStreamJsonParseError) -> ClaudeCodeLineParserError {
    ClaudeCodeLineParserError {
        code: AdapterErrorCode::JsonParse,
        redacted: format!("parse error: {}", err.message),
        details: err.message,
    }
}

pub fn normalize_claude_code_event(
    line_number: usize,
    context: NormalizationContext,
    captured_raw: Option<CapturedRaw>,
    _event: &ClaudeStreamJsonEvent,
) -> NormalizedEvents {
    NormalizedEvents(vec![NormalizedWrapperEvent {
        line_number,
        agent_kind: WrapperAgentKind::ClaudeCode,
        kind: NormalizedEventKind::Unknown,
        context,
        channel: None,
        captured_raw,
    }])
}
