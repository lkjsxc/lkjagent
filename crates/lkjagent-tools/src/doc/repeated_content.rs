use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;

pub fn repeated_sibling_checks(
    root: &Path,
    files: &[PathBuf],
    failures: &mut Vec<String>,
) -> ToolResult<()> {
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in files {
        let text = fs::read_to_string(file)?;
        let signature = heading_signature(&text);
        if signature.len() > 1 && generic_density(&text) >= 2 {
            groups.entry(signature).or_default().push(rel(root, file));
        }
    }
    for paths in groups.values().filter(|paths| paths.len() >= 3) {
        failures.push(format!("repeated_generic_content: {}", paths.join(",")));
    }
    Ok(())
}

fn heading_signature(text: &str) -> String {
    text.lines()
        .filter_map(|line| line.strip_prefix("## "))
        .take(5)
        .collect::<Vec<_>>()
        .join("|")
}

fn generic_density(text: &str) -> usize {
    let lower = text.to_ascii_lowercase();
    [
        "this file records",
        "generated documentation tree",
        "this section explains",
        "coming soon",
        "todo",
        "scaffolded",
    ]
    .iter()
    .filter(|needle| lower.contains(**needle))
    .count()
}

fn rel(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative,
        Err(_) => path,
    }
    .to_string_lossy()
    .to_string()
}
