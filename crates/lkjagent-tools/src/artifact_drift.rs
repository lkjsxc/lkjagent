use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;

const FORBIDDEN_PATHS: &[&str] = &[
    "baking.md",
    "fermentation.md",
    "flour-water-salt-yeast.md",
    "kneading.md",
    "shaping.md",
    "ciabatta.md",
    "focaccia.md",
    "rye-bread.md",
    "sourdough-country-loaf.md",
];

const FORBIDDEN_TEXT: &[&str] = &[
    "this bread cookbook section",
    "bread cookbook",
    "sourdough country loaf",
    "ciabatta",
    "focaccia",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftReport {
    pub failures: Vec<String>,
}

impl DriftReport {
    pub fn is_empty(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn observation(&self, root: &str) -> String {
        format!(
            "artifact audit failed\nroot={root}\nchecks=16\npassed=15\nfailed=1\nfailures:\n- objective_drift: {}\nnext_action=repair drifted paths then artifact.audit",
            self.failures.join("; ")
        )
    }

    pub fn block_message(&self, root: &str) -> String {
        format!(
            "artifact drift guard active\nroot={root}\nblocked=artifact.next\nfailures={}\nnext_action=artifact.audit after repair",
            self.failures.join("; ")
        )
    }
}

pub fn japanese_cookbook(root: &Path) -> ToolResult<Option<DriftReport>> {
    let catalog = optional_catalog(root)?;
    if !is_japanese_cookbook(root, &catalog) {
        return Ok(None);
    }
    let mut failures = Vec::new();
    for path in markdown_files(root)? {
        let relative = rel(root, &path);
        if forbidden_path(&relative) {
            failures.push(format!("path {relative}"));
        }
        let text = fs::read_to_string(&path)?;
        if let Some(term) = forbidden_text(&text) {
            failures.push(format!("content {relative} term={term}"));
        }
    }
    Ok(Some(DriftReport { failures }))
}

fn is_japanese_cookbook(root: &Path, catalog: &str) -> bool {
    let haystack = format!("{} {catalog}", root.display()).to_ascii_lowercase();
    haystack.contains("japanese") && haystack.contains("cookbook")
}

fn forbidden_path(relative: &str) -> bool {
    let lower = relative.to_ascii_lowercase();
    FORBIDDEN_PATHS.iter().any(|name| lower.ends_with(name))
}

fn forbidden_text(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    FORBIDDEN_TEXT
        .iter()
        .copied()
        .find(|term| lower.contains(term))
}

fn markdown_files(root: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect(root, &mut files)?;
    files.retain(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name != "README.md")
    });
    files.sort();
    Ok(files)
}

fn collect(dir: &Path, files: &mut Vec<PathBuf>) -> ToolResult<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    Ok(())
}

fn optional_catalog(root: &Path) -> ToolResult<String> {
    let path = root.join("catalog.toml");
    if path.is_file() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(String::new())
    }
}

fn rel(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative,
        Err(_) => path,
    }
    .to_string_lossy()
    .to_string()
}
