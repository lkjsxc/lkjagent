use crate::error::{ToolError, ToolResult};

const PHRASES: &[(&str, &str)] = &[
    ("replace this skeleton", "skeleton"),
    ("add the requested substance", "requested-substance"),
    (
        "real cookbook content before dispatch",
        "scaffold-instruction",
    ),
    ("real story content before dispatch", "scaffold-instruction"),
    (
        "real artifact content before dispatch",
        "scaffold-instruction",
    ),
    (
        "requested substance details and verification notes here",
        "requested-substance",
    ),
    ("coming soon", "status-only"),
    ("to be written", "status-only"),
    ("this file records", "generic-record"),
    ("this section describes", "generic-description"),
    ("placeholder content", "placeholder"),
    ("stub content", "stub"),
    ("scaffold only", "scaffold-only"),
    ("future work", "future-work"),
    ("table of contents without body", "empty-toc"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceholderHit {
    pub phrase: &'static str,
    pub class: &'static str,
}

pub fn detect(text: &str) -> Option<&'static str> {
    detect_hit(text).map(|hit| hit.phrase)
}

pub fn detect_hit(text: &str) -> Option<PlaceholderHit> {
    let normalized = normalize(text);
    PHRASES
        .iter()
        .find(|(phrase, _)| normalized.contains(phrase))
        .map(|(phrase, class)| PlaceholderHit { phrase, class })
}

pub fn reject_for_path(path: &str, text: &str) -> ToolResult<()> {
    if let Some(hit) = detect_hit(text) {
        return Err(ToolError::invalid(format!(
            "scaffold phrase refused\npath={path}\nphrase={}\nphrase_class={}\nwhy=scaffold-like generic or status-only prose is not content readiness\nacceptable_replacement=use concrete names, procedures, examples, paths, commands, invariants, observed facts, or domain-specific details",
            hit.phrase, hit.class
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
