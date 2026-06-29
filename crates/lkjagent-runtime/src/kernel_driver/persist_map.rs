use lkjagent_store::runtime_authority::{DecisionDetailInput, SnapshotDetailInput};

use crate::kernel::{ActionTemplate, RuntimeDecision, RuntimeSnapshot};

pub fn snapshot_detail(snapshot_id: i64, snapshot: &RuntimeSnapshot) -> SnapshotDetailInput<'_> {
    SnapshotDetailInput {
        snapshot_id,
        graph_phase: snapshot.graph.phase.as_deref().unwrap_or("none"),
        artifact_root: snapshot.artifact.root.as_deref(),
        weak_cursor: None,
        latest_observation: snapshot.observation.latest.as_deref(),
        prompt_frame_head: snapshot.prompt_frame_fingerprint.as_deref(),
        authority_fingerprint: snapshot.authority_fingerprint.as_str(),
    }
}

pub fn decision_detail<'a>(
    decision_id: i64,
    snapshot: &'a RuntimeSnapshot,
    decision: &'a RuntimeDecision,
) -> DecisionDetailInput<'a> {
    DecisionDetailInput {
        decision_id,
        decision_kind: decision_kind(decision),
        graph_phase: decision.graph_phase.as_deref().unwrap_or("none"),
        exact_next_action_class: next_action_class(decision),
        runtime_effect_kind: runtime_effect_kind(decision),
        artifact_root: snapshot.artifact.root.as_deref(),
        weak_cursor: None,
        latest_observation: snapshot.observation.latest.as_deref(),
        prompt_frame_head: snapshot.prompt_frame_fingerprint.as_deref(),
        authority_fingerprint: decision.authority_fingerprint.as_str(),
        staleness_fingerprint: decision.staleness_fingerprint.as_str(),
    }
}

pub fn tool_names(tools: &[crate::kernel::ToolName]) -> Vec<String> {
    tools.iter().map(|tool| tool.as_str().to_string()).collect()
}

pub fn next_action_class(decision: &RuntimeDecision) -> &str {
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { tool, .. }) => tool.as_str(),
        Some(ActionTemplate::RuntimeEffect(_)) => "runtime_effect",
        Some(ActionTemplate::ExternalOwnerWait) => "owner_wait",
        Some(ActionTemplate::ClosedIdle) | None => "none",
    }
}

pub fn runtime_effect_kind(decision: &RuntimeDecision) -> Option<&str> {
    decision.runtime_effect.as_ref().map(|effect| match effect {
        crate::kernel::RuntimeEffectCommand::CompactNow => "hard_compaction",
        crate::kernel::RuntimeEffectCommand::WaitClosedIdle => "closed_idle_wait",
        crate::kernel::RuntimeEffectCommand::DeterministicInspection { .. } => {
            "deterministic_inspection"
        }
        crate::kernel::RuntimeEffectCommand::RecordBlockedHandoff => "blocked_handoff",
        _ => "runtime_effect",
    })
}

pub fn decision_kind(decision: &RuntimeDecision) -> &str {
    match decision.kind {
        crate::kernel::RuntimeDecisionKind::ModelCall => "model_call",
        crate::kernel::RuntimeDecisionKind::RuntimeEffect => "runtime_effect",
        crate::kernel::RuntimeDecisionKind::AdmitDispatch => "admit_dispatch",
        crate::kernel::RuntimeDecisionKind::RefuseAdmission => "refuse_admission",
        crate::kernel::RuntimeDecisionKind::BlockCompletion => "block_completion",
        crate::kernel::RuntimeDecisionKind::CloseCase => "close_case",
        crate::kernel::RuntimeDecisionKind::WaitForOwner => "wait_for_owner",
        crate::kernel::RuntimeDecisionKind::ClosedIdle => "closed_idle",
    }
}

pub fn maintenance_state(snapshot: &RuntimeSnapshot) -> &str {
    if snapshot.maintenance.cooldown_active {
        "cooldown"
    } else if snapshot.maintenance.active {
        "active"
    } else if snapshot.maintenance.due {
        "due"
    } else {
        "inactive"
    }
}

pub fn queue_head_i64(snapshot: &RuntimeSnapshot) -> Option<i64> {
    snapshot.queue.head_id.as_deref()?.parse::<i64>().ok()
}
