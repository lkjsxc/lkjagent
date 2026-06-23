#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TurnAuthorityInput {
    pub pending_owner_rows: usize,
    pub active_owner_case: bool,
    pub recoverable_owner_case: bool,
    pub compaction_required: bool,
    pub maintenance_due: bool,
    pub maintenance_active: bool,
    pub endpoint_retry_pending: bool,
}

impl TurnAuthorityInput {
    pub fn owner_work_exists(self) -> bool {
        self.pending_owner_rows > 0 || self.active_owner_case || self.recoverable_owner_case
    }
}
