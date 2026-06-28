use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::NoticeKind;
use lkjagent_graph::case_recovery::RecoveryRecord;
use lkjagent_graph::{promote_recovery_track, CaseStatus, GraphNodeId, TaskPhase};
use lkjagent_store::events::EventKind;

use crate::graph_state::graph_notice_frame;
use crate::prompt::token_estimate;
use crate::step::fault_key::retry_key;
use crate::step::fault_meta::{fault_kind, fault_name, set_graph_fault_count};
use crate::step::frames::append_notice;
use crate::step::recovery_select::recovery_transition;
use crate::step::Effect;
use crate::task::RuntimeState;

#[derive(Clone, Copy)]
pub(crate) enum RecoveryFault {
    Parse,
    Payload,
    Params,
    Repeat,
    Tool,
}

pub(super) fn record_recoverable_fault(
    state: &mut RuntimeState,
    fault: RecoveryFault,
    count: u8,
    action_fingerprint: Option<String>,
    summary: &str,
    effects: &mut Vec<Effect>,
) {
    let Some(graph) = state.graph.as_mut() else {
        return;
    };
    let kind = fault_kind(fault);
    set_graph_fault_count(graph, kind, count);
    graph.recovery.history.push(RecoveryRecord {
        kind,
        summary: summary.to_string(),
        action_fingerprint: action_fingerprint.clone(),
    });
    graph.health.recent_faults = graph.health.recent_faults.saturating_add(1);
    let Some(case_id) = graph.case_id else {
        return;
    };
    let key = retry_key(fault, action_fingerprint.as_deref());
    effects.push(Effect::RecordGraphFault {
        case_id,
        kind: fault_name(kind).to_string(),
        node: graph.active_node.0.to_string(),
        tool: key.tool,
        parameter_shape: key.parameter_shape,
        fault_class: key.fault_class,
        action_fingerprint,
        summary: summary.to_string(),
        count,
    });
}

pub(super) fn enter_recovery_route(
    mut state: RuntimeState,
    fault: RecoveryFault,
    count: u8,
    action_fingerprint: Option<String>,
    effects: &mut Vec<Effect>,
) -> RuntimeState {
    let notice = route_notice(fault, count);
    state = append_notice(state, NoticeKind::Error, &notice);
    effects.push(Effect::RecordEvent {
        kind: EventKind::Notice,
        content: notice.clone(),
        tokens: token_estimate(&notice) as i64,
    });
    route_graph(
        &mut state,
        fault,
        count,
        action_fingerprint,
        &notice,
        effects,
    );
    state
}

fn route_notice(fault: RecoveryFault, count: u8) -> String {
    let prefix = match fault {
        RecoveryFault::Parse => "Consecutive parse faults",
        RecoveryFault::Payload => "Consecutive large-payload parse faults",
        RecoveryFault::Params => "Consecutive parameter faults",
        RecoveryFault::Repeat => "Consecutive repeated actions",
        RecoveryFault::Tool => "Consecutive tool errors",
    };
    if matches!(fault, RecoveryFault::Payload) {
        return format!(
            "{prefix} reached count={count}; graph recovery is active. Use artifact.plan, artifact.next, doc.audit, or fs.batch_write. Raw fs.write remains blocked while payload risk is active."
        );
    }
    format!(
        "{prefix} reached count={count}; graph recovery is active. Use graph.recover, reduce scope, choose an alternate native tool, or replan around the blocked step."
    )
}

fn route_graph(
    state: &mut RuntimeState,
    fault: RecoveryFault,
    count: u8,
    action_fingerprint: Option<String>,
    notice: &str,
    effects: &mut Vec<Effect>,
) {
    let Some(graph) = state.graph.as_mut() else {
        return;
    };
    let from = graph.active_node;
    let kind = fault_kind(fault);
    let selection = recovery_transition(graph, fault);
    graph.phase = TaskPhase::Recovery;
    graph.status = CaseStatus::Active;
    graph.active_node = selection.target.unwrap_or(GraphNodeId("recover"));
    if let Some(action_class) = selection.forced_action_class {
        graph.next_action_class = action_class;
    }
    graph.recovery.ladder_position = count.min(5);
    graph.recovery.strategy = Some(notice.to_string());
    graph.recovery.last_failed_action_fingerprint = action_fingerprint.clone();
    let label = format!("{}-recovery", fault_name(kind));
    promote_recovery_track(
        &mut graph.state_tracks,
        &label,
        graph.active_node,
        graph.phase,
    );
    graph.objective.attach_tracks(&graph.state_tracks);
    graph.recovery.history.push(RecoveryRecord {
        kind,
        summary: notice.to_string(),
        action_fingerprint: action_fingerprint.clone(),
    });
    graph.health.recent_faults = graph.health.recent_faults.saturating_add(1);
    push_graph_effects(graph, from, notice, effects);
    state.context = append_frame(&state.context, graph_notice_frame(graph));
}

fn push_graph_effects(
    graph: &lkjagent_graph::TaskGraphState,
    from: GraphNodeId,
    notice: &str,
    effects: &mut Vec<Effect>,
) {
    let Some(case_id) = graph.case_id else {
        return;
    };
    effects.push(Effect::UpdateGraphRecovery {
        case_id,
        ladder_position: graph.recovery.ladder_position,
        strategy: notice.to_string(),
    });
    effects.push(Effect::RecordGraphTransition {
        case_id,
        from_node: from.0.to_string(),
        to_node: graph.active_node.0.to_string(),
        decision: "recovery-route".to_string(),
        reason: notice.to_string(),
    });
    effects.push(Effect::UpdateGraphCase {
        case_id,
        phase: graph.phase.as_str().to_string(),
        active_node: graph.active_node.0.to_string(),
        status: graph.status_text().to_string(),
    });
    effects.push(Effect::ReplaceGraphStateTracks {
        case_id,
        tracks: crate::graph_state_tracks::graph_track_effects(graph),
    });
}
