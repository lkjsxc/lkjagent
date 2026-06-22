use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;
use crate::fs::workspace_path;

pub fn audit(workspace: &Path, root: &str) -> ToolResult<String> {
    let full = workspace_path(workspace, root)?;
    if !full.exists() {
        return Ok(failed(root, &["missing_root".to_string()], 0));
    }
    let files = dictionary_files(&full)?;
    if files.is_empty() {
        return Ok(failed(root, &["missing_dictionary_files".to_string()], 0));
    }
    let mut entries = Vec::new();
    for file in files {
        let text = fs::read_to_string(file)?;
        entries.extend(parse_entries(&text));
    }
    let failures = readiness_failures(&entries);
    if failures.is_empty() {
        Ok(format!(
            "artifact audit passed\nroot={root}\nkind=dictionary\nentries={}\nreadiness=content-bearing\nnext_action=record artifact-readiness evidence",
            entries.len()
        ))
    } else {
        Ok(failed(root, &failures, entries.len()))
    }
}

fn dictionary_files(path: &Path) -> ToolResult<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(if dictionary_extension(path) {
            vec![path.to_path_buf()]
        } else {
            Vec::new()
        });
    }
    let mut files = Vec::new();
    collect_dictionary_files(path, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_dictionary_files(dir: &Path, files: &mut Vec<PathBuf>) -> ToolResult<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_dictionary_files(&path, files)?;
        } else if dictionary_extension(&path) && !metadata_file(&path) {
            files.push(path);
        }
    }
    Ok(())
}

fn dictionary_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext, "md" | "txt"))
}

fn metadata_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "README.md")
}

fn parse_entries(text: &str) -> Vec<Entry> {
    let mut entries = heading_entries(text);
    entries.extend(bullet_entries(text));
    entries
}

fn heading_entries(text: &str) -> Vec<Entry> {
    text.split("\n## ").skip(1).map(Entry::from_block).collect()
}

fn bullet_entries(text: &str) -> Vec<Entry> {
    text.lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .filter(|line| line.contains(':'))
        .map(Entry::from_block)
        .collect()
}

fn readiness_failures(entries: &[Entry]) -> Vec<String> {
    let mut failures = Vec::new();
    if entries.len() < 20 {
        failures.push(format!(
            "entry_count: expected>=20 actual={}",
            entries.len()
        ));
    }
    for (label, count) in [
        (
            "pronunciation",
            entries.iter().filter(|entry| entry.pronunciation).count(),
        ),
        (
            "part_of_speech",
            entries.iter().filter(|entry| entry.part_of_speech).count(),
        ),
        (
            "definition",
            entries.iter().filter(|entry| entry.definition).count(),
        ),
        (
            "etymology",
            entries.iter().filter(|entry| entry.etymology).count(),
        ),
        (
            "example",
            entries.iter().filter(|entry| entry.example).count(),
        ),
    ] {
        if below_threshold(count, entries.len()) {
            failures.push(format!(
                "{label}_coverage: expected>=80% actual={count}/{}",
                entries.len()
            ));
        }
    }
    failures
}

fn below_threshold(count: usize, total: usize) -> bool {
    total == 0 || count.saturating_mul(100) < total.saturating_mul(80)
}

fn failed(root: &str, failures: &[String], entries: usize) -> String {
    format!(
        "artifact audit failed\nroot={root}\nkind=dictionary\nentries={entries}\nreadiness=failed\nfailures:\n- {}\nnext_action=artifact.next or fs.batch_write dictionary entries",
        failures.join("\n- ")
    )
}

struct Entry {
    pronunciation: bool,
    part_of_speech: bool,
    definition: bool,
    etymology: bool,
    example: bool,
}

impl Entry {
    fn from_block(block: &str) -> Self {
        let lower = block.to_ascii_lowercase();
        Self {
            pronunciation: lower.contains("pronunciation:") || lower.contains("ipa:"),
            part_of_speech: lower.contains("part of speech:") || lower.contains("term class:"),
            definition: has_labeled_detail(&lower, "definition:"),
            etymology: lower.contains("etymology:") || lower.contains("origin:"),
            example: lower.contains("example:") || lower.contains("for example"),
        }
    }
}

fn has_labeled_detail(text: &str, label: &str) -> bool {
    text.lines()
        .find_map(|line| {
            line.split_once(label)
                .map(|(_, value)| word_count(value) >= 8)
        })
        .unwrap_or(false)
}

fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_ascii_alphabetic()))
        .count()
}
