use super::model::{ActiveMode, RuntimeSnapshot};
use crate::kernel::{
    build_snapshot, RuntimeEvent, RuntimeFault as KernelFault, RuntimeMission as KernelMission,
    SnapshotAdapterInput,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimeMission {
    HardRuntimeCompaction,
    OwnerRecovery,
    SchemaRepair,
    ArtifactRepair,
    VerificationRepair,
    OwnerExecution,
    OwnerVerification,
    OwnerCompletion,
    IdleMaintenance,
    ClosedIdle,
}

impl RuntimeMission {
    pub fn active_mode(self) -> ActiveMode {
        match self {
            Self::HardRuntimeCompaction => ActiveMode::Compaction,
            Self::OwnerRecovery
            | Self::SchemaRepair
            | Self::ArtifactRepair
            | Self::VerificationRepair => ActiveMode::Recovery,
            Self::OwnerExecution | Self::OwnerVerification | Self::OwnerCompletion => {
                ActiveMode::OwnerTask
            }
            Self::IdleMaintenance => ActiveMode::Maintenance,
            Self::ClosedIdle => ActiveMode::ClosedIdle,
        }
    }

    pub fn from_kernel(value: KernelMission) -> Self {
        match value {
            KernelMission::HardRuntimeCompaction => Self::HardRuntimeCompaction,
            KernelMission::OwnerRecovery => Self::OwnerRecovery,
            KernelMission::SchemaRepair => Self::SchemaRepair,
            KernelMission::ArtifactRepair => Self::ArtifactRepair,
            KernelMission::VerificationRepair => Self::VerificationRepair,
            KernelMission::OwnerExecution => Self::OwnerExecution,
            KernelMission::OwnerVerification => Self::OwnerVerification,
            KernelMission::OwnerCompletion => Self::OwnerCompletion,
            KernelMission::IdleMaintenance => Self::IdleMaintenance,
            KernelMission::ClosedIdle => Self::ClosedIdle,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::HardRuntimeCompaction => "hard_runtime_compaction",
            Self::OwnerRecovery => "owner_recovery",
            Self::SchemaRepair => "schema_repair",
            Self::ArtifactRepair => "artifact_repair",
            Self::VerificationRepair => "verification_repair",
            Self::OwnerExecution => "owner_execution",
            Self::OwnerVerification => "owner_verification",
            Self::OwnerCompletion => "owner_completion",
            Self::IdleMaintenance => "idle_maintenance",
            Self::ClosedIdle => "closed_idle",
        }
    }
}

impl From<RuntimeMission> for ActiveMode {
    fn from(value: RuntimeMission) -> Self {
        value.active_mode()
    }
}

impl From<ActiveMode> for RuntimeMission {
    fn from(value: ActiveMode) -> Self {
        match value {
            ActiveMode::OwnerTask => Self::OwnerExecution,
            ActiveMode::Recovery => Self::OwnerRecovery,
            ActiveMode::Maintenance => Self::IdleMaintenance,
            ActiveMode::Compaction => Self::HardRuntimeCompaction,
            ActiveMode::ClosedIdle => Self::ClosedIdle,
        }
    }
}

pub fn select_runtime_mission(snapshot: &RuntimeSnapshot) -> RuntimeMission {
    kernel_mission(snapshot).map_or_else(|| legacy_mission(snapshot), RuntimeMission::from_kernel)
}

fn kernel_mission(snapshot: &RuntimeSnapshot) -> Option<KernelMission> {
    let input = SnapshotAdapterInput {
        case_id: adapter_case_id(snapshot),
        graph_node: snapshot.graph_node.clone(),
        graph_phase: snapshot.graph_phase.clone(),
        pending_owner_count: usize::from(snapshot.owner_work_exists),
        required_evidence: snapshot.required_evidence.clone(),
        missing_evidence: snapshot.missing_evidence.clone(),
        artifact_root: snapshot.active_artifact.clone(),
        context_hard_pressure: snapshot.context_pressure_active,
        maintenance_due: snapshot.maintenance_eligible,
        latest_fault: snapshot
            .recovery_ladder_active
            .then_some(KernelFault::TurnBudgetExhausted),
        ..SnapshotAdapterInput::default()
    };
    let event = if snapshot.recovery_ladder_active {
        RuntimeEvent::TurnBudgetExhausted
    } else {
        RuntimeEvent::CaseResumed
    };
    build_snapshot(input)
        .ok()
        .map(|built| crate::kernel::select_mission(&built, &event))
}

fn adapter_case_id(snapshot: &RuntimeSnapshot) -> Option<String> {
    snapshot.case_id.clone().or_else(|| {
        snapshot
            .owner_work_exists
            .then(|| "mode-adapter".to_string())
    })
}

fn legacy_mission(snapshot: &RuntimeSnapshot) -> RuntimeMission {
    if snapshot.context_pressure_active {
        return RuntimeMission::HardRuntimeCompaction;
    }
    if snapshot.recovery_ladder_active {
        return RuntimeMission::OwnerRecovery;
    }
    if snapshot.owner_work_exists {
        return RuntimeMission::OwnerExecution;
    }
    if snapshot.maintenance_eligible {
        return RuntimeMission::IdleMaintenance;
    }
    RuntimeMission::ClosedIdle
}

pub fn mission_for_snapshot(snapshot: &RuntimeSnapshot) -> RuntimeMission {
    select_runtime_mission(snapshot)
}
