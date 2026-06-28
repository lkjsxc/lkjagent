use crate::kernel::admission::{admitted_tools_for, blocked_tools_for, ToolAdmissionView};
use crate::kernel::completion::close_allowed;
use crate::kernel::decision::{
    ActionTemplate, DecisionInvariantError, RuntimeDecision, RuntimeDecisionId,
    RuntimeDecisionInput, RuntimeDecisionKind, RuntimeMission,
};
use crate::kernel::effect::attach_runtime_effect;
use crate::kernel::event::RuntimeEvent;
use crate::kernel::mission_select::select_mission;
use crate::kernel::next_action::next_action_for;
use crate::kernel::render::prompt_card_for;
use crate::kernel::repeat_guard::repeat_guard;
use crate::kernel::snapshot::{RuntimeEventId, RuntimeSnapshot, ToolName};
use crate::kernel::write_contract::content_contract_for;

pub fn reduce_with_event_id(
    snapshot: &RuntimeSnapshot,
    event_id: RuntimeEventId,
    event: RuntimeEvent,
) -> Result<RuntimeDecision, DecisionInvariantError> {
    let mut decision = reduce(snapshot, event)?;
    decision.event_id = event_id;
    Ok(decision)
}

pub fn reduce(
    snapshot: &RuntimeSnapshot,
    event: RuntimeEvent,
) -> Result<RuntimeDecision, DecisionInvariantError> {
    let mission = select_mission(snapshot, &event);
    let active_mode = mission.active_mode();
    let mut admission_view = repeat_guard(
        ToolAdmissionView::new(
            active_mode,
            admitted_tools_for(mission),
            blocked_tools_for(mission),
            snapshot.staleness_fingerprint.clone(),
        )
        .with_missing_evidence(snapshot.evidence.missing.clone()),
        snapshot,
    );
    block_direct_audit_owned_evidence(snapshot, &mut admission_view);
    let close_case = mission == RuntimeMission::OwnerCompletion && close_allowed(snapshot);
    if close_case {
        admission_view.completion_allowed = true;
        admission_view
            .admitted_tools
            .push(ToolName::from_static("agent.done"));
    }
    let input = RuntimeDecisionInput {
        decision_id: RuntimeDecisionId::Pending,
        snapshot_id: snapshot.snapshot_id,
        event_id: RuntimeEventId(0),
        mission,
        kind: decision_kind_for(mission, close_case),
        admission_view,
        authority_fingerprint: snapshot.authority_fingerprint.clone(),
        staleness_fingerprint: snapshot.staleness_fingerprint.clone(),
    };
    let mut decision = RuntimeDecision::new(input)?;
    populate_decision(snapshot, &event, &mut decision, close_case);
    attach_runtime_effect(decision, mission)
}

fn populate_decision(
    snapshot: &RuntimeSnapshot,
    event: &RuntimeEvent,
    decision: &mut RuntimeDecision,
    close_case: bool,
) {
    decision.case_id = snapshot.case.case_id.clone();
    decision.graph_node = snapshot.graph.node.clone();
    decision.graph_phase = snapshot.graph.phase.clone();
    decision.missing_evidence = snapshot.evidence.missing.clone();
    decision.existing_evidence = snapshot.evidence.existing.clone();
    decision.artifact_id = snapshot.artifact.artifact_id.clone();
    decision.root = snapshot.artifact.root.clone();
    decision.artifact_kind = snapshot.artifact.kind.clone();
    decision.artifact_profile = snapshot.artifact.profile.clone();
    decision.weak_paths = snapshot.artifact.weak_paths.clone();
    decision.cursor = snapshot
        .artifact
        .cursor
        .clone()
        .or(snapshot.artifact.batch_cursor.clone());
    decision.fault_class = snapshot.latest_fault.map(|fault| fault.class());
    decision.retry_count = snapshot.retry_count;
    decision.provider_anomaly_budget = 3_u32.saturating_sub(snapshot.provider.retry_count);
    decision.compaction_policy = compaction_policy(snapshot);
    decision.completion_allowed = close_case;
    decision.completion_blockers = snapshot.evidence.missing.clone();
    decision.completion_refusal = completion_refusal(decision, close_case);
    decision.context_package_ids = snapshot.graph.context_package_ids.clone();
    decision.rule_explanation = rule_explanation(decision.mission, event);
    decision.forced_next_action = next_action_for(decision.mission, snapshot);
    apply_forced_action(snapshot, decision);
    decision.prompt_card =
        prompt_card_for(snapshot, decision.mission, decision.active_mode, decision);
}

fn block_direct_audit_owned_evidence(
    snapshot: &RuntimeSnapshot,
    admission_view: &mut ToolAdmissionView,
) {
    if !only_audit_owned_gaps(snapshot) {
        return;
    }
    admission_view
        .admitted_tools
        .retain(|tool| tool.as_str() != "graph.evidence");
    if !admission_view
        .blocked_tools
        .iter()
        .any(|tool| tool.as_str() == "graph.evidence")
    {
        admission_view
            .blocked_tools
            .push(ToolName::from_static("graph.evidence"));
    }
}

fn only_audit_owned_gaps(snapshot: &RuntimeSnapshot) -> bool {
    !snapshot.evidence.missing.is_empty()
        && snapshot
            .evidence
            .missing
            .iter()
            .all(|item| matches!(item.as_str(), "document-structure" | "artifact-readiness"))
}

fn apply_forced_action(snapshot: &RuntimeSnapshot, decision: &mut RuntimeDecision) {
    let Some(ActionTemplate::ExactTool { body, tool }) = &decision.forced_next_action else {
        return;
    };
    if !decision.admission_view.admits(tool) {
        decision.admission_view.admitted_tools.push(tool.clone());
    }
    if tool.as_str() == "fs.batch_write" {
        decision.content_write_contract = content_contract_for(snapshot);
    } else {
        decision.admission_view.exact_next_action = Some(body.clone());
    }
}

fn compaction_policy(snapshot: &RuntimeSnapshot) -> Option<String> {
    if snapshot.context.hard_pressure {
        Some("hard_runtime_compaction".to_string())
    } else {
        snapshot.context.compaction_head.clone()
    }
}

fn completion_refusal(decision: &RuntimeDecision, close_case: bool) -> Option<String> {
    (decision.mission == RuntimeMission::OwnerCompletion && !close_case)
        .then(|| decision.completion_blockers.join(","))
}

fn rule_explanation(mission: RuntimeMission, event: &RuntimeEvent) -> String {
    format!(
        "first_matching_rule={} event={}",
        mission.as_str(),
        event.kind().as_str()
    )
}

fn decision_kind_for(mission: RuntimeMission, close_case: bool) -> RuntimeDecisionKind {
    match mission {
        RuntimeMission::OwnerCompletion if close_case => RuntimeDecisionKind::CloseCase,
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle => {
            RuntimeDecisionKind::RuntimeEffect
        }
        RuntimeMission::OwnerCompletion => RuntimeDecisionKind::BlockCompletion,
        _ => RuntimeDecisionKind::ModelCall,
    }
}
