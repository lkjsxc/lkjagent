use crate::kernel::admission::{admitted_tools_for, blocked_tools_for, ToolAdmissionView};
use crate::kernel::completion::close_allowed;
use crate::kernel::decision::{
    ActionTemplate, DecisionInvariantError, RuntimeDecision, RuntimeDecisionId,
    RuntimeDecisionInput, RuntimeDecisionKind, RuntimeMission,
};
use crate::kernel::effect::attach_runtime_effect;
use crate::kernel::event::RuntimeEvent;
use crate::kernel::fault::{FaultClass, RuntimeFault};
use crate::kernel::next_action::next_action_for;
use crate::kernel::render::prompt_card_for;
use crate::kernel::repeat_guard::repeat_guard;
use crate::kernel::snapshot::{RuntimeEventId, RuntimeSnapshot, ToolName};

pub fn select_mission(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> RuntimeMission {
    if snapshot.context.hard_pressure || matches!(event, RuntimeEvent::ContextPressureDetected) {
        return RuntimeMission::HardRuntimeCompaction;
    }
    if is_owner_recovery(snapshot, event) {
        return RuntimeMission::OwnerRecovery;
    }
    if is_schema_repair(snapshot, event) {
        return RuntimeMission::SchemaRepair;
    }
    if snapshot.artifact.needs_repair()
        || matches!(
            event,
            RuntimeEvent::ArtifactWeakPathFound | RuntimeEvent::ArtifactAuditFailed
        )
    {
        return RuntimeMission::ArtifactRepair;
    }
    if is_verification_repair(snapshot, event) {
        return RuntimeMission::VerificationRepair;
    }
    if snapshot.owner_work_exists() {
        return owner_mission(event);
    }
    if matches!(
        event,
        RuntimeEvent::MaintenanceNoop
            | RuntimeEvent::MaintenanceNoopCooldownRecorded
            | RuntimeEvent::MaintenanceDeferred
    ) {
        return RuntimeMission::ClosedIdle;
    }
    if (snapshot.maintenance.due || snapshot.maintenance.active)
        && !snapshot.maintenance.cooldown_active
    {
        return RuntimeMission::IdleMaintenance;
    }
    RuntimeMission::ClosedIdle
}
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
    decision.graph_node = snapshot.graph.node.clone();
    decision.graph_phase = snapshot.graph.phase.clone();
    decision.missing_evidence = snapshot.evidence.missing.clone();
    decision.existing_evidence = snapshot.evidence.existing.clone();
    decision.completion_allowed = close_case;
    decision.completion_refusal = (mission == RuntimeMission::OwnerCompletion && !close_case)
        .then(|| snapshot.evidence.missing.join(","));
    decision.context_package_ids = snapshot.graph.context_package_ids.clone();
    decision.forced_next_action = next_action_for(mission, snapshot);
    if let Some(ActionTemplate::ExactTool { body, tool }) = &decision.forced_next_action {
        if !decision.admission_view.admits(tool) {
            decision.admission_view.admitted_tools.push(tool.clone());
        }
        decision.admission_view.exact_next_action = Some(body.clone());
    }
    decision.prompt_card = prompt_card_for(snapshot, mission, active_mode, &decision);
    attach_runtime_effect(decision, mission)
}

fn is_schema_repair(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> bool {
    matches!(event, RuntimeEvent::SchemaFault { .. })
        || snapshot
            .latest_fault
            .is_some_and(|fault| fault.class() == FaultClass::Schema)
}

fn is_owner_recovery(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> bool {
    is_owner_recovery_event(event)
        || snapshot.provider.anomaly_class.is_some()
        || snapshot.latest_fault.is_some_and(is_owner_recovery_fault)
        || snapshot.recovery_route.is_some()
}

fn is_verification_repair(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> bool {
    matches!(event, RuntimeEvent::VerificationFailed)
        || snapshot
            .latest_fault
            .is_some_and(|fault| fault.class() == FaultClass::Verification)
}

fn owner_mission(event: &RuntimeEvent) -> RuntimeMission {
    if is_completion_event(event) {
        RuntimeMission::OwnerCompletion
    } else if matches!(event, RuntimeEvent::VerificationRequested) {
        RuntimeMission::OwnerVerification
    } else {
        RuntimeMission::OwnerExecution
    }
}

fn is_completion_event(event: &RuntimeEvent) -> bool {
    matches!(
        event,
        RuntimeEvent::CompletionRequested
            | RuntimeEvent::CompletionAccepted
            | RuntimeEvent::CompletionRefused
            | RuntimeEvent::CompletionBlocked
            | RuntimeEvent::CaseClosed
    )
}

fn is_owner_recovery_event(event: &RuntimeEvent) -> bool {
    matches!(
        event,
        RuntimeEvent::ParseFault { .. }
            | RuntimeEvent::EndpointFault { .. }
            | RuntimeEvent::ProviderAnomaly { .. }
            | RuntimeEvent::AdmissionRefused { .. }
            | RuntimeEvent::StaleActionRefused { .. }
            | RuntimeEvent::RepeatedActionRefused { .. }
            | RuntimeEvent::RepeatActionDetected { .. }
            | RuntimeEvent::PayloadOverflowDetected { .. }
            | RuntimeEvent::ToolFailed { .. }
            | RuntimeEvent::TurnBudgetExhausted
    )
}

fn is_owner_recovery_fault(fault: RuntimeFault) -> bool {
    matches!(
        fault.class(),
        FaultClass::Parse
            | FaultClass::Parameter
            | FaultClass::Tool
            | FaultClass::Repeat
            | FaultClass::Endpoint
            | FaultClass::Budget
            | FaultClass::Context
            | FaultClass::Payload
            | FaultClass::Completion
            | FaultClass::MaintenanceConflict
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
