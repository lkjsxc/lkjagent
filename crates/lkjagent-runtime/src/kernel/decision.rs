use crate::kernel::active_mode::ActiveMode;
use crate::kernel::admission::ToolAdmissionView;
use crate::kernel::effect::RuntimeEffectCommand;
use crate::kernel::fault::FaultClass;
pub use crate::kernel::mission::RuntimeMission;
use crate::kernel::render::PromptCardData;
use crate::kernel::snapshot::{
    AuthorityFingerprint, RuntimeEventId, RuntimeSnapshotId, StalenessFingerprint, ToolName,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeDecisionId {
    Pending,
    Stored(u64),
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
pub struct ContentWriteContract {
    pub root: String,
    pub paths: Vec<String>,
    pub max_files: usize,
    pub max_file_bytes: usize,
    pub max_batch_bytes: usize,
    pub required_sections: Vec<String>,
    pub forbidden_weak_phrase_classes: Vec<String>,
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
    pub case_id: Option<String>,
    pub mission: RuntimeMission,
    pub active_mode: ActiveMode,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub kind: RuntimeDecisionKind,
    pub admission_view: ToolAdmissionView,
    pub forced_next_action: Option<ActionTemplate>,
    pub content_write_contract: Option<ContentWriteContract>,
    pub recommended_next_actions: Vec<ActionTemplate>,
    pub missing_evidence: Vec<String>,
    pub existing_evidence: Vec<String>,
    pub artifact_id: Option<String>,
    pub root: Option<String>,
    pub artifact_kind: Option<String>,
    pub artifact_profile: Option<String>,
    pub weak_paths: Vec<String>,
    pub cursor: Option<String>,
    pub fault_class: Option<FaultClass>,
    pub retry_count: u32,
    pub provider_anomaly_budget: u32,
    pub compaction_policy: Option<String>,
    pub completion_allowed: bool,
    pub completion_blockers: Vec<String>,
    pub completion_refusal: Option<String>,
    pub completion_gate_inputs: Vec<String>,
    pub resolver_plan: Option<String>,
    pub progress_key: Option<String>,
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
    pub rule_explanation: String,
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
            case_id: None,
            mission: input.mission,
            active_mode: input.mission.active_mode(),
            graph_node: None,
            graph_phase: None,
            kind: input.kind,
            admission_view: input.admission_view,
            forced_next_action: None,
            content_write_contract: None,
            recommended_next_actions: Vec::new(),
            missing_evidence: Vec::new(),
            existing_evidence: Vec::new(),
            artifact_id: None,
            root: None,
            artifact_kind: None,
            artifact_profile: None,
            weak_paths: Vec::new(),
            cursor: None,
            fault_class: None,
            retry_count: 0,
            provider_anomaly_budget: 0,
            compaction_policy: None,
            completion_allowed: false,
            completion_blockers: Vec::new(),
            completion_refusal: None,
            completion_gate_inputs: Vec::new(),
            resolver_plan: None,
            progress_key: None,
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
            rule_explanation: input.mission.as_str().to_string(),
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
