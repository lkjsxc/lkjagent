use super::model::{ActiveMode, ActiveModeInput};

pub fn select_active_mode(input: ActiveModeInput) -> ActiveMode {
    if input.compaction_required {
        return ActiveMode::Compaction;
    }
    if input.pending_owner_rows > 0 {
        return ActiveMode::OwnerTask;
    }
    if input.recoverable_owner_case {
        return ActiveMode::Recovery;
    }
    if input.active_owner_case {
        return ActiveMode::OwnerTask;
    }
    if input.maintenance_active || input.maintenance_due {
        return ActiveMode::Maintenance;
    }
    ActiveMode::ClosedIdle
}
