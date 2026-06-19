use std::fs;
use std::path::Path;

use crate::count_guard::CountGuard;
use crate::count_profile::DeliverableProfile;
use crate::count_seed_allocation::allocation_for;
use crate::error::{ToolError, ToolResult};

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
    let objective = objective_summary(objective);
    let allocation = allocation_for(guard.target, &objective);
    let profile = DeliverableProfile::from_objective(&objective);
    write_file(
        &root.join("README.md"),
        &profile.root_readme(
            allocation.docs,
            allocation.main,
            allocation.index_files(),
            &objective,
        ),
    )?;
    if allocation.indexes {
        write_file(
            &root.join("docs/README.md"),
            &profile.docs_readme(allocation.docs, allocation.main, &objective),
        )?;
        write_file(
            &root.join("main/README.md"),
            &profile.main_readme(allocation.main, &objective),
        )?;
    }
    for index in 1..=allocation.docs {
        write_file(
            &root.join(format!("docs/design-{index:03}.md")),
            &profile.doc_page(index, allocation.docs, allocation.main, &objective),
        )?;
    }
    for index in 1..=allocation.main {
        write_file(
            &root.join(format!("main/part-{index:03}.md")),
            &profile.main_page(index, allocation.main, &objective),
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
