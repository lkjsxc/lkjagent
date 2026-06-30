use crate::kernel::admission::{admitted_tools_for, blocked_tools_for, ToolAdmissionView};
use crate::kernel::authority_ledger::authority_ledger_entries;
use crate::kernel::completion::{completion_gate, CompletionGateDecision};
use crate::kernel::completion_inputs::completion_inputs;
use crate::kernel::decision::{
    DecisionInvariantError, RuntimeDecision, RuntimeDecisionId, RuntimeDecisionInput,
    RuntimeDecisionKind, RuntimeMission,
};
use crate::kernel::decision_apply::apply_forced_action;
use crate::kernel::effect::attach_runtime_effect;
use crate::kernel::event::RuntimeEvent;
use crate::kernel::mission_select::select_mission;
use crate::kernel::obligation::obligations_for;
use crate::kernel::obligation_facts::runtime_facts;
use crate::kernel::render::prompt_card_for;
use crate::kernel::repeat_guard::repeat_guard;
use crate::kernel::resolver::{
    action_for_plan, resolve_obligations, resolver_label, resolver_rule_id, ResolverPlan,
};
use crate::kernel::snapshot::{RuntimeEventId, RuntimeSnapshot, ToolName};
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
    let facts = runtime_facts(snapshot, &event);
    let obligations = obligations_for(&facts);
    let completion = completion_gate(snapshot);
    let plan = resolve_obligations(mission, snapshot, &facts, &obligations, completion.allowed);
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
    if matches!(plan, ResolverPlan::CloseCase) {
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
        kind: decision_kind_for(mission, &plan),
        admission_view,
        authority_fingerprint: snapshot.authority_fingerprint.clone(),
        staleness_fingerprint: snapshot.staleness_fingerprint.clone(),
    };
    let mut decision = RuntimeDecision::new(input)?;
    populate_decision(snapshot, &event, &mut decision, &plan, &completion);
    attach_runtime_effect(decision, mission)
}
fn populate_decision(
    snapshot: &RuntimeSnapshot,
    event: &RuntimeEvent,
    decision: &mut RuntimeDecision,
    plan: &ResolverPlan,
    completion: &CompletionGateDecision,
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
    decision.completion_allowed = completion.allowed;
    decision.completion_blockers = completion.missing_inputs.clone();
    decision.completion_gate_inputs = completion_inputs(completion);
    decision.completion_refusal = completion_refusal(decision);
    decision.context_package_ids = snapshot.graph.context_package_ids.clone();
    decision.resolver_plan = Some(resolver_label(plan));
    if let ResolverPlan::BlockedHandoff { reason } = plan {
        decision.blocked_handoff_plan = Some(reason.clone());
    }
    decision.progress_key = Some(progress_key(decision, plan));
    decision.rule_explanation = rule_explanation(decision.mission, event, plan);
    decision.forced_next_action = action_for_plan(plan, snapshot);
    apply_forced_action(snapshot, decision);
    decision.persistence_plan = authority_ledger_entries(snapshot, event, decision);
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

fn compaction_policy(snapshot: &RuntimeSnapshot) -> Option<String> {
    if snapshot.context.hard_pressure {
        Some("hard_runtime_compaction".to_string())
    } else {
        snapshot.context.compaction_head.clone()
    }
}
fn completion_refusal(decision: &RuntimeDecision) -> Option<String> {
    (decision.mission == RuntimeMission::OwnerCompletion && !decision.completion_allowed)
        .then(|| decision.completion_blockers.join(","))
}

fn progress_key(decision: &RuntimeDecision, plan: &ResolverPlan) -> String {
    format!(
        "mission={} plan={} root={} cursor={}",
        decision.mission.as_str(),
        resolver_label(plan),
        decision.root.as_deref().unwrap_or("none"),
        decision.cursor.as_deref().unwrap_or("none")
    )
}
fn rule_explanation(mission: RuntimeMission, event: &RuntimeEvent, plan: &ResolverPlan) -> String {
    format!(
        "selected_rule={} mission={} resolver_plan={} event={}",
        resolver_rule_id(plan),
        mission.as_str(),
        resolver_label(plan),
        event.kind().as_str()
    )
}

fn decision_kind_for(mission: RuntimeMission, plan: &ResolverPlan) -> RuntimeDecisionKind {
    match plan {
        ResolverPlan::CloseCase => RuntimeDecisionKind::CloseCase,
        ResolverPlan::RuntimeEffect => RuntimeDecisionKind::RuntimeEffect,
        ResolverPlan::OwnerWait => RuntimeDecisionKind::WaitForOwner,
        ResolverPlan::BlockedHandoff { .. } => RuntimeDecisionKind::RuntimeEffect,
        ResolverPlan::ExactInspection { .. } | ResolverPlan::Audit { .. } => {
            RuntimeDecisionKind::RuntimeEffect
        }
        ResolverPlan::SemanticWriteContract { .. } | ResolverPlan::EvidenceRecording { .. } => {
            if mission == RuntimeMission::OwnerCompletion {
                RuntimeDecisionKind::BlockCompletion
            } else {
                RuntimeDecisionKind::ModelCall
            }
        }
    }
}
