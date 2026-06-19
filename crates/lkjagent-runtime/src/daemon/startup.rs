use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use lkjagent_context::budget::{PREFIX_MEMORY_DIGEST, PREFIX_WORKSPACE_BRIEF};
use lkjagent_context::model::Frame;
use lkjagent_skills::index::{entry_from_skill, render_index_text};
use rusqlite::Connection;

use crate::error::{RuntimeError, RuntimeResult};
use crate::prompt::{build_prefix, token_estimate, PromptInputs};

pub fn build_prefix_from_store(
    conn: &Connection,
    skill_library: &Path,
    workspace: &Path,
) -> RuntimeResult<Vec<Frame>> {
    build_prefix(&PromptInputs {
        skill_index: skill_index(skill_library)?,
        workspace_brief: workspace_brief(workspace)?,
        memory_digest: memory_digest(conn)?,
    })
}

pub fn startup_summary(conn: &Connection) -> RuntimeResult<Option<String>> {
    let open_task = lkjagent_store::state::get(conn, "open task")?;
    Ok(open_task.and_then(|task| {
        if task == "none" {
            None
        } else {
            Some(format!("open task at restart: {task}"))
        }
    }))
}

fn skill_index(root: &Path) -> RuntimeResult<String> {
    let known = known_paths(root)?;
    let mut entries = Vec::new();
    for path in &known {
        let text = fs::read_to_string(path).map_err(io_error)?;
        let source = lkjagent_skills::model::SkillSource {
            path,
            text: &text,
            known_paths: &known,
        };
        if let Ok(skill) = lkjagent_skills::validate::parse(&source) {
            entries.push(entry_from_skill(&skill, 0));
        }
    }
    Ok(render_index_text(&entries))
}

fn known_paths(root: &Path) -> RuntimeResult<BTreeSet<String>> {
    let mut paths = BTreeSet::new();
    if !root.exists() {
        return Ok(paths);
    }
    for entry in fs::read_dir(root).map_err(io_error)? {
        let entry = entry.map_err(io_error)?;
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == "md") {
            paths.insert(path.to_string_lossy().replace('\\', "/"));
        }
    }
    Ok(paths)
}

fn workspace_brief(workspace: &Path) -> RuntimeResult<String> {
    let path = workspace.join("AGENTS.md");
    let text = fs::read_to_string(path).unwrap_or_else(|_| "No workspace AGENTS.md found.".into());
    if token_estimate(&text) <= PREFIX_WORKSPACE_BRIEF {
        return Ok(text);
    }
    let limit = PREFIX_WORKSPACE_BRIEF.saturating_mul(4).saturating_sub(96);
    let head = text.chars().take(limit).collect::<String>();
    Ok(format!("{head}\n[truncated to workspace brief budget]"))
}

fn memory_digest(conn: &Connection) -> RuntimeResult<String> {
    let rows = lkjagent_store::memory::digest(conn, None, PREFIX_MEMORY_DIGEST as i64)?;
    if rows.is_empty() {
        return Ok("none".to_string());
    }
    Ok(rows
        .iter()
        .map(|row| {
            format!(
                "kind={}\ntitle={}\ntags={}\n{}",
                row.kind, row.title, row.tags, row.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n"))
}

fn io_error(error: std::io::Error) -> RuntimeError {
    RuntimeError::Prompt(error.to_string())
}
