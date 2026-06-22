use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_graph::{compaction_plan, source_graph, TaskGraphState, TaskPhase};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use crate::error::RuntimeResult;

pub(super) fn compaction_summary(
    conn: &Connection,
    reason: &str,
    before: usize,
    graph: Option<&TaskGraphState>,
    log: &[Frame],
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
        format!("last_successful_observation={}", last_observation(log)),
    ];
    if let Some(graph) = graph {
        push_graph_summary(conn, &mut lines, graph)?;
    }
    Ok(lines.join("\n"))
}

fn push_graph_summary(
    conn: &Connection,
    lines: &mut Vec<String>,
    graph: &TaskGraphState,
) -> RuntimeResult<()> {
    let plan = compaction_plan(graph);
    lines.push(format!("graph_case={}", plan.case_id.unwrap_or_default()));
    lines.push(format!("active_mission={}", active_mission(graph)));
    lines.push(format!("phase={}", plan.phase.as_str()));
    lines.push(format!("active_node={}", plan.active_node.0));
    lines.push(format!("objective={}", plan.objective));
    lines.push(format!(
        "required_evidence={}",
        join_or_none(&graph.evidence.requirement_ids())
    ));
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
    lines.push(format!(
        "recovery_ladder_step={}",
        plan.recovery.ladder_position
    ));
    lines.push(format!(
        "last_failed_action={}",
        last_failed_action(&plan.recovery.last_failed_action_fingerprint)
    ));
    lines.push(format!(
        "admitted_next_tools={}",
        join_or_none(&active_allowed_tools(graph))
    ));
    lines.push(format!(
        "exact_next_valid_action={}",
        exact_next_action(graph)
    ));
    lines.push(format!("completion_ready={}", plan.completion_ready));
    lines.push(format!(
        "completion_blocked_reasons={}",
        join_or_none(&plan.missing_evidence)
    ));
    lines.push(format!(
        "legal_next={}",
        join_or_none(&plan.legal_next_transitions)
    ));
    if let Some(document) = &graph.document {
        lines.push(format!("active_artifact_id={}", document.root));
        lines.push(format!("document_root={}", document.root));
        lines.push(format!("document_kind={}", document.kind));
        lines.push(format!("document_audit={:?}", document.audit_status));
        lines.push(format!(
            "write_batch_cursor={}",
            cursor_value(conn, &document.root)?
        ));
    } else {
        lines.push("active_artifact_id=none".to_string());
        lines.push("write_batch_cursor=none".to_string());
    }
    Ok(())
}

fn cursor_value(conn: &Connection, root: &str) -> RuntimeResult<String> {
    Ok(
        lkjagent_store::state::get(conn, &format!("artifact.next cursor {root}"))?
            .unwrap_or_else(|| "none".to_string()),
    )
}

fn active_mission(graph: &TaskGraphState) -> &'static str {
    if graph.recovery.ladder_position > 0 {
        return "Recovery";
    }
    match graph.phase {
        TaskPhase::Recovery => "Recovery",
        TaskPhase::Verification | TaskPhase::Completion => "Verification",
        TaskPhase::Maintenance => "Maintenance",
        TaskPhase::Compaction => "Compaction",
        TaskPhase::Waiting | TaskPhase::Closed => "Idle",
        TaskPhase::Intake | TaskPhase::Planning | TaskPhase::Context | TaskPhase::Execution => {
            "OwnerWork"
        }
    }
}

fn active_allowed_tools(graph: &TaskGraphState) -> Vec<String> {
    match source_graph()
        .nodes
        .iter()
        .find(|node| node.id == graph.active_node)
    {
        Some(node) => node
            .allowed_actions
            .iter()
            .map(|tool| (*tool).to_string())
            .collect(),
        None => Vec::new(),
    }
}

fn exact_next_action(graph: &TaskGraphState) -> String {
    let allowed = active_allowed_tools(graph);
    if allowed.iter().any(|tool| tool == "graph.recover") {
        return "<act><tool>graph.recover</tool></act>".to_string();
    }
    if allowed.iter().any(|tool| tool == "artifact.next") {
        let root = graph
            .document
            .as_ref()
            .map_or("artifact", |doc| doc.root.as_str());
        return format!("<act><tool>artifact.next</tool><root>{root}</root></act>");
    }
    match allowed.first() {
        Some(tool) => format!("<act><tool>{tool}</tool></act>"),
        None => "none".to_string(),
    }
}

fn last_failed_action(fingerprint: &Option<String>) -> &str {
    match fingerprint.as_deref() {
        Some(value) => value,
        None => "none",
    }
}

fn last_observation(log: &[Frame]) -> String {
    log.iter()
        .rev()
        .find(|frame| matches!(frame.kind, FrameKind::Observation))
        .map_or_else(|| "none".to_string(), |frame| first_line(&frame.content))
}

fn first_line(value: &str) -> String {
    value
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.chars().take(160).collect())
        .unwrap_or_else(|| "none".to_string())
}

fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().take(8).cloned().collect::<Vec<_>>().join(",")
    }
}
