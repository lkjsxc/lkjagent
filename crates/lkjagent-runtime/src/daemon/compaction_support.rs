use lkjagent_graph::{compaction_plan, TaskGraphState};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use crate::error::RuntimeResult;

pub(super) fn compaction_summary(
    conn: &Connection,
    reason: &str,
    before: usize,
    graph: Option<&TaskGraphState>,
) -> RuntimeResult<String> {
    let task = match store_state::get(conn, "open task")? {
        Some(value) => value,
        None => "active".to_string(),
    };
    let mut lines = vec![
        "compaction resume".to_string(),
        format!("reason={reason}"),
        format!("open_task={task}"),
        format!("before_tokens={before}"),
    ];
    if let Some(graph) = graph {
        push_graph_summary(&mut lines, graph);
    }
    Ok(lines.join("\n"))
}

fn push_graph_summary(lines: &mut Vec<String>, graph: &TaskGraphState) {
    let plan = compaction_plan(graph);
    lines.push(format!("graph_case={}", plan.case_id.unwrap_or_default()));
    lines.push(format!("phase={}", plan.phase.as_str()));
    lines.push(format!("active_node={}", plan.active_node.0));
    lines.push(format!("objective={}", plan.objective));
    lines.push(format!(
        "missing_evidence={}",
        join_or_none(&plan.missing_evidence)
    ));
    lines.push(format!(
        "touched_paths={}",
        join_or_none(&plan.touched_paths)
    ));
    lines.push(format!(
        "selected_packages={}",
        join_or_none(&plan.selected_packages)
    ));
    lines.push(format!(
        "recovery=parse:{} params:{} tool:{} repeat:{}",
        plan.recovery.parse_failures,
        plan.recovery.param_failures,
        plan.recovery.tool_failures,
        plan.recovery.repeat_failures
    ));
    lines.push(format!("completion_ready={}", plan.completion_ready));
    lines.push(format!(
        "legal_next={}",
        join_or_none(&plan.legal_next_transitions)
    ));
    if let Some(document) = &graph.document {
        lines.push(format!("document_root={}", document.root));
        lines.push(format!("document_kind={}", document.kind));
        lines.push(format!("document_audit={:?}", document.audit_status));
    }
}

fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().take(8).cloned().collect::<Vec<_>>().join(",")
    }
}
