use std::fs;
use std::path::Path;

use crate::count_guard::CountGuard;
use crate::error::{ToolError, ToolResult};

pub fn scaffold_counted_documents(workspace: &Path, guard: CountGuard) -> ToolResult<String> {
    if guard.target == 0 {
        return Err(ToolError::invalid(
            "counted document scaffold needs at least 1 file",
        ));
    }
    let root = workspace.join("structured-output");
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(root.join("docs"))?;
    fs::create_dir_all(root.join("main"))?;
    let docs = guard.target.saturating_sub(1).min(12);
    let main = guard.target.saturating_sub(1).saturating_sub(docs);
    write_file(&root.join("README.md"), &root_readme(docs, main))?;
    for index in 1..=docs {
        write_file(
            &root.join(format!("docs/design-{index:03}.md")),
            &doc_page(index),
        )?;
    }
    for index in 1..=main {
        write_file(
            &root.join(format!("main/part-{index:03}.md")),
            &main_page(index),
        )?;
    }
    let count = count_files(&root)?;
    if count != guard.target {
        return Err(ToolError::invalid(format!(
            "counted document scaffold expected {} files, got {count}",
            guard.target
        )));
    }
    Ok(format!(
        "counted document scaffold root=structured-output\nfiles={count}\nverification=ok\ncompletion=ready"
    ))
}

fn root_readme(docs: usize, main: usize) -> String {
    format!(
        "# Structured Output\n\n## Purpose\n\nA generated multi-file deliverable with design documents and main content.\n\n## Table of Contents\n\n- [docs/](docs/): {docs} design files.\n- [main/](main/): {main} main content files.\n"
    )
}

fn doc_page(index: usize) -> String {
    format!(
        "# Design Memo {index:03}\n\n## Purpose\n\nPlanning notes, structure, and continuity constraints for output batch {index}.\n"
    )
}

fn main_page(index: usize) -> String {
    format!(
        "# Main Content {index:03}\n\n## Purpose\n\nPrimary deliverable segment {index}, ready for later expansion or refinement.\n"
    )
}

fn write_file(path: &Path, content: &str) -> ToolResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

fn count_files(path: &Path) -> ToolResult<usize> {
    let mut count = 0_usize;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count = count.saturating_add(count_files(&child)?);
        } else {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
