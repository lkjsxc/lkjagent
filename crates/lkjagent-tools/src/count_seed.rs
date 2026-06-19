use std::fs;
use std::path::Path;

use crate::count_guard::CountGuard;
use crate::error::{ToolError, ToolResult};

const DESIGN_FOCUSES: [&str; 12] = [
    "scope and acceptance criteria",
    "directory map and reading order",
    "voice, format, and naming conventions",
    "continuity ledger",
    "source assumptions and constraints",
    "section pacing plan",
    "quality checklist",
    "revision workflow",
    "risk register",
    "verification plan",
    "handoff notes",
    "completion summary",
];

pub fn scaffold_counted_documents(
    workspace: &Path,
    guard: CountGuard,
    objective: &str,
) -> ToolResult<String> {
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
    let objective = objective_summary(objective);
    write_file(
        &root.join("README.md"),
        &root_readme(docs, main, &objective),
    )?;
    for index in 1..=docs {
        write_file(
            &root.join(format!("docs/design-{index:03}.md")),
            &doc_page(index, &objective),
        )?;
    }
    for index in 1..=main {
        write_file(
            &root.join(format!("main/part-{index:03}.md")),
            &main_page(index, &objective),
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

fn root_readme(docs: usize, main: usize, objective: &str) -> String {
    format!(
        "# Structured Output\n\n## Purpose\n\nA generated multi-file deliverable for this objective:\n\n{objective}\n\n## Table of Contents\n\n- [docs/](docs/): {docs} design files for planning, continuity, and verification.\n- [main/](main/): {main} ordered main content files.\n\n## Verification\n\nThe scaffold was generated as an exact counted deliverable. Keep the total file count stable unless the owner changes the target.\n"
    )
}

fn doc_page(index: usize, objective: &str) -> String {
    let focus = DESIGN_FOCUSES
        .get(index.saturating_sub(1))
        .copied()
        .unwrap_or("supplemental planning notes");
    format!(
        "# Design Memo {index:03}\n\n## Focus\n\n{focus}\n\n## Objective Context\n\n{objective}\n\n## Notes\n\nUse this memo to keep the generated file set coherent, ordered, and verifiable. Record decisions that should shape the corresponding main content files.\n"
    )
}

fn main_page(index: usize, objective: &str) -> String {
    let arc = index.saturating_sub(1) / 10 + 1;
    let slot = index.saturating_sub(1) % 10 + 1;
    let next = index.saturating_add(1);
    format!(
        "# Main Content {index:03}\n\n## Position\n\n- Arc: {arc}\n- Segment: {slot}\n\n## Objective Context\n\n{objective}\n\n## Draft Content\n\nThis segment turns the objective into concrete material for ordered part {index}. It should read as a complete unit while preserving continuity with the surrounding files.\n\n## Continuity Hand-Off\n\n- Carry forward terms, decisions, and open questions from earlier parts.\n- Leave a clear next-step hook for part {next:03} when another part follows.\n"
    )
}

fn objective_summary(objective: &str) -> String {
    let trimmed = objective.trim();
    if trimmed.is_empty() {
        return "No explicit objective was provided.".to_string();
    }
    trimmed.chars().take(400).collect()
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
