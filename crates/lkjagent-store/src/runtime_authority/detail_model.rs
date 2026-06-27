#[derive(Debug, Clone, Copy)]
pub struct SnapshotDetailInput<'a> {
    pub snapshot_id: i64,
    pub graph_phase: &'a str,
    pub artifact_root: Option<&'a str>,
    pub weak_cursor: Option<i64>,
    pub latest_observation: Option<&'a str>,
    pub prompt_frame_head: Option<&'a str>,
    pub authority_fingerprint: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct DecisionDetailInput<'a> {
    pub decision_id: i64,
    pub decision_kind: &'a str,
    pub graph_phase: &'a str,
    pub exact_next_action_class: &'a str,
    pub runtime_effect_kind: Option<&'a str>,
    pub artifact_root: Option<&'a str>,
    pub weak_cursor: Option<i64>,
    pub latest_observation: Option<&'a str>,
    pub prompt_frame_head: Option<&'a str>,
    pub authority_fingerprint: &'a str,
    pub staleness_fingerprint: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotDetailRow {
    pub snapshot_id: i64,
    pub graph_phase: String,
    pub artifact_root: Option<String>,
    pub weak_cursor: Option<i64>,
    pub latest_observation: Option<String>,
    pub prompt_frame_head: Option<String>,
    pub authority_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionDetailRow {
    pub decision_id: i64,
    pub decision_kind: String,
    pub graph_phase: String,
    pub exact_next_action_class: String,
    pub runtime_effect_kind: Option<String>,
    pub artifact_root: Option<String>,
    pub weak_cursor: Option<i64>,
    pub latest_observation: Option<String>,
    pub prompt_frame_head: Option<String>,
    pub authority_fingerprint: String,
    pub staleness_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityChainRow {
    pub snapshot_id: Option<i64>,
    pub event_id: i64,
    pub decision_id: i64,
    pub prompt_frame_id: Option<i64>,
    pub admission_id: Option<i64>,
    pub observation_id: Option<i64>,
}
