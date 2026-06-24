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

impl ActiveModePolicy {
    pub fn blocked_preferred_tool(&self) -> Option<&'static str> {
        self.blocked_tools
            .iter()
            .copied()
            .find(|tool| self.preferred_next_action.contains(tool))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSnapshot {
    pub active_mode: ActiveMode,
    pub case_id: Option<String>,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub owner_work_exists: bool,
    pub recovery_ladder_active: bool,
    pub context_pressure_active: bool,
    pub maintenance_eligible: bool,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub active_artifact: Option<String>,
    pub last_tool_attempt: Option<String>,
    pub latest_fault: Option<RuntimeFault>,
    pub repeated_action: bool,
    pub external_owner_input_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeFault {
    Parse,
    Parameter,
    Schema,
    ToolRuntime,
    Repeat,
    PolicyContradiction,
    PayloadTooLarge,
    ArtifactAuditFailure,
    WeakArtifactContent,
    FalseCompletion,
    VerificationMismatch,
    CompletionRefused,
    CompactionPressure,
    CompactionResumeGap,
    MaintenanceConflict,
    EndpointFault,
    TurnBudgetExhausted,
    ContextInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FaultClass {
    Parse,
    Parameter,
    Tool,
    Repeat,
    Endpoint,
    Budget,
    Context,
    Verification,
    Compaction,
    Payload,
    Completion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryClass {
    ParseFault,
    ParameterFault,
    SchemaFault,
    ToolAdmissionContradiction,
    RepeatActionFault,
    PayloadOverflow,
    ArtifactAuditFailure,
    WeakArtifactContent,
    FalseCompletion,
    VerificationFailure,
    CompactionResumeGap,
    MaintenancePreemption,
    EndpointFault,
    TurnBudgetExhaustion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPlan {
    pub fault_class: FaultClass,
    pub recovery_class: RecoveryClass,
    pub recovery_route: String,
    pub previous_mission: ActiveMode,
    pub retry_budget: u8,
    pub allowed_observation_tools: Vec<String>,
    pub allowed_repair_tools: Vec<String>,
    pub forced_tool: String,
    pub forced_next_action: String,
    pub escalation_route: String,
    pub exact_valid_example: String,
    pub fallback_action: String,
    pub blocked_handoff_behavior: String,
    pub partial_handoff: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeEvent {
    OwnerMessageQueued,
    EndpointActionParsed { tool: String },
    EndpointActionParseFailed,
    ToolSucceeded,
    ToolFailed { fault: RuntimeFault },
    VerificationSucceeded,
    VerificationFailed,
    ContextPressureRaised,
    MaintenanceTick,
    CompletionRequested,
    QueueBecameNonEmpty,
    TurnBudgetCheckpoint,
    TurnBudgetExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAdmission {
    pub admitted: bool,
    pub reason: String,
    pub active_mode: ActiveMode,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub next_valid_tools: Vec<String>,
    pub exact_valid_example: Option<String>,
    pub contradiction: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeDecision {
    ExecuteTool(ToolAdmission),
    AskEndpoint,
    RefuseAction(ToolAdmission),
    StartRecovery(RecoveryPlan),
    ContinueRecovery {
        plan: RecoveryPlan,
        admission: ToolAdmission,
    },
    StartCompaction,
    StartMaintenance,
    StartVerification,
    CloseCase,
    BlockCompletion(ToolAdmission),
}
