use crate::kernel::active_mode::ActiveMode;
use crate::kernel::admission::ToolAdmissionView;
use crate::kernel::effect::RuntimeEffectCommand;
use crate::kernel::render::PromptCardData;
use crate::kernel::snapshot::{
    AuthorityFingerprint, RuntimeEventId, RuntimeSnapshotId, StalenessFingerprint, ToolName,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeDecisionId {
    Pending,
    Stored(u64),
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeDecisionKind {
    ModelCall,
    RuntimeEffect,
    AdmitDispatch,
    RefuseAdmission,
    BlockCompletion,
    CloseCase,
    WaitForOwner,
    ClosedIdle,
}

impl RuntimeDecisionKind {
    pub fn requires_model_tool_surface(self) -> bool {
        matches!(self, Self::ModelCall)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionTemplate {
    ExactTool { tool: ToolName, body: String },
    RuntimeEffect(RuntimeEffectCommand),
    ExternalOwnerWait,
    ClosedIdle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionInvariantError {
    ModelCallWithoutAdmittedTools,
    RuntimeEffectWithoutCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDecisionInput {
    pub decision_id: RuntimeDecisionId,
    pub snapshot_id: RuntimeSnapshotId,
    pub event_id: RuntimeEventId,
    pub mission: RuntimeMission,
    pub kind: RuntimeDecisionKind,
    pub admission_view: ToolAdmissionView,
    pub authority_fingerprint: AuthorityFingerprint,
    pub staleness_fingerprint: StalenessFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDecision {
    pub decision_id: RuntimeDecisionId,
    pub snapshot_id: RuntimeSnapshotId,
    pub event_id: RuntimeEventId,
    pub mission: RuntimeMission,
    pub active_mode: ActiveMode,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub kind: RuntimeDecisionKind,
    pub admission_view: ToolAdmissionView,
    pub forced_next_action: Option<ActionTemplate>,
    pub recommended_next_actions: Vec<ActionTemplate>,
    pub missing_evidence: Vec<String>,
    pub existing_evidence: Vec<String>,
    pub completion_allowed: bool,
    pub completion_refusal: Option<String>,
    pub recovery_plan: Option<String>,
    pub compaction_plan: Option<String>,
    pub maintenance_plan: Option<String>,
    pub blocked_handoff_plan: Option<String>,
    pub context_package_ids: Vec<String>,
    pub prompt_card: Option<PromptCardData>,
    pub persistence_plan: Vec<String>,
    pub authority_fingerprint: AuthorityFingerprint,
    pub staleness_fingerprint: StalenessFingerprint,
    pub runtime_effect: Option<RuntimeEffectCommand>,
}

impl RuntimeDecision {
    pub fn new(input: RuntimeDecisionInput) -> Result<Self, DecisionInvariantError> {
        if input.kind.requires_model_tool_surface()
            && input.admission_view.admitted_tools.is_empty()
        {
            return Err(DecisionInvariantError::ModelCallWithoutAdmittedTools);
        }
        Ok(Self {
            decision_id: input.decision_id,
            snapshot_id: input.snapshot_id,
            event_id: input.event_id,
            mission: input.mission,
            active_mode: input.mission.active_mode(),
            graph_node: None,
            graph_phase: None,
            kind: input.kind,
            admission_view: input.admission_view,
            forced_next_action: None,
            recommended_next_actions: Vec::new(),
            missing_evidence: Vec::new(),
            existing_evidence: Vec::new(),
            completion_allowed: false,
            completion_refusal: None,
            recovery_plan: None,
            compaction_plan: None,
            maintenance_plan: None,
            blocked_handoff_plan: None,
            context_package_ids: Vec::new(),
            prompt_card: None,
            persistence_plan: Vec::new(),
            authority_fingerprint: input.authority_fingerprint,
            staleness_fingerprint: input.staleness_fingerprint,
            runtime_effect: None,
        })
    }

    pub fn with_runtime_effect(
        mut self,
        command: RuntimeEffectCommand,
    ) -> Result<Self, DecisionInvariantError> {
        if self.kind != RuntimeDecisionKind::RuntimeEffect {
            return Err(DecisionInvariantError::RuntimeEffectWithoutCommand);
        }
        self.forced_next_action = Some(ActionTemplate::RuntimeEffect(command.clone()));
        self.runtime_effect = Some(command);
        Ok(self)
    }
}
