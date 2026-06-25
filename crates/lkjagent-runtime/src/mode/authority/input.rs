#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TurnAuthorityInput {
    pub pending_owner_rows: usize,
    pub active_owner_case: bool,
    pub recoverable_owner_case: bool,
    pub compaction_required: bool,
    pub maintenance_due: bool,
    pub maintenance_active: bool,
    pub endpoint_retry_pending: bool,
    pub case_id: Option<i64>,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub artifact_root: Option<String>,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub latest_decision_id: Option<String>,
    pub prompt_frame_id: Option<String>,
    pub staleness_fingerprint: Option<String>,
}

impl TurnAuthorityInput {
    pub fn owner_work_exists(&self) -> bool {
        self.pending_owner_rows > 0 || self.active_owner_case || self.recoverable_owner_case
    }
}
