#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveMode {
    OwnerTask,
    Recovery,
    Maintenance,
    Compaction,
    ClosedIdle,
}

impl ActiveMode {
    pub fn allows_completion(self) -> bool {
        matches!(self, Self::OwnerTask | Self::Recovery | Self::Maintenance)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ActiveModeInput {
    pub pending_owner_rows: usize,
    pub active_owner_case: bool,
    pub recoverable_owner_case: bool,
    pub compaction_required: bool,
    pub maintenance_due: bool,
    pub maintenance_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveModePolicy {
    pub mode: ActiveMode,
    pub allowed_tools: Vec<&'static str>,
    pub blocked_tools: Vec<&'static str>,
    pub preferred_next_action: &'static str,
    pub completion_condition: &'static str,
    pub graph_policy_applies: bool,
    pub maintenance_policy_applies: bool,
    pub compaction_policy_applies: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSnapshot {
    pub active_mission: ActiveMode,
    pub owner_work_exists: bool,
    pub recovery_ladder_active: bool,
    pub context_pressure_active: bool,
    pub maintenance_eligible: bool,
    pub required_evidence: Vec<&'static str>,
    pub missing_evidence: Vec<&'static str>,
    pub active_artifact: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeEvent {
    OwnerMessageQueued,
    EndpointActionParsed,
    EndpointActionParseFailed,
    ToolSucceeded,
    ToolFailed,
    VerificationSucceeded,
    VerificationFailed,
    ContextPressureRaised,
    MaintenanceTick,
    CompletionRequested,
    QueueBecameNonEmpty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAdmission {
    pub admitted: bool,
    pub reason: &'static str,
    pub active_mission: ActiveMode,
    pub required_evidence: Vec<&'static str>,
    pub missing_evidence: Vec<&'static str>,
    pub next_valid_tools: Vec<&'static str>,
    pub exact_valid_example: Option<&'static str>,
    pub contradiction: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeDecision {
    ExecuteTool(ToolAdmission),
    AskEndpoint,
    RefuseAction(ToolAdmission),
    StartRecovery,
    ContinueRecovery(ToolAdmission),
    StartCompaction,
    StartMaintenance,
    StartVerification,
    CloseCase,
    BlockCompletion(ToolAdmission),
}
