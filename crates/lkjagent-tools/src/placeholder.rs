use crate::error::{ToolError, ToolResult};

const PHRASES: &[&str] = &[
    "replace this skeleton",
    "add the requested substance",
    "real cookbook content before dispatch",
    "real story content before dispatch",
    "real artifact content before dispatch",
    "requested substance details and verification notes here",
];

pub fn detect(text: &str) -> Option<&'static str> {
    let normalized = normalize(text);
    PHRASES
        .iter()
        .copied()
        .find(|phrase| normalized.contains(phrase))
}

pub fn reject(text: &str) -> ToolResult<()> {
    if let Some(phrase) = detect(text) {
        return Err(ToolError::invalid(format!(
            "scaffold phrase refused: {phrase}"
        )));
    }
    Ok(())
}

fn normalize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last_space = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_space = false;
        } else if !last_space {
            out.push(' ');
            last_space = true;
        }
    }
    out.trim().to_string()
}
