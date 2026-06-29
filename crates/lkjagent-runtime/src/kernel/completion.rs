use crate::kernel::snapshot::RuntimeSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionGateInput {
    pub objective_present: bool,
    pub missing_evidence: Vec<String>,
    pub latest_fault_present: bool,
    pub weak_paths: Vec<String>,
    pub artifact_required: bool,
    pub artifact_ready: bool,
    pub decision_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionGateDecision {
    pub allowed: bool,
    pub missing_inputs: Vec<String>,
    pub input: CompletionGateInput,
}

pub(crate) fn completion_gate(snapshot: &RuntimeSnapshot) -> CompletionGateDecision {
    let input = CompletionGateInput {
        objective_present: snapshot.case.owner_objective.is_some(),
        missing_evidence: snapshot.evidence.missing.clone(),
        latest_fault_present: snapshot.latest_fault.is_some(),
        weak_paths: snapshot.artifact.weak_paths.clone(),
        artifact_required: artifact_required(snapshot),
        artifact_ready: artifact_ready_if_present(snapshot),
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
