use super::model::ActiveMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionPolicy {
    OwnerTask(OwnerCompletionGate),
    Recovery(RecoveryCompletionGate),
    Maintenance(MaintenanceCompletionGate),
    Compaction(RuntimeOnlyCompletionGate),
    ClosedIdle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerCompletionGate {
    pub requires_plan: bool,
    pub requires_observation: bool,
    pub requires_verification: bool,
    pub requires_artifact_readiness: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryCompletionGate {
    pub requires_fault_resolution: bool,
    pub allows_blocked_handoff: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintenanceCompletionGate {
    pub requires_real_effect_or_noop_cooldown: bool,
    pub forbids_duplicate_memory_row: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOnlyCompletionGate {
    pub model_done_allowed: bool,
    pub runtime_snapshot_required: bool,
}

pub fn completion_policy_for(mode: ActiveMode) -> CompletionPolicy {
    match mode {
        ActiveMode::OwnerTask => CompletionPolicy::OwnerTask(OwnerCompletionGate {
            requires_plan: true,
            requires_observation: true,
            requires_verification: true,
            requires_artifact_readiness: true,
        }),
        ActiveMode::Recovery => CompletionPolicy::Recovery(RecoveryCompletionGate {
            requires_fault_resolution: true,
            allows_blocked_handoff: true,
        }),
        ActiveMode::Maintenance => CompletionPolicy::Maintenance(MaintenanceCompletionGate {
            requires_real_effect_or_noop_cooldown: true,
            forbids_duplicate_memory_row: true,
        }),
        ActiveMode::Compaction => CompletionPolicy::Compaction(RuntimeOnlyCompletionGate {
            model_done_allowed: false,
            runtime_snapshot_required: true,
        }),
        ActiveMode::ClosedIdle => CompletionPolicy::ClosedIdle,
    }
}
