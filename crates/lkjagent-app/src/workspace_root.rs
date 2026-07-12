use std::fs;
use std::path::{Component, Path, PathBuf};

use lkjagent_core::workspace_record::{parse_record, record_fingerprint};
use lkjagent_store::record_rows::{records, upsert_record};
use rusqlite::Connection;

pub const DEFAULT_WORKSPACE_ROOT: &str = "../workspace";

pub fn resolve(data_root: &Path, configured: &str) -> Result<PathBuf, String> {
    validate_text(configured)?;
    let configured = Path::new(configured);
    let root = if configured.is_absolute() {
        clean(configured)
    } else {
        clean(&data_root.join(configured))
    };
    reject_internal_root(data_root, &root)?;
    Ok(root)
}

pub fn open(root: &Path) -> Result<PathBuf, String> {
    if !root.exists() {
        fs::create_dir_all(root).map_err(|error| format!("create workspace root: {error}"))?;
    }
    let metadata =
        fs::symlink_metadata(root).map_err(|error| format!("open workspace root: {error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err("open workspace root: root must be a real directory".to_string());
    }
    root.canonicalize()
        .map_err(|error| format!("open workspace root: {error}"))
}

pub fn ensure_for_path(workspace: &Path, rel: &str) -> Result<(), String> {
    let _opened = open(workspace)?;
    let path = workspace.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn refresh_for_path(workspace: &Path, _rel: &str) -> Result<(), String> {
    let _opened = open(workspace)?;
    Ok(())
}

pub fn repair_record_links(
    conn: &Connection,
    workspace: &Path,
    moved_id: &str,
    old_path: &str,
    new_path: &str,
    now: &str,
) -> Result<usize, String> {
    let _opened = open(workspace)?;
    let rows = records(conn, None, true).map_err(|error| error.to_string())?;
    let mut repaired = 0;
    for mut row in rows {
        if row.id == moved_id {
            continue;
        }
        if row.archived && !crate::effect_files::path_occupied(workspace, &row.path)? {
            continue;
        }
        let text = crate::effect_files::read_text(workspace, &row.path)?;
        let current = record_fingerprint(&text).map_err(|error| error.message)?;
        let Some(output) = patch_link(&text, old_path, new_path)? else {
            continue;
        };
        if current != row.fingerprint {
            return Err(format!("linked record changed: {}", row.id));
        }
        row.fingerprint = record_fingerprint(&output).map_err(|error| error.message)?;
        row.updated_at = now.to_string();
        upsert_record(conn, &row).map_err(|error| error.to_string())?;
        crate::effect_files::write_bytes(workspace, &row.path, output.as_bytes())?;
        repaired += 1;
    }
    Ok(repaired)
}

fn patch_link(text: &str, old_path: &str, new_path: &str) -> Result<Option<String>, String> {
    let mut parsed = parse_record(text)?;
    let mut changed = false;
    for link in &mut parsed.links {
        if link == old_path {
            *link = new_path.to_string();
            changed = true;
        }
    }
    if !changed {
        return Ok(None);
    }
    let front = text
        .strip_prefix("---\n")
        .ok_or_else(|| "record frontmatter missing".to_string())?;
    let end = front
        .find("\n---\n")
        .ok_or_else(|| "record frontmatter is unterminated".to_string())?
        + 4;
    let area = &text[..end];
    let starts = area
        .match_indices("\nlinks:")
        .map(|(offset, _)| offset + 1)
        .collect::<Vec<_>>();
    if starts.len() != 1 {
        return Err("record links field is ambiguous".to_string());
    }
    let start = starts[0];
    let line_end = text[start..]
        .find('\n')
        .map(|offset| start + offset)
        .ok_or_else(|| "record links field is unterminated".to_string())?;
    let line = format!("links: [{}]", parsed.links.join(", "));
    Ok(Some(format!(
        "{}{}{}",
        &text[..start],
        line,
        &text[line_end..]
    )))
}

fn validate_text(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err("workspace root must be nonempty text without control characters".to_string());
    }
    Ok(())
}

fn reject_internal_root(data_root: &Path, root: &Path) -> Result<(), String> {
    let data = absolute_clean(data_root)?;
    let root = absolute_clean(root)?;
    if root == data || root.starts_with(&data) {
        return Err(
            "workspace root must be separate from runtime data, database, logs, and temp"
                .to_string(),
        );
    }
    Ok(())
}

fn absolute_clean(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        Ok(clean(path))
    } else {
        std::env::current_dir()
            .map(|cwd| clean(&cwd.join(path)))
            .map_err(|error| error.to_string())
    }
}

fn clean(path: &Path) -> PathBuf {
    let mut output = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                output.pop();
            }
            other => output.push(other.as_os_str()),
        }
    }
    output
}
