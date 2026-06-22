use crate::kernel_authority::{required_context_slices_from_tracks, tool_biases_from_tracks};
use crate::kernel_types::*;
use crate::kernel_vector::{dominant_tracks, guard_tracks};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptMode {
    Intake,
    SemanticSeed,
    Expansion,
    Relation,
    Audit,
    Repair,
    Recovery,
    Maintenance,
    Verification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptFrame {
    pub case_id: String,
    pub owner_objective: String,
    pub normalized_objective: String,
    pub hard_state: StateNode,
    pub active_tracks: Vec<TrackLabel>,
    pub dominant_guards: Vec<TrackLabel>,
    pub allowed_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub repeated_action_signatures: Vec<String>,
    pub context_slices: Vec<String>,
    pub output_grammar: String,
    pub completion_blockers: Vec<String>,
    pub next_action_recommendation: String,
    pub mode: PromptMode,
}

pub fn compile_prompt_frame(state: &CaseState) -> PromptFrame {
    let dominant_guards = guard_tracks(&state.state_vector)
        .into_iter()
        .map(|track| track.label)
        .collect::<Vec<_>>();
    let active_tracks = dominant_tracks(&state.state_vector)
        .into_iter()
        .map(|track| track.label)
        .collect::<Vec<_>>();
    PromptFrame {
        case_id: state.case_id.0.clone(),
        owner_objective: state.objective.raw.clone(),
        normalized_objective: state.objective.normalized.clone(),
        hard_state: state.hard_state.node,
        active_tracks,
        dominant_guards: dominant_guards.clone(),
        allowed_tools: state.hard_state.allowed_tools.clone(),
        blocked_tools: state.hard_state.blocked_tools.clone(),
        required_evidence: evidence_names(&state.hard_state.completion_gates),
        missing_evidence: missing_evidence(&state.hard_state.completion_gates),
        repeated_action_signatures: state.repeated_signatures.clone(),
        context_slices: required_context_slices_from_tracks(&state.state_vector),
        output_grammar: "one <act> block or line-oriented file block".to_string(),
        completion_blockers: completion_blockers(state, &dominant_guards),
        next_action_recommendation: next_action(state, &dominant_guards),
        mode: prompt_mode(state, &dominant_guards),
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

fn completion_blockers(state: &CaseState, guards: &[TrackLabel]) -> Vec<String> {
    let mut blockers = missing_evidence(&state.hard_state.completion_gates);
    blockers.extend(guards.iter().map(|guard| format!("guard:{guard:?}")));
    blockers.sort();
    blockers.dedup();
    blockers
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
    } else {
        tool_biases_from_tracks(&state.state_vector)
            .first()
            .cloned()
            .unwrap_or_else(|| "continue with smallest legal action".to_string())
    }
}

fn prompt_mode(state: &CaseState, guards: &[TrackLabel]) -> PromptMode {
    if guards.contains(&TrackLabel::ParseRecovery)
        || guards.contains(&TrackLabel::RepeatedActionRisk)
    {
        PromptMode::Recovery
    } else {
        match state.hard_state.node {
            StateNode::Intake => PromptMode::Intake,
            StateNode::Planning => PromptMode::SemanticSeed,
            StateNode::Executing => PromptMode::Expansion,
            StateNode::Auditing => PromptMode::Audit,
            StateNode::Recovering => PromptMode::Recovery,
            StateNode::Compacting => PromptMode::Repair,
            StateNode::CompletionBlocked => PromptMode::Repair,
            StateNode::Done => PromptMode::Verification,
        }
    }
}
