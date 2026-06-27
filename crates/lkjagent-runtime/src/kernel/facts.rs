#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CaseFacts {
    pub case_id: Option<String>,
    pub owner_objective: Option<String>,
    pub normalized_objective: Option<String>,
    pub task_family: Option<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
    pub risks: Vec<String>,
    pub active_plan_step: Option<String>,
}

impl CaseFacts {
    pub fn owner_work_exists(&self) -> bool {
        self.case_id.is_some() || self.owner_objective.is_some()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GraphFacts {
    pub node: Option<String>,
    pub phase: Option<String>,
    pub legal_transitions: Vec<String>,
    pub ranked_tracks: Vec<String>,
    pub context_package_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QueueFacts {
    pub head_id: Option<String>,
    pub pending_owner_count: usize,
}

impl QueueFacts {
    pub fn has_owner_work(&self) -> bool {
        self.pending_owner_count > 0 || self.head_id.is_some()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvidenceFacts {
    pub required: Vec<String>,
    pub missing: Vec<String>,
    pub existing: Vec<String>,
    pub owners: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtifactFacts {
    pub artifact_id: Option<String>,
    pub root: Option<String>,
    pub kind: Option<String>,
    pub profile: Option<String>,
    pub head_state: Option<String>,
    pub weak_paths: Vec<String>,
    pub cursor: Option<String>,
    pub batch_cursor: Option<String>,
    pub audit_status: Option<String>,
    pub drift_state: Option<String>,
}

impl ArtifactFacts {
    pub fn needs_repair(&self) -> bool {
        !self.weak_paths.is_empty()
            || matches!(self.audit_status.as_deref(), Some("failed" | "missing"))
            || matches!(self.drift_state.as_deref(), Some("drifted"))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObservationFacts {
    pub latest: Option<String>,
    pub latest_successful: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContextFacts {
    pub hard_pressure: bool,
    pub compaction_head: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MaintenanceFacts {
    pub due: bool,
    pub active: bool,
    pub cooldown_active: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderFacts {
    pub latest_exchange_id: Option<String>,
    pub anomaly_class: Option<String>,
    pub retry_count: u32,
    pub pause_deadline: Option<String>,
}
