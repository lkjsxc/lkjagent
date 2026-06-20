use super::model::ActiveMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointDecision {
    CallModel,
    RuntimeCompact,
    DeliverOwner,
    DeferMaintenance,
    ClosedIdle,
    WaitForRetry,
}

pub fn endpoint_decision_for(
    mode: ActiveMode,
    input: super::input::TurnAuthorityInput,
) -> EndpointDecision {
    if input.maintenance_active && input.owner_work_exists() {
        return EndpointDecision::DeferMaintenance;
    }
    if input.pending_owner_rows > 0 {
        return EndpointDecision::DeliverOwner;
    }
    if input.compaction_required {
        return EndpointDecision::RuntimeCompact;
    }
    if mode == ActiveMode::Compaction {
        return EndpointDecision::RuntimeCompact;
    }
    if input.endpoint_retry_pending {
        return EndpointDecision::WaitForRetry;
    }
    if mode == ActiveMode::ClosedIdle {
        return EndpointDecision::ClosedIdle;
    }
    EndpointDecision::CallModel
}
