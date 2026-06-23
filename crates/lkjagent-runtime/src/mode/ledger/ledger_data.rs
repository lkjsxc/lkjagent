use super::mission::RuntimeMission;
use super::model::ActiveMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionKind {
    ExecuteTool,
    AskEndpoint,
    RefuseAction,
    StartRecovery,
    ContinueRecovery,
    StartCompaction,
    StartMaintenance,
    StartVerification,
    CloseCase,
    BlockCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityFingerprint(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDecisionRecord {
    pub decision_id: String,
    pub case_id: String,
    pub event_id: String,
    pub event_kind: String,
    pub kind: DecisionKind,
    pub mission: RuntimeMission,
    pub active_mode: ActiveMode,
    pub state_node: String,
    pub admitted_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub forced_next_action: String,
    pub recommended_next_actions: Vec<String>,
    pub exact_valid_example: Option<String>,
    pub missing_evidence: Vec<String>,
    pub completion_allowed: bool,
    pub completion_refusal: Option<String>,
    pub recovery_route: Option<String>,
    pub compaction_required: bool,
    pub maintenance_allowed: bool,
    pub authority_fingerprint: AuthorityFingerprint,
}
