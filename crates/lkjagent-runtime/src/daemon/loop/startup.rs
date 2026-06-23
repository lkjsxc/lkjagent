use std::fs;
use std::path::Path;

use lkjagent_context::budget::{PREFIX_MEMORY_DIGEST, PREFIX_WORKSPACE_BRIEF};
use lkjagent_context::model::Frame;
use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::graph_state::prefix_graph_state;
use crate::prompt::{build_prefix, token_estimate, PromptInputs};

pub fn build_prefix_from_store(conn: &Connection, workspace: &Path) -> RuntimeResult<Vec<Frame>> {
    build_prefix(&PromptInputs {
        graph_state: prefix_graph_state(conn)?,
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
    let rows = lkjagent_store::memory::digest(
        conn,
        last_task_summary_id(conn)?,
        PREFIX_MEMORY_DIGEST as i64,
    )?;
    let budget = PREFIX_MEMORY_DIGEST.saturating_sub(token_estimate("## memory digest\n"));
    if rows.is_empty() {
        return Ok("none".to_string());
    }
    Ok(render_memory_rows(&rows, budget))
}

fn last_task_summary_id(conn: &Connection) -> RuntimeResult<Option<i64>> {
    Ok(lkjagent_store::state::get(conn, "last task summary id")?
        .and_then(|value| value.parse::<i64>().ok()))
}

fn render_memory_rows(rows: &[lkjagent_store::memory::MemoryRow], budget: usize) -> String {
    let mut rendered = String::new();
    for row in rows {
        let entry = format!(
            "kind={}\ntitle={}\ntags={}\n{}",
            row.kind, row.title, row.tags, row.content
        );
        let next = append_memory_entry(&rendered, &entry);
        if token_estimate(&next) <= budget {
            rendered = next;
        } else if rendered.is_empty() {
            rendered = truncate_memory_entry(&entry, budget);
        }
    }
    if rendered.is_empty() {
        "none".to_string()
    } else {
        rendered
    }
}

fn append_memory_entry(rendered: &str, entry: &str) -> String {
    if rendered.is_empty() {
        entry.to_string()
    } else {
        format!("{rendered}\n\n{entry}")
    }
}

fn truncate_memory_entry(entry: &str, budget: usize) -> String {
    let marker = "\n[truncated to memory digest budget]";
    let mut text = String::new();
    for ch in entry.chars() {
        let candidate = format!("{text}{ch}{marker}");
        if token_estimate(&candidate) > budget {
            break;
        }
        text.push(ch);
    }
    format!("{text}{marker}")
}
