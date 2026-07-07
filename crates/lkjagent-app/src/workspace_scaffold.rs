use std::fs;
use std::path::{Path, PathBuf};

pub fn ensure_for_path(workspace: &Path, rel: &str) -> Result<(), String> {
    fs::create_dir_all(workspace).map_err(|error| error.to_string())?;
    write_if_missing(
        &workspace.join("README.md"),
        "# Workspace\n\nOwner-readable files linked to the lkjagent ledger.\n",
    )?;
    let path = workspace.join(rel);
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    for dir in lineage(workspace, parent) {
        write_readme(&dir)?;
    }
    Ok(())
}

fn lineage(workspace: &Path, leaf: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut current = leaf.to_path_buf();
    while current.starts_with(workspace) && current != workspace {
        dirs.push(current.clone());
        if !current.pop() {
            break;
        }
    }
    dirs.reverse();
    dirs
}

fn write_readme(dir: &Path) -> Result<(), String> {
    let name = dir
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("workspace");
    let title = title(name);
    let body =
        format!("# {title}\n\nPurpose: owner-readable workspace directory managed by lkjagent.\n");
    write_if_missing(&dir.join("README.md"), &body)
}

fn write_if_missing(path: &Path, body: &str) -> Result<(), String> {
    if !path.exists() {
        fs::write(path, body).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn title(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
        None => "Workspace".to_string(),
    }
}
