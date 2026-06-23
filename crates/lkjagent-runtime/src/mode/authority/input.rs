use super::model::ActiveModeInput;

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
    pub fn mode_input(self) -> ActiveModeInput {
        ActiveModeInput {
            pending_owner_rows: self.pending_owner_rows,
            active_owner_case: self.active_owner_case,
            recoverable_owner_case: self.recoverable_owner_case,
            compaction_required: self.compaction_required,
            maintenance_due: self.maintenance_due,
            maintenance_active: self.maintenance_active,
        }
    }

    pub fn owner_work_exists(self) -> bool {
        self.pending_owner_rows > 0 || self.active_owner_case || self.recoverable_owner_case
    }
}
