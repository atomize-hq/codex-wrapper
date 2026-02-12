use serde_json::Value;

use crate::StreamJsonLineError;

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
        match serde_json::from_str::<Value>(&line.raw) {
            Ok(value) => out.push(StreamJsonLineOutcome::Ok { line, value }),
            Err(err) => out.push(StreamJsonLineOutcome::Err {
                line,
                error: StreamJsonLineError {
                    line_number,
                    message: err.to_string(),
                },
            }),
        }
    }
    out
}
