use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::record_rows::{records, upsert_record};
use rusqlite::Connection;

pub fn ensure_root(workspace: &Path) -> Result<(), String> {
    for rel in [
        "inbox",
        "records",
        "artifacts/transcripts",
        "artifacts/proof",
        "indexes",
        "system/manifests",
    ] {
        fs::create_dir_all(workspace.join(rel)).map_err(|error| error.to_string())?;
    }
    write_readme(workspace, workspace)?;
    for rel in [
        "inbox",
        "records",
        "artifacts",
        "artifacts/transcripts",
        "artifacts/proof",
        "indexes",
        "system",
        "system/manifests",
    ] {
        write_readme(workspace, &workspace.join(rel))?;
    }
    Ok(())
}

pub fn ensure_for_path(workspace: &Path, rel: &str) -> Result<(), String> {
    fs::create_dir_all(workspace).map_err(|error| error.to_string())?;
    let path = workspace.join(rel);
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    refresh_for_path(workspace, rel)
}

pub fn refresh_for_path(workspace: &Path, rel: &str) -> Result<(), String> {
    let path = workspace.join(rel);
    let Some(parent) = path.parent() else {
        return write_readme(workspace, workspace);
    };
    for dir in readme_dirs(workspace, parent) {
        write_readme(workspace, &dir)?;
    }
    Ok(())
}

pub fn repair_record_links(
    conn: &Connection,
    workspace: &Path,
    moved_id: &str,
    old_path: &str,
    new_path: &str,
    now: &str,
) -> usize {
    let Ok(rows) = records(conn, None, true) else {
        return 0;
    };
    let mut repaired = 0;
    for mut row in rows {
        if row.id == moved_id {
            continue;
        }
        let Ok(text) = crate::effect_files::read_text(workspace, &row.path) else {
            continue;
        };
        if !text.contains(old_path) {
            continue;
        }
        let output = text.replace(old_path, new_path);
        let Ok(fingerprint) = record_fingerprint(&output) else {
            continue;
        };
        if crate::effect_files::write_bytes(workspace, &row.path, output.as_bytes()).is_err() {
            continue;
        }
        row.fingerprint = fingerprint;
        row.updated_at = now.to_string();
        if upsert_record(conn, &row).is_ok() {
            let _ = refresh_for_path(workspace, &row.path);
            repaired += 1;
        }
    }
    repaired
}

fn readme_dirs(workspace: &Path, leaf: &Path) -> Vec<PathBuf> {
    let mut dirs = vec![workspace.to_path_buf()];
    let mut current = leaf.to_path_buf();
    let mut lineage = Vec::new();
    while current.starts_with(workspace) && current != workspace {
        lineage.push(current.clone());
        if !current.pop() {
            break;
        }
    }
    lineage.reverse();
    dirs.extend(lineage);
    dirs
}

fn write_readme(workspace: &Path, dir: &Path) -> Result<(), String> {
    let title = title(
        dir.file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("workspace"),
    );
    let mut lines = vec![
        format!("# {title}"),
        String::new(),
        "Purpose: owner-readable workspace directory managed by lkjagent.".to_string(),
        String::new(),
        "## Children".to_string(),
        String::new(),
    ];
    let children = child_links(dir)?;
    if children.is_empty() {
        lines.push("none".to_string());
    } else {
        lines.extend(children);
    }
    lines.push(String::new());
    let rel = dir
        .join("README.md")
        .strip_prefix(workspace)
        .map_err(|error| error.to_string())?
        .to_str()
        .ok_or_else(|| "README path is not UTF-8".to_string())?
        .to_string();
    crate::effect_files::write_bytes(workspace, &rel, lines.join("\n").as_bytes())
}

fn child_links(dir: &Path) -> Result<Vec<String>, String> {
    let mut links = Vec::new();
    for entry in fs::read_dir(dir).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "README.md" {
            continue;
        }
        if entry.path().is_dir() {
            links.push(format!("- [{name}]({name}/)"));
        } else {
            links.push(format!("- [{name}]({name})"));
        }
    }
    links.sort();
    Ok(links)
}

fn title(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
        None => "Workspace".to_string(),
    }
}
