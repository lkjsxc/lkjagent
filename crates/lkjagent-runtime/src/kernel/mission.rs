use crate::kernel::active_mode::ActiveMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const PRIORITY: [Self; 10] = [
        Self::HardRuntimeCompaction,
        Self::OwnerRecovery,
        Self::SchemaRepair,
        Self::ArtifactRepair,
        Self::VerificationRepair,
        Self::OwnerExecution,
        Self::OwnerVerification,
        Self::OwnerCompletion,
        Self::IdleMaintenance,
        Self::ClosedIdle,
    ];

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
