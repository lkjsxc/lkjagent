use crate::mode::{RuntimeDecision, RuntimeSnapshot};
use crate::task::RuntimeState;

pub(super) fn decision_label(decision: &RuntimeDecision) -> String {
    match decision {
        RuntimeDecision::StartCompaction => "runtime-compaction".to_string(),
        RuntimeDecision::StartRecovery(plan) => format!("recovery:{:?}", plan.recovery_class),
        RuntimeDecision::ContinueRecovery { plan, .. } => {
            format!("recovery:{:?}", plan.recovery_class)
        }
        RuntimeDecision::BlockCompletion(_) => "completion-refused-continue".to_string(),
        RuntimeDecision::CloseCase => "completion-gate-passed".to_string(),
        RuntimeDecision::StartMaintenance => "maintenance-blocked-by-owner-work".to_string(),
        RuntimeDecision::StartVerification => "verification".to_string(),
        RuntimeDecision::ExecuteTool(_) | RuntimeDecision::RefuseAction(_) => {
            "authority-dispatch".to_string()
        }
        RuntimeDecision::AskEndpoint => "continue-owner-execution".to_string(),
    }
}

pub(super) fn checkpoint_notice(
    state: &RuntimeState,
    snapshot: &RuntimeSnapshot,
    decision: &RuntimeDecision,
    decision_label: &str,
) -> String {
    format!(
        "turn budget checkpoint reached; continuing autonomously\ncheckpoint_event=TurnBudgetCheckpoint\nepoch_index={}\nturns_used={}\ncheckpoint_interval={}\nactive_mode={:?}\ncontinuation_decision={decision_label}\nmissing_evidence={}\nexact_next_action={}",
        state.continuation_epoch.epoch_index,
        state.continuation_epoch.turns_used,
        state.continuation_epoch.checkpoint_turns,
        snapshot.active_mode,
        join_or_none(&snapshot.missing_evidence),
        exact_next_action(decision)
    )
}

fn exact_next_action(decision: &RuntimeDecision) -> String {
    match decision {
        RuntimeDecision::StartRecovery(plan) => plan.exact_valid_example.clone(),
        RuntimeDecision::ContinueRecovery { plan, .. } => plan.exact_valid_example.clone(),
        RuntimeDecision::BlockCompletion(admission)
        | RuntimeDecision::ExecuteTool(admission)
        | RuntimeDecision::RefuseAction(admission) => admission
            .exact_valid_example
            .clone()
            .unwrap_or_else(|| "continue with admitted tool".to_string()),
        RuntimeDecision::StartCompaction => "runtime-owned compaction snapshot".to_string(),
        RuntimeDecision::StartVerification => "run admitted verification".to_string(),
        RuntimeDecision::CloseCase => "central completion gate may close".to_string(),
        RuntimeDecision::StartMaintenance => "owner work blocks maintenance".to_string(),
        RuntimeDecision::AskEndpoint => "continue by emitting the next valid action".to_string(),
    }
}

fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().take(8).cloned().collect::<Vec<_>>().join(",")
    }
}
