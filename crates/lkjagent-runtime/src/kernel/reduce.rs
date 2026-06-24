use crate::kernel::admission::{admitted_tools_for, blocked_tools_for, ToolAdmissionView};
use crate::kernel::decision::{
    ActionTemplate, DecisionInvariantError, RuntimeDecision, RuntimeDecisionId,
    RuntimeDecisionInput, RuntimeDecisionKind, RuntimeMission,
};
use crate::kernel::effect::RuntimeEffectCommand;
use crate::kernel::event::RuntimeEvent;
use crate::kernel::fault::{FaultClass, RuntimeFault};
use crate::kernel::render::{example_for, prompt_card_for};
use crate::kernel::snapshot::{RuntimeEventId, RuntimeSnapshot, ToolName};

pub fn select_mission(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> RuntimeMission {
    if snapshot.context.hard_pressure || matches!(event, RuntimeEvent::ContextPressureDetected) {
        return RuntimeMission::HardRuntimeCompaction;
    }
    if is_schema_repair(snapshot, event) {
        return RuntimeMission::SchemaRepair;
    }
    if is_owner_recovery(snapshot, event) {
        return RuntimeMission::OwnerRecovery;
    }
    if snapshot.artifact.needs_repair() || matches!(event, RuntimeEvent::ArtifactWeakPathFound) {
        return RuntimeMission::ArtifactRepair;
    }
    if is_verification_repair(snapshot, event) {
        return RuntimeMission::VerificationRepair;
    }
    if snapshot.owner_work_exists() {
        return owner_mission(event);
    }
    if (snapshot.maintenance.due || snapshot.maintenance.active)
        && !snapshot.maintenance.cooldown_active
    {
        return RuntimeMission::IdleMaintenance;
    }
    RuntimeMission::ClosedIdle
}

pub fn reduce(
    snapshot: &RuntimeSnapshot,
    event: RuntimeEvent,
) -> Result<RuntimeDecision, DecisionInvariantError> {
    let mission = select_mission(snapshot, &event);
    let active_mode = mission.active_mode();
    let admission_view = ToolAdmissionView::new(
        active_mode,
        admitted_tools_for(mission),
        blocked_tools_for(mission),
        snapshot.staleness_fingerprint.clone(),
    )
    .with_missing_evidence(snapshot.evidence.missing.clone());
    let input = RuntimeDecisionInput {
        decision_id: RuntimeDecisionId::Pending,
        snapshot_id: snapshot.snapshot_id,
        event_id: RuntimeEventId(0),
        mission,
        kind: decision_kind_for(mission),
        admission_view,
        authority_fingerprint: snapshot.authority_fingerprint.clone(),
        staleness_fingerprint: snapshot.staleness_fingerprint.clone(),
    };
    let mut decision = RuntimeDecision::new(input)?;
    decision.graph_node = snapshot.graph.node.clone();
    decision.graph_phase = snapshot.graph.phase.clone();
    decision.missing_evidence = snapshot.evidence.missing.clone();
    decision.existing_evidence = snapshot.evidence.existing.clone();
    decision.context_package_ids = snapshot.graph.context_package_ids.clone();
    decision.forced_next_action = next_action_for(mission, snapshot);
    if let Some(ActionTemplate::ExactTool { body, .. }) = &decision.forced_next_action {
        decision.admission_view.exact_next_action = Some(body.clone());
    }
    decision.prompt_card = prompt_card_for(snapshot, mission, active_mode, &decision);
    with_runtime_effect(decision, mission)
}

fn is_schema_repair(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> bool {
    matches!(event, RuntimeEvent::SchemaFault { .. })
        || snapshot
            .latest_fault
            .is_some_and(|fault| fault.class() == FaultClass::Schema)
}

fn is_owner_recovery(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> bool {
    is_owner_recovery_event(event)
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
    if matches!(event, RuntimeEvent::CompletionRequested) {
        RuntimeMission::OwnerCompletion
    } else if matches!(event, RuntimeEvent::VerificationRequested) {
        RuntimeMission::OwnerVerification
    } else {
        RuntimeMission::OwnerExecution
    }
}

fn is_owner_recovery_event(event: &RuntimeEvent) -> bool {
    matches!(
        event,
        RuntimeEvent::ParseFault { .. }
            | RuntimeEvent::EndpointFault { .. }
            | RuntimeEvent::AdmissionRefused { .. }
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

fn decision_kind_for(mission: RuntimeMission) -> RuntimeDecisionKind {
    match mission {
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle => {
            RuntimeDecisionKind::RuntimeEffect
        }
        RuntimeMission::OwnerCompletion => RuntimeDecisionKind::BlockCompletion,
        _ => RuntimeDecisionKind::ModelCall,
    }
}

fn next_action_for(mission: RuntimeMission, snapshot: &RuntimeSnapshot) -> Option<ActionTemplate> {
    let tool = match mission {
        RuntimeMission::HardRuntimeCompaction => return None,
        RuntimeMission::OwnerRecovery => "graph.state",
        RuntimeMission::SchemaRepair => "fs.batch_write",
        RuntimeMission::ArtifactRepair => "artifact.next",
        RuntimeMission::VerificationRepair => "artifact.audit",
        RuntimeMission::OwnerExecution => "artifact.next",
        RuntimeMission::OwnerVerification => "artifact.audit",
        RuntimeMission::OwnerCompletion => "agent.done",
        RuntimeMission::IdleMaintenance => "memory.find",
        RuntimeMission::ClosedIdle => return None,
    };
    Some(ActionTemplate::ExactTool {
        tool: ToolName::from_static(tool),
        body: example_for(tool, snapshot),
    })
}

fn with_runtime_effect(
    decision: RuntimeDecision,
    mission: RuntimeMission,
) -> Result<RuntimeDecision, DecisionInvariantError> {
    match mission {
        RuntimeMission::HardRuntimeCompaction => {
            decision.with_runtime_effect(RuntimeEffectCommand::CompactNow)
        }
        RuntimeMission::ClosedIdle => {
            decision.with_runtime_effect(RuntimeEffectCommand::WaitClosedIdle)
        }
        _ => Ok(decision),
    }
}
