use crate::kernel::manuscript::facts_from_snapshot;
use crate::kernel::snapshot::RuntimeSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionGateInput {
    pub objective_present: bool,
    pub missing_evidence: Vec<String>,
    pub latest_fault_present: bool,
    pub weak_paths: Vec<String>,
    pub artifact_required: bool,
    pub artifact_ready: bool,
    pub manuscript_active: bool,
    pub manuscript_words_written: usize,
    pub manuscript_word_floor: usize,
    pub manuscript_target_words: Option<usize>,
    pub manuscript_chapter_count: Option<usize>,
    pub missing_manuscript_paths: Vec<String>,
    pub next_manuscript_path: Option<String>,
    pub decision_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionGateDecision {
    pub allowed: bool,
    pub missing_inputs: Vec<String>,
    pub input: CompletionGateInput,
}

pub(crate) fn completion_gate(snapshot: &RuntimeSnapshot) -> CompletionGateDecision {
    let manuscript = facts_from_snapshot(snapshot);
    let input = CompletionGateInput {
        objective_present: snapshot.case.owner_objective.is_some(),
        missing_evidence: snapshot.evidence.missing.clone(),
        latest_fault_present: snapshot.latest_fault.is_some(),
        weak_paths: snapshot.artifact.weak_paths.clone(),
        artifact_required: artifact_required(snapshot),
        artifact_ready: artifact_ready_if_present(snapshot),
        manuscript_active: manuscript.as_ref().is_some_and(|facts| facts.active),
        manuscript_words_written: manuscript
            .as_ref()
            .map(|facts| facts.words_written)
            .unwrap_or(0),
        manuscript_word_floor: manuscript
            .as_ref()
            .map(|facts| facts.target_word_floor)
            .unwrap_or(0),
        manuscript_target_words: manuscript.as_ref().and_then(|facts| facts.target_words),
        manuscript_chapter_count: manuscript.as_ref().and_then(|facts| facts.chapter_count),
        missing_manuscript_paths: manuscript
            .as_ref()
            .map(|facts| facts.missing_paths.clone())
            .unwrap_or_default(),
        next_manuscript_path: manuscript.and_then(|facts| facts.next_path),
        decision_fingerprint: snapshot.staleness_fingerprint.as_str().to_string(),
    };
    let missing_inputs = missing_inputs(&input);
    CompletionGateDecision {
        allowed: missing_inputs.is_empty(),
        missing_inputs,
        input,
    }
}

fn missing_inputs(input: &CompletionGateInput) -> Vec<String> {
    let mut missing = Vec::new();
    if !input.objective_present {
        missing.push("objective".to_string());
    }
    missing.extend(input.missing_evidence.iter().cloned());
    if input.latest_fault_present {
        missing.push("recovery-fault".to_string());
    }
    if !input.weak_paths.is_empty() {
        missing.push("artifact-weak-paths".to_string());
    }
    if input.artifact_required && !input.artifact_ready {
        missing.push("artifact-readiness".to_string());
    }
    if input.manuscript_active && !input.missing_manuscript_paths.is_empty() {
        missing.push(format!(
            "manuscript-paths:{}",
            input.next_manuscript_path.as_deref().unwrap_or("unknown")
        ));
    }
    if input.manuscript_active && input.manuscript_words_written < input.manuscript_word_floor {
        missing.push(format!(
            "manuscript-word-count:{}/{}",
            input.manuscript_words_written, input.manuscript_word_floor
        ));
    }
    dedup(missing)
}

fn artifact_required(snapshot: &RuntimeSnapshot) -> bool {
    snapshot
        .evidence
        .required
        .iter()
        .any(|evidence| evidence == "artifact-readiness")
}

fn artifact_ready_if_present(snapshot: &RuntimeSnapshot) -> bool {
    !artifact_required(snapshot)
        || snapshot
            .evidence
            .existing
            .iter()
            .any(|evidence| evidence == "artifact-readiness")
}

fn dedup(values: Vec<String>) -> Vec<String> {
    values.into_iter().fold(Vec::new(), |mut unique, value| {
        if !unique.iter().any(|existing| existing == &value) {
            unique.push(value);
        }
        unique
    })
}
