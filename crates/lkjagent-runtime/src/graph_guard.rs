use rusqlite::Connection;

use lkjagent_store::state as store_state;
use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::count_guard::{CountKind, CountMode};

use crate::error::RuntimeResult;

pub fn append_store_guard(conn: &Connection, graph: String) -> RuntimeResult<String> {
    let value = store_state::get(conn, "completion guard")?.unwrap_or_else(|| "none".to_string());
    Ok(append_guard(
        graph,
        CompletionGuard::from_state_value(&value),
    ))
}

pub fn append_plan_guard(plan: &mut String, guard: CompletionGuard) {
    if let Some(text) = guard_text(guard) {
        plan.push('\n');
        plan.push_str(&text);
    }
}

fn append_guard(mut graph: String, guard: CompletionGuard) -> String {
    if graph.contains("completion_guard=") {
        return graph;
    }
    if let Some(text) = guard_text(guard) {
        graph.push('\n');
        graph.push_str(&text);
    }
    graph
}

fn guard_text(guard: CompletionGuard) -> Option<String> {
    let count = guard.count_guard()?;
    Some(format!(
        "completion_guard={}\ncount_guard_instruction={}",
        guard.as_state_value(),
        count_instruction(count.kind, count.mode)
    ))
}

fn count_instruction(kind: CountKind, mode: CountMode) -> &'static str {
    match (kind, mode) {
        (CountKind::File, CountMode::Exact) => {
            "exact file count active; keep the act payload under about 1200 chars; shell.run starts in workspace, so do not cd /workspace; use one compact shell.run command with direct /bin/sh loops and printf templates for bulk creation and count verification; no brace expansion, cat heredocs, bash scripts, literal bodies, or one fs.write per file"
        }
        (CountKind::File, CountMode::Approximate) => {
            "approximate file count active; keep the act payload under about 1200 chars; shell.run starts in workspace, so do not cd /workspace; use one compact shell.run command with direct /bin/sh loops and printf templates for bulk creation and count verification; no brace expansion, cat heredocs, bash scripts, literal bodies, or one fs.write per file"
        }
        (CountKind::Markdown, CountMode::Exact) => {
            "exact markdown count active; keep the act payload under about 1200 chars; shell.run starts in workspace, so do not cd /workspace; use one compact shell.run command with direct /bin/sh loops and printf templates for bulk markdown creation and count verification; no brace expansion, cat heredocs, bash scripts, literal bodies, or one fs.write per file"
        }
        (CountKind::Markdown, CountMode::Approximate) => {
            "approximate markdown count active; keep the act payload under about 1200 chars; shell.run starts in workspace, so do not cd /workspace; use one compact shell.run command with direct /bin/sh loops and printf templates for bulk markdown creation and count verification; no brace expansion, cat heredocs, bash scripts, literal bodies, or one fs.write per file"
        }
    }
}
