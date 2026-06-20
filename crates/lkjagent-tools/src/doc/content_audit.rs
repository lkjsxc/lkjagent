use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;

pub fn content_checks(root: &Path, failures: &mut Vec<String>) -> ToolResult<()> {
    if !requires_content(root)? {
        return Ok(());
    }
    for file in markdown_leaves(root)? {
        let text = fs::read_to_string(&file)?;
        if scaffold_only(&text) {
            failures.push(format!("scaffold_only_content: {}", rel(root, &file)));
        }
    }
    Ok(())
}

fn requires_content(root: &Path) -> ToolResult<bool> {
    let manifest = root.join(".lkj-doc-graph.md");
    if !manifest.is_file() {
        return Ok(false);
    }
    let text = fs::read_to_string(manifest)?;
    Ok(text.contains("NarrativeManuscript") || text.contains("Cookbook"))
}

fn markdown_leaves(root: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_markdown(root, &mut files)?;
    files.retain(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name != "README.md" && name != ".lkj-doc-graph.md")
    });
    files.sort();
    Ok(files)
}

fn collect_markdown(dir: &Path, files: &mut Vec<PathBuf>) -> ToolResult<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_markdown(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    Ok(())
}

fn scaffold_only(text: &str) -> bool {
    text.contains("This file records the")
        && text.contains("generated documentation tree")
        && text.contains("\nscaffolded\n")
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}
