#[derive(Debug, Clone, Copy)]
pub struct AuthoritySnapshotInput<'a> {
    pub case_scope: &'a str,
    pub case_id: Option<i64>,
    pub queue_head: Option<i64>,
    pub queue_pending_count: i64,
    pub owner_objective: &'a str,
    pub active_mode: &'a str,
    pub active_node: &'a str,
    pub missing_evidence: &'a [String],
    pub artifact_head: Option<&'a str>,
    pub fault_head: Option<&'a str>,
    pub compaction_head: Option<&'a str>,
    pub maintenance_state: &'a str,
    pub prompt_frame_id: Option<&'a str>,
    pub context_frame_id: Option<&'a str>,
    pub staleness_fingerprint: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct AuthorityEventInput<'a> {
    pub snapshot_id: Option<i64>,
    pub case_scope: &'a str,
    pub case_id: Option<i64>,
    pub event_kind: &'a str,
    pub event_payload: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct AuthorityDecisionInput<'a> {
    pub snapshot_id: Option<i64>,
    pub case_scope: &'a str,
    pub case_id: Option<i64>,
    pub event_id: i64,
    pub mission: &'a str,
    pub active_mode: &'a str,
    pub active_node: &'a str,
    pub admitted_tools: &'a [String],
    pub blocked_tools: &'a [String],
    pub missing_evidence: &'a [String],
    pub forced_next_action: &'a str,
    pub exact_valid_example: Option<&'a str>,
    pub completion_allowed: bool,
    pub completion_refusal: Option<&'a str>,
    pub recovery_route: Option<&'a str>,
    pub compaction_required: bool,
    pub maintenance_allowed: bool,
    pub authority_fingerprint: &'a str,
    pub staleness_fingerprint: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct ToolAdmissionInput<'a> {
    pub decision_id: i64,
    pub case_scope: &'a str,
    pub case_id: Option<i64>,
    pub requested_tool: &'a str,
    pub admitted: bool,
    pub refusal_reason: &'a str,
    pub exact_valid_example: Option<&'a str>,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeTransitionInput<'a> {
    pub snapshot_id: i64,
    pub event_id: i64,
    pub decision_id: i64,
    pub case_scope: &'a str,
    pub case_id: Option<i64>,
    pub from_node: &'a str,
    pub to_node: &'a str,
    pub transition_kind: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeEffectInput<'a> {
    pub decision_id: i64,
    pub admission_id: Option<i64>,
    pub effect_kind: &'a str,
    pub effect_summary: &'a str,
    pub observation_event_id: Option<i64>,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritySnapshotRow {
    pub id: i64,
    pub case_scope: String,
    pub case_id: Option<i64>,
    pub active_mode: String,
    pub active_node: String,
    pub missing_evidence: String,
    pub staleness_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityDecisionRow {
    pub id: i64,
    pub snapshot_id: Option<i64>,
    pub case_scope: String,
    pub case_id: Option<i64>,
    pub event_id: i64,
    pub mission: String,
    pub active_mode: String,
    pub active_node: String,
    pub admitted_tools: String,
    pub blocked_tools: String,
    pub missing_evidence: String,
    pub forced_next_action: String,
    pub exact_valid_example: Option<String>,
    pub completion_allowed: bool,
    pub completion_refusal: Option<String>,
    pub recovery_route: Option<String>,
    pub compaction_required: bool,
    pub maintenance_allowed: bool,
    pub authority_fingerprint: String,
    pub staleness_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAdmissionRow {
    pub id: i64,
    pub decision_id: i64,
    pub requested_tool: String,
    pub admitted: bool,
    pub refusal_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTransitionRow {
    pub id: i64,
    pub decision_id: i64,
    pub from_node: String,
    pub to_node: String,
    pub transition_kind: String,
}
