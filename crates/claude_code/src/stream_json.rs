use serde_json::Value;

use crate::StreamJsonLineError;

#[derive(Debug, Clone)]
pub struct ClaudeStreamJsonEvent {
    pub value: Value,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub struct ClaudeStreamJsonParseError {
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct ClaudeStreamJsonParser;

impl ClaudeStreamJsonParser {
    pub fn new() -> Self {
        Self
    }

    pub fn reset(&mut self) {}

    pub fn parse_line(
        &mut self,
        line: &str,
    ) -> Result<Option<ClaudeStreamJsonEvent>, ClaudeStreamJsonParseError> {
        let line = line.trim_end_matches('\r');
        if line.chars().all(|ch| ch.is_whitespace()) {
            return Ok(None);
        }

        serde_json::from_str::<Value>(line)
            .map(|value| Some(ClaudeStreamJsonEvent { value }))
            .map_err(|err| ClaudeStreamJsonParseError {
                message: err.to_string(),
            })
    }
}

#[derive(Debug, Clone)]
pub struct StreamJsonLine {
    pub line_number: usize,
    pub raw: String,
}

#[derive(Debug, Clone)]
pub enum StreamJsonLineOutcome {
    Ok {
        line: StreamJsonLine,
        value: Value,
    },
    Err {
        line: StreamJsonLine,
        error: StreamJsonLineError,
    },
}

pub fn parse_stream_json_lines(text: &str) -> Vec<StreamJsonLineOutcome> {
    let mut out = Vec::new();
    let mut parser = ClaudeStreamJsonParser::new();
    for (idx, raw) in text.lines().enumerate() {
        let line_number = idx + 1;
        let raw = raw.trim_end_matches('\r');
        if raw.trim().is_empty() {
            continue;
        }
        let line = StreamJsonLine {
            line_number,
            raw: raw.to_string(),
        };
        match parser.parse_line(&line.raw) {
            Ok(Some(event)) => out.push(StreamJsonLineOutcome::Ok {
                line,
                value: event.value,
            }),
            Ok(None) => {}
            Err(err) => out.push(StreamJsonLineOutcome::Err {
                line,
                error: StreamJsonLineError {
                    line_number,
                    message: err.message,
                },
            }),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_is_tolerant_and_line_oriented() {
        let mut parser = ClaudeStreamJsonParser::new();

        assert!(parser.parse_line("   ").unwrap().is_none());
        assert!(parser.parse_line("{\"k\":1}").unwrap().is_some());
        assert!(parser.parse_line("{not-json}").is_err());
        assert!(parser.parse_line("{\"k\":2}").unwrap().is_some());
    }
}
