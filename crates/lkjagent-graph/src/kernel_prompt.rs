use crate::kernel_context::{compile_context_frame, ContextFrame};
use crate::kernel_types::*;

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
    let context = compile_context_frame(state);
    compile_prompt_frame_from_context(&context)
}

pub fn compile_prompt_frame_from_context(context: &ContextFrame) -> PromptFrame {
    PromptFrame {
        case_id: context.case_id.clone(),
        owner_objective: context.owner_raw_input.clone(),
        normalized_objective: context.normalized_objective.clone(),
        hard_state: context.hard_state,
        active_tracks: context.dominant_tracks.clone(),
        dominant_guards: context.guard_tracks.clone(),
        allowed_tools: context.allowed_tools.clone(),
        blocked_tools: context.blocked_tools.clone(),
        required_evidence: context.required_evidence.clone(),
        missing_evidence: context.missing_evidence.clone(),
        repeated_action_signatures: context.forbidden_action_signatures.clone(),
        context_slices: context.selected_context_slices.clone(),
        output_grammar: context.output_grammar.clone(),
        completion_blockers: context.completion_blockers.clone(),
        next_action_recommendation: context.next_action_recommendation.clone(),
        mode: prompt_mode(context),
    }
}

fn prompt_mode(context: &ContextFrame) -> PromptMode {
    if context.guard_tracks.contains(&TrackLabel::ParseRecovery)
        || context
            .guard_tracks
            .contains(&TrackLabel::RepeatedActionRisk)
    {
        PromptMode::Recovery
    } else {
        match context.hard_state {
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
