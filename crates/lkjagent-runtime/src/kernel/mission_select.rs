use crate::kernel::event::RuntimeEvent;
use crate::kernel::fault::{FaultClass, RuntimeFault};
use crate::kernel::mission::RuntimeMission;
use crate::kernel::snapshot::RuntimeSnapshot;

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
