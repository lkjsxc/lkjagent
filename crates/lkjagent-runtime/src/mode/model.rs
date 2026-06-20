#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveMode {
    OwnerTask,
    Recovery,
    Maintenance,
    Compaction,
    ClosedIdle,
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
