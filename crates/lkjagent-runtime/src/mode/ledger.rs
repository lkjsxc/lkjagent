use super::ledger_data::{AuthorityFingerprint, DecisionKind, RuntimeDecisionRecord};
use super::ledger_event::{event_kind, fingerprint};
use super::ledger_fields::{blocked_tools, completion_refusal, decision_fields, recovery_route};
use super::mission::{mission_for_snapshot, RuntimeMission};
use super::model::{RuntimeDecision, RuntimeEvent, RuntimeSnapshot};
use super::policy::policy_for_mode;
use super::reducer::decide;

pub fn decide_record(snapshot: &RuntimeSnapshot, event: RuntimeEvent) -> RuntimeDecisionRecord {
    let event_kind = event_kind(&event).to_string();
    let event_debug = format!("{event:?}");
    let decision = decide(snapshot, event);
    let mission = mission_for_decision(snapshot, &decision);
    let active_mode = mission.active_mode();
    let kind = decision_kind(&decision);
    let policy = policy_for_mode(active_mode);
    let fields = decision_fields(snapshot, &decision);
    let state_node = snapshot
        .active_artifact
        .clone()
        .unwrap_or_else(|| "none".to_string());
    let case_id = format!("case:unknown:{state_node}");
    let fp = fingerprint(&[
        &case_id,
        mission.as_str(),
        &state_node,
        &fields.admitted_tools.join(","),
        &snapshot.missing_evidence.join(","),
        &event_debug,
    ]);

    RuntimeDecisionRecord {
        decision_id: format!("decision-{fp}"),
        case_id,
        event_id: format!("event-{fp}"),
        event_kind,
        kind,
        mission,
        active_mode,
        state_node,
        admitted_tools: fields.admitted_tools,
        blocked_tools: blocked_tools(&policy.blocked_tools, &decision),
        forced_next_action: fields.forced_next_action,
        recommended_next_actions: fields.recommended_next_actions,
        exact_valid_example: fields.exact_valid_example,
        missing_evidence: snapshot.missing_evidence.clone(),
        completion_allowed: matches!(decision, RuntimeDecision::CloseCase),
        completion_refusal: completion_refusal(&decision),
        recovery_route: recovery_route(&decision),
        compaction_required: matches!(decision, RuntimeDecision::StartCompaction),
        maintenance_allowed: matches!(decision, RuntimeDecision::StartMaintenance),
        authority_fingerprint: AuthorityFingerprint(fp),
    }
}

fn mission_for_decision(snapshot: &RuntimeSnapshot, decision: &RuntimeDecision) -> RuntimeMission {
    match decision {
        RuntimeDecision::StartCompaction => RuntimeMission::HardRuntimeCompaction,
        RuntimeDecision::StartMaintenance => RuntimeMission::IdleMaintenance,
        RuntimeDecision::StartVerification => RuntimeMission::OwnerVerification,
        RuntimeDecision::CloseCase => RuntimeMission::OwnerCompletion,
        RuntimeDecision::StartRecovery(_) | RuntimeDecision::ContinueRecovery { .. } => {
            RuntimeMission::OwnerRecovery
        }
        _ => mission_for_snapshot(snapshot),
    }
}

fn decision_kind(decision: &RuntimeDecision) -> DecisionKind {
    match decision {
        RuntimeDecision::ExecuteTool(_) => DecisionKind::ExecuteTool,
        RuntimeDecision::AskEndpoint => DecisionKind::AskEndpoint,
        RuntimeDecision::RefuseAction(_) => DecisionKind::RefuseAction,
        RuntimeDecision::StartRecovery(_) => DecisionKind::StartRecovery,
        RuntimeDecision::ContinueRecovery { .. } => DecisionKind::ContinueRecovery,
        RuntimeDecision::StartCompaction => DecisionKind::StartCompaction,
        RuntimeDecision::StartMaintenance => DecisionKind::StartMaintenance,
        RuntimeDecision::StartVerification => DecisionKind::StartVerification,
        RuntimeDecision::CloseCase => DecisionKind::CloseCase,
        RuntimeDecision::BlockCompletion(_) => DecisionKind::BlockCompletion,
    }
}
