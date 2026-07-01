use std::fs;
use std::path::Path;

use lkjagent_store::artifact_graph::{split_lines, AtomRow};

use crate::artifact_story_text::prose_words;
use crate::error::ToolResult;

pub struct Measurement {
    pub count: usize,
    pub weak_classes: Vec<String>,
    pub status: String,
    pub summary: String,
}

pub fn measure_atom(root: &Path, atom: &AtomRow) -> ToolResult<Measurement> {
    let path = root.join(&atom.path);
    if !path.is_file() {
        return Ok(measurement(0, vec!["missing-file".to_string()], "missing"));
    }
    let text = fs::read_to_string(&path)?;
    let count = measure(&text, &atom.measurement_kind);
    let weak = weak_classes(&text, atom, count);
    let status = if weak.is_empty() { "ready" } else { "weak" };
    Ok(measurement(count, weak, status))
}

fn measurement(count: usize, weak_classes: Vec<String>, status: &str) -> Measurement {
    let summary = if weak_classes.is_empty() {
        format!("measured_count={count} weak=none")
    } else {
        format!("measured_count={count} weak={}", weak_classes.join(","))
    };
    Measurement {
        count,
        weak_classes,
        status: status.to_string(),
        summary,
    }
}

fn measure(text: &str, kind: &str) -> usize {
    match kind {
        "characters" => text.chars().filter(|ch| !ch.is_whitespace()).count(),
        "items" | "cards" | "lessons" => item_count(text),
        _ => prose_words(text),
    }
}

fn item_count(text: &str) -> usize {
    let bullets = text
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count();
    let headings = text
        .lines()
        .filter(|line| line.trim_start().starts_with("## "))
        .count();
    bullets
        .max(headings)
        .max(usize::from(!text.trim().is_empty()))
}

fn weak_classes(text: &str, atom: &AtomRow, count: usize) -> Vec<String> {
    let lower = text.to_ascii_lowercase();
    let mut weak = Vec::new();
    if count < atom.count_floor as usize {
        weak.push("below-count-floor".to_string());
    }
    for section in split_lines(&atom.required_sections) {
        if !lower.contains(&section.to_ascii_lowercase()) {
            weak.push("missing-required-section".to_string());
            break;
        }
    }
    for (needle, class) in weak_needles() {
        if lower.contains(needle) {
            weak.push(class.to_string());
        }
    }
    if matches!(atom.role.as_str(), "scene" | "chapter") && story_bible_only(&lower) {
        weak.push("story-bible-only".to_string());
    }
    dedup(weak)
}

fn weak_needles() -> [(&'static str, &'static str); 5] {
    [
        ("content_state=structure-only", "scaffold-only"),
        ("placeholder", "placeholder"),
        ("todo", "placeholder"),
        ("outline", "outline-only"),
        ("example only", "generic-example"),
    ]
}

fn story_bible_only(lower: &str) -> bool {
    ["premise", "setting", "character", "outline"]
        .iter()
        .any(|needle| lower.contains(needle))
        && !["dialogue", "said", "thought", "walked", "looked"]
            .iter()
            .any(|needle| lower.contains(needle))
}

fn dedup(values: Vec<String>) -> Vec<String> {
    values.into_iter().fold(Vec::new(), |mut out, value| {
        if !out.iter().any(|existing| existing == &value) {
            out.push(value);
        }
        out
    })
}
