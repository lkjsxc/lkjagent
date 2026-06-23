use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_graph::{TaskGraphState, TransitionDecision};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::authority_ledger::{persist_authority_ledger, AuthorityGraphView};
use super::graph_policy::completion_decision;
use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::mode::TurnAuthority;

pub(super) fn persist_authority_snapshot(
    daemon: &ResidentDaemon,
    conn: &Connection,
    authority: &TurnAuthority,
) -> RuntimeResult<()> {
    let graph = graph_fields(conn, daemon.state.graph.as_ref());
    store_state::set(conn, "authority mission", authority.mission.as_str())?;
    store_state::set(
        conn,
        "authority active mode",
        &format!("{:?}", authority.mode),
    )?;
    store_state::set(
        conn,
        "authority endpoint decision",
        &format!("{:?}", authority.endpoint_decision),
    )?;
    store_state::set(conn, "authority case id", &graph.case_id)?;
    store_state::set(conn, "authority node", &graph.node)?;
    store_state::set(conn, "authority phase", &graph.phase)?;
    store_state::set(conn, "authority evidence gaps", &graph.evidence_gaps)?;
    store_state::set(conn, "authority artifact root", &graph.artifact_root)?;
    store_state::set(conn, "authority recovery route", &graph.recovery_route)?;
    store_state::set(
        conn,
        "authority last failed action",
        &graph.last_failed_action,
    )?;
    store_state::set(
        conn,
        "authority repeated action count",
        &daemon.dispatch_state.repeat_count.to_string(),
    )?;
    store_state::set(
        conn,
        "authority last successful observation",
        &last_successful_observation(&daemon.state.context.log),
    )?;
    store_state::set(
        conn,
        "authority allowed tools",
        &join_or_none(&authority.effective_policy.allowed_tools),
    )?;
    store_state::set(
        conn,
        "authority blocked tools",
        &join_or_none(&authority.effective_policy.blocked_tools),
    )?;
    store_state::set(conn, "authority next action", &authority.valid_example)?;
    persist_authority_ledger(
        daemon,
        conn,
        authority,
        AuthorityGraphView {
            case_id: &graph.case_id,
            node: &graph.node,
            evidence_gaps: &graph.evidence_gaps,
            recovery_route: &graph.recovery_route,
        },
    )?;
    Ok(())
}

struct GraphAuthorityFields {
    case_id: String,
    node: String,
    phase: String,
    evidence_gaps: String,
    artifact_root: String,
    recovery_route: String,
    last_failed_action: String,
}

fn graph_fields(conn: &Connection, graph: Option<&TaskGraphState>) -> GraphAuthorityFields {
    let Some(graph) = graph else {
        return empty_graph_fields();
    };
    GraphAuthorityFields {
        case_id: graph
            .case_id
            .map_or_else(|| "none".to_string(), |id| id.to_string()),
        node: graph.active_node.0.to_string(),
        phase: graph.phase.as_str().to_string(),
        evidence_gaps: evidence_gaps(conn, graph),
        artifact_root: graph
            .document
            .as_ref()
            .map_or_else(|| "none".to_string(), |doc| doc.root.clone()),
        recovery_route: recovery_route(graph),
        last_failed_action: graph
            .recovery
            .last_failed_action_fingerprint
            .clone()
            .unwrap_or_else(|| "none".to_string()),
    }
}

fn empty_graph_fields() -> GraphAuthorityFields {
    GraphAuthorityFields {
        case_id: "none".to_string(),
        node: "none".to_string(),
        phase: "none".to_string(),
        evidence_gaps: "none".to_string(),
        artifact_root: "none".to_string(),
        recovery_route: "none".to_string(),
        last_failed_action: "none".to_string(),
    }
}

fn evidence_gaps(conn: &Connection, graph: &TaskGraphState) -> String {
    match completion_decision(conn, graph) {
        TransitionDecision::Admit { .. } => "none".to_string(),
        TransitionDecision::Defer { missing } => join_strings(&missing),
        TransitionDecision::Recover { reason, .. } | TransitionDecision::Refuse { reason } => {
            reason
        }
    }
}

fn recovery_route(graph: &TaskGraphState) -> String {
    if graph.recovery.ladder_position > 0 || graph.phase == lkjagent_graph::TaskPhase::Recovery {
        graph.active_node.0.to_string()
    } else {
        "none".to_string()
    }
}

fn last_successful_observation(log: &[Frame]) -> String {
    log.iter()
        .rev()
        .find(|frame| matches!(frame.kind, FrameKind::Observation))
        .map_or_else(|| "none".to_string(), |frame| one_line(&frame.content))
}

fn one_line(value: &str) -> String {
    value
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.chars().take(160).collect())
        .unwrap_or_else(|| "none".to_string())
}

fn join_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn join_strings(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().take(8).cloned().collect::<Vec<_>>().join(",")
    }
}
