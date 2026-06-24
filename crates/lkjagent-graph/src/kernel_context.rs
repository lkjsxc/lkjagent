use crate::kernel_context_select::{required_context_slices_from_tracks, tool_biases_from_tracks};
use crate::kernel_types::*;
use crate::kernel_vector::{dominant_tracks, guard_tracks};

#[derive(Debug, Clone, PartialEq)]
pub struct ContextFrame {
    pub case_id: String,
    pub owner_raw_input: String,
    pub normalized_objective: String,
    pub objective_contract: String,
    pub documentation_contract: Option<String>,
    pub artifact_contract: Option<String>,
    pub hard_state: StateNode,
    pub weighted_tracks: Vec<ContextTrack>,
    pub dominant_tracks: Vec<TrackLabel>,
    pub guard_tracks: Vec<TrackLabel>,
    pub allowed_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub evidence_owners: Vec<String>,
    pub forbidden_action_signatures: Vec<String>,
    pub tool_schema_slice: Vec<String>,
    pub selected_context_slices: Vec<String>,
    pub output_grammar: String,
    pub completion_blockers: Vec<String>,
    pub next_action_recommendation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextTrack {
    pub label: TrackLabel,
    pub posture: Posture,
    pub weight: f32,
    pub confidence: f32,
    pub evidence_gap: Option<String>,
    pub guard: Option<GuardPolicy>,
}

pub fn compile_context_frame(state: &CaseState) -> ContextFrame {
    let guards = guard_tracks(&state.state_vector)
        .into_iter()
        .map(|track| track.label)
        .collect::<Vec<_>>();
    let dominant = dominant_tracks(&state.state_vector)
        .into_iter()
        .map(|track| track.label)
        .collect::<Vec<_>>();
    let required = evidence_names(&state.hard_state.completion_gates);
    let missing = missing_evidence(&state.hard_state.completion_gates);
    let completion_blockers = completion_blockers(&missing, &guards);
    ContextFrame {
        case_id: state.case_id.0.clone(),
        owner_raw_input: state.objective.raw.clone(),
        normalized_objective: state.objective.normalized.clone(),
        objective_contract: objective_contract(state),
        documentation_contract: documentation_contract(state),
        artifact_contract: artifact_contract(state),
        hard_state: state.hard_state.node,
        weighted_tracks: context_tracks(&state.state_vector),
        dominant_tracks: dominant,
        guard_tracks: guards.clone(),
        allowed_tools: state.hard_state.allowed_tools.clone(),
        blocked_tools: state.hard_state.blocked_tools.clone(),
        required_evidence: required,
        missing_evidence: missing,
        evidence_owners: evidence_owners(&state.evidence),
        forbidden_action_signatures: state.repeated_signatures.clone(),
        tool_schema_slice: tool_biases_from_tracks(&state.state_vector),
        selected_context_slices: required_context_slices_from_tracks(&state.state_vector),
        output_grammar: output_grammar(&guards),
        completion_blockers,
        next_action_recommendation: next_action(state, &guards),
    }
}

fn context_tracks(vector: &StateVector) -> Vec<ContextTrack> {
    let mut tracks = vector
        .tracks
        .iter()
        .map(|track| ContextTrack {
            label: track.label,
            posture: track.posture,
            weight: track.weight.0,
            confidence: track.confidence.0,
            evidence_gap: track.evidence_gap.clone(),
            guard: track.guard,
        })
        .collect::<Vec<_>>();
    tracks.sort_by(|a, b| b.weight.total_cmp(&a.weight));
    tracks.truncate(5);
    tracks
}

fn objective_contract(state: &CaseState) -> String {
    format!(
        "raw={}\nnormalized={}",
        state.objective.raw, state.objective.normalized
    )
}

fn documentation_contract(state: &CaseState) -> Option<String> {
    if state
        .objective
        .normalized
        .to_ascii_lowercase()
        .contains("doc")
    {
        Some("preserve requested topics and require semantic audits".to_string())
    } else {
        None
    }
}

fn artifact_contract(state: &CaseState) -> Option<String> {
    let text = state.objective.normalized.to_ascii_lowercase();
    if text.contains("cookbook") || text.contains("story") || text.contains("dictionary") {
        Some("match artifact subject and block drift before completion".to_string())
    } else {
        None
    }
}

fn evidence_names(gates: &[CompletionGate]) -> Vec<String> {
    gates.iter().map(|gate| gate.name.clone()).collect()
}

fn missing_evidence(gates: &[CompletionGate]) -> Vec<String> {
    gates
        .iter()
        .filter(|gate| !gate.satisfied)
        .map(|gate| gate.name.clone())
        .collect()
}

fn evidence_owners(ledger: &crate::kernel_events::EvidenceLedger) -> Vec<String> {
    ledger
        .0
        .iter()
        .map(|evidence| format!("{:?}:{:?}", evidence.kind, evidence.owner))
        .collect()
}

fn completion_blockers(missing: &[String], guards: &[TrackLabel]) -> Vec<String> {
    let mut blockers = missing.to_vec();
    blockers.extend(guards.iter().map(|guard| format!("guard:{guard:?}")));
    blockers.sort();
    blockers.dedup();
    blockers
}

fn output_grammar(guards: &[TrackLabel]) -> String {
    if guards.contains(&TrackLabel::ParseRecovery) {
        "one scalar <action> block with line-oriented fields".to_string()
    } else {
        "one <action> block or line-oriented file block".to_string()
    }
}

fn next_action(state: &CaseState, guards: &[TrackLabel]) -> String {
    if guards.contains(&TrackLabel::ParseRecovery) {
        "emit one small valid action".to_string()
    } else if guards.contains(&TrackLabel::ModelSpecificNaming) {
        "run sanitizer or model-name audit repair".to_string()
    } else if guards.contains(&TrackLabel::MockContentRisk) {
        "replace mock content before completion".to_string()
    } else if guards.contains(&TrackLabel::StructureConnectivity) {
        "repair relation graph and backlinks".to_string()
    } else if guards.contains(&TrackLabel::MaintenanceNoopRisk) {
        "record no-op suppression or produce real maintenance output".to_string()
    } else if guards.contains(&TrackLabel::WorkspaceEvidenceRisk) {
        "collect filesystem or workspace evidence before workspace claims".to_string()
    } else {
        tool_biases_from_tracks(&state.state_vector)
            .first()
            .cloned()
            .unwrap_or_else(|| "continue with smallest legal action".to_string())
    }
}
