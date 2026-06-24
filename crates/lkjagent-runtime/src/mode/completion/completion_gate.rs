use super::model::{ActiveMode, RuntimeSnapshot};
use lkjagent_tools::dispatch::registry_valid_example;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    OwnerTask,
    Maintenance,
    BlockedHandoff,
    RuntimeOnly,
    ClosedIdle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionDecision {
    pub allowed: bool,
    pub completion_kind: CompletionKind,
    pub failed_gates: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub existing_evidence: Vec<String>,
    pub next_executable_action: String,
    pub valid_example: String,
    pub blocked_handoff_allowed: bool,
    pub current_artifact: Option<String>,
    pub status_text: String,
}

pub fn decide_completion(snapshot: &RuntimeSnapshot) -> CompletionDecision {
    let failed_gates = failed_gates(snapshot);
    let allowed = failed_gates.is_empty();
    let completion_kind = completion_kind(snapshot);
    let next_executable_action = next_action(snapshot, allowed);
    let valid_example = registry_valid_example(&next_executable_action)
        .unwrap_or_else(|| format!("runtime action: {next_executable_action}"));
    CompletionDecision {
        allowed,
        completion_kind,
        failed_gates: failed_gates.clone(),
        missing_evidence: snapshot.missing_evidence.clone(),
        existing_evidence: existing_evidence(snapshot),
        next_executable_action,
        valid_example,
        blocked_handoff_allowed: completion_kind == CompletionKind::BlockedHandoff,
        current_artifact: snapshot.active_artifact.clone(),
        status_text: status_text(snapshot, allowed, &failed_gates),
    }
}

fn failed_gates(snapshot: &RuntimeSnapshot) -> Vec<String> {
    let mut gates = Vec::new();
    match snapshot.active_mode {
        ActiveMode::OwnerTask => owner_gates(snapshot, &mut gates),
        ActiveMode::Maintenance => maintenance_gates(snapshot, &mut gates),
        ActiveMode::Recovery => gates.push("recovery-resolution".to_string()),
        ActiveMode::Compaction => gates.push("runtime-compaction".to_string()),
        ActiveMode::ClosedIdle => gates.push("closed-idle".to_string()),
    }
    gates
}

fn owner_gates(snapshot: &RuntimeSnapshot, gates: &mut Vec<String>) {
    if !snapshot.owner_work_exists {
        gates.push("owner-work".to_string());
    }
    if snapshot.recovery_ladder_active {
        gates.push("recovery-resolution".to_string());
    }
    if snapshot.context_pressure_active {
        gates.push("runtime-compaction".to_string());
    }
    gates.extend(snapshot.missing_evidence.iter().cloned());
}

fn maintenance_gates(snapshot: &RuntimeSnapshot, gates: &mut Vec<String>) {
    if snapshot.owner_work_exists {
        gates.push("owner-work-preempts-maintenance".to_string());
    }
    if snapshot.recovery_ladder_active {
        gates.push("recovery-preempts-maintenance".to_string());
    }
    if snapshot.context_pressure_active {
        gates.push("runtime-compaction".to_string());
    }
}

fn completion_kind(snapshot: &RuntimeSnapshot) -> CompletionKind {
    match snapshot.active_mode {
        ActiveMode::OwnerTask => CompletionKind::OwnerTask,
        ActiveMode::Maintenance => CompletionKind::Maintenance,
        ActiveMode::Recovery => CompletionKind::BlockedHandoff,
        ActiveMode::Compaction => CompletionKind::RuntimeOnly,
        ActiveMode::ClosedIdle => CompletionKind::ClosedIdle,
    }
}

fn next_action(snapshot: &RuntimeSnapshot, allowed: bool) -> String {
    if allowed {
        return "agent.done".to_string();
    }
    if snapshot.context_pressure_active {
        return "runtime.compact".to_string();
    }
    if snapshot
        .missing_evidence
        .iter()
        .any(|item| item == "verification")
    {
        return "verify.xtask".to_string();
    }
    if snapshot
        .missing_evidence
        .iter()
        .any(|item| item == "artifact-readiness")
    {
        return "artifact.audit".to_string();
    }
    if snapshot.missing_evidence.iter().any(|item| item == "plan") {
        return "graph.plan".to_string();
    }
    "fs.read".to_string()
}

fn existing_evidence(snapshot: &RuntimeSnapshot) -> Vec<String> {
    snapshot
        .required_evidence
        .iter()
        .filter(|item| {
            !snapshot
                .missing_evidence
                .iter()
                .any(|missing| missing == *item)
        })
        .cloned()
        .collect()
}

fn status_text(snapshot: &RuntimeSnapshot, allowed: bool, failed_gates: &[String]) -> String {
    if allowed {
        return "completion admitted".to_string();
    }
    let artifact = snapshot.active_artifact.as_deref().unwrap_or("none");
    format!(
        "completion refused: {}; artifact={artifact}; next={}",
        failed_gates.join(","),
        next_action(snapshot, false)
    )
}
