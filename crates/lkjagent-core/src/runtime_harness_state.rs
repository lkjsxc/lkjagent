use serde::{Deserialize, Serialize};

use crate::runtime_decision::OutputEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeHarnessState {
    Intake,
    Clarify,
    Plan,
    Act,
    Observe,
    Recover,
    Record,
    Maintain,
    Idle,
}

impl RuntimeHarnessState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Intake => "intake",
            Self::Clarify => "clarify",
            Self::Plan => "plan",
            Self::Act => "act",
            Self::Observe => "observe",
            Self::Recover => "recover",
            Self::Record => "record",
            Self::Maintain => "maintain",
            Self::Idle => "idle",
        }
    }

    pub fn purpose(self) -> &'static str {
        match self {
            Self::Intake => "classify owner turn and write transcript or inbox evidence",
            Self::Clarify => "ask or answer one bounded missing-information question",
            Self::Plan => "produce a bounded plan or content shape before effects",
            Self::Act => "execute selected model action, content write, or native effect",
            Self::Observe => "run checks and evaluate completion evidence",
            Self::Recover => "repair parse, admission, endpoint, effect, or check failure",
            Self::Record => "write owner-readable personal or work records",
            Self::Maintain => "rebuild indexes, rebalance paths, or collect proof",
            Self::Idle => "wait only when no executable unresolved work exists",
        }
    }

    pub fn context_policy(self) -> &'static str {
        match self {
            Self::Intake | Self::Record => "recent owner turn plus workspace maps",
            Self::Plan | Self::Act => "canonical docs plus selected workspace evidence",
            Self::Observe => "checks, artifacts, fingerprints, and proof refs",
            Self::Recover => "bounded fault diagnosis without raw failed output",
            Self::Clarify => "missing fact and prior question only",
            Self::Maintain => "workspace indexes, manifests, aliases, and proof refs",
            Self::Idle => "no model context unless new work arrives",
        }
    }

    pub fn workspace_policy(self) -> &'static str {
        match self {
            Self::Intake => "write transcript or inbox trace",
            Self::Record => "write record, history, fingerprint, README, and index evidence",
            Self::Act | Self::Maintain => "path-checked workspace effects only",
            _ => "read bounded selected workspace refs only",
        }
    }

    pub fn failure_policy(self) -> &'static str {
        match self {
            Self::Recover => "narrow tools and retry the smallest valid envelope",
            Self::Idle => "stay idle only with blocker, closure, or no-work evidence",
            _ => "write recovery.failure before any happy response",
        }
    }

    pub fn prompt_fragment(self) -> String {
        format!(
            "Harness state: {}\nState purpose: {}\nContext policy: {}\nWorkspace policy: {}\nFailure policy: {}",
            self.as_str(),
            self.purpose(),
            self.context_policy(),
            self.workspace_policy(),
            self.failure_policy()
        )
    }
}

pub fn derive_harness_state(
    selected_state_key: Option<&str>,
    operation: &str,
    envelope: OutputEnvelope,
    recovery_policy: &str,
) -> RuntimeHarnessState {
    let namespace =
        selected_state_key.and_then(|label| label.split_once(':').map(|(left, _)| left));
    if namespace == Some("recovery") || operation.starts_with("recovery.") {
        return RuntimeHarnessState::Recover;
    }
    if matches!(selected_state_key, Some("case:owner-intake")) || operation == "owner.intake" {
        return RuntimeHarnessState::Intake;
    }
    if matches!(selected_state_key, Some("case:waiting-answer")) || operation == "owner.answer" {
        return RuntimeHarnessState::Clarify;
    }
    if is_record_namespace(namespace) || is_record_operation(operation) {
        return RuntimeHarnessState::Record;
    }
    if is_maintain_namespace(namespace) || is_maintain_operation(operation) {
        return RuntimeHarnessState::Maintain;
    }
    if operation.starts_with("check.run/") || operation.starts_with("completion.") {
        return RuntimeHarnessState::Observe;
    }
    if operation == "runtime.idle" || envelope == OutputEnvelope::None && recovery_policy == "none"
    {
        return RuntimeHarnessState::Idle;
    }
    match envelope {
        OutputEnvelope::Plan => RuntimeHarnessState::Plan,
        OutputEnvelope::Message => RuntimeHarnessState::Clarify,
        OutputEnvelope::Action | OutputEnvelope::Content | OutputEnvelope::Verdict => {
            RuntimeHarnessState::Act
        }
        OutputEnvelope::None => RuntimeHarnessState::Act,
    }
}

fn is_record_namespace(namespace: Option<&str>) -> bool {
    matches!(
        namespace,
        Some(
            "journal"
                | "todo"
                | "calendar"
                | "finance"
                | "note"
                | "contact"
                | "reference"
                | "routine"
                | "dev"
                | "project"
        )
    )
}

fn is_record_operation(operation: &str) -> bool {
    matches!(
        operation
            .split_once('/')
            .map_or(operation, |(head, _)| head),
        "journal.record"
            | "todo.review"
            | "calendar.review"
            | "finance.review"
            | "note.record"
            | "contact.record"
            | "reference.record"
            | "routine.run"
            | "dev.review"
            | "project.advance"
    )
}

fn is_maintain_namespace(namespace: Option<&str>) -> bool {
    matches!(
        namespace,
        Some("index" | "proof" | "maintenance" | "workspace")
    )
}

fn is_maintain_operation(operation: &str) -> bool {
    let head = operation
        .split_once('/')
        .map_or(operation, |(head, _)| head);
    matches!(
        head,
        "index.rebuild" | "proof.collect" | "workspace.rebalance" | "workspace.maintain"
    )
}
