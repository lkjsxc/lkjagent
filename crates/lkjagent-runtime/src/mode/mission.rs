use super::input::TurnAuthorityInput;
use super::model::{ActiveMode, RuntimeSnapshot};

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

pub fn select_runtime_mission(input: TurnAuthorityInput) -> RuntimeMission {
    if input.compaction_required {
        return RuntimeMission::HardRuntimeCompaction;
    }
    if input.recoverable_owner_case {
        return RuntimeMission::OwnerRecovery;
    }
    if input.pending_owner_rows > 0 || input.active_owner_case {
        return RuntimeMission::OwnerExecution;
    }
    if input.maintenance_due || input.maintenance_active {
        return RuntimeMission::IdleMaintenance;
    }
    RuntimeMission::ClosedIdle
}

pub fn mission_for_snapshot(snapshot: &RuntimeSnapshot) -> RuntimeMission {
    if snapshot.context_pressure_active {
        RuntimeMission::HardRuntimeCompaction
    } else if snapshot.recovery_ladder_active {
        RuntimeMission::OwnerRecovery
    } else {
        RuntimeMission::from(snapshot.active_mission)
    }
}
