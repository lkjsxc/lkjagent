use crate::kernel_context_select::tool_biases_from_tracks;
use crate::kernel_track_ops::weight;
use crate::kernel_types::*;
use crate::kernel_vector::guard_tracks;

const COMPLETION_THRESHOLD: f32 = 0.80;

pub fn authorize_tool_intent(state: &CaseState, intent: &ToolIntent) -> ToolAuthorization {
    let mut blocked_by = Vec::new();
    if !hard_allows(&state.hard_state, intent) {
        return refusal("hard state does not allow tool", blocked_by, state);
    }
    apply_weighted_guards(state, intent, &mut blocked_by);
    if blocked_by.is_empty() {
        ToolAuthorization {
            allowed: true,
            reason: "tool admitted by hard state and weighted guards".to_string(),
            blocked_by,
            preferred_tools: tool_biases_from_tracks(&state.state_vector),
        }
    } else {
        refusal("weighted guard blocks tool", blocked_by, state)
    }
}

pub fn check_completion_gates(state: &CaseState) -> ToolAuthorization {
    let gates_ok = state
        .hard_state
        .completion_gates
        .iter()
        .all(|gate| gate.satisfied);
    let ready =
        weight(&state.state_vector, TrackLabel::CompletionReadiness) >= COMPLETION_THRESHOLD;
    let guards = guard_tracks(&state.state_vector);
    if gates_ok && ready && guards.is_empty() {
        ToolAuthorization {
            allowed: true,
            reason: "completion gates satisfied".to_string(),
            blocked_by: Vec::new(),
            preferred_tools: Vec::new(),
        }
    } else {
        ToolAuthorization {
            allowed: false,
            reason: "completion gates missing or guard tracks active".to_string(),
            blocked_by: guards.into_iter().map(|track| track.label).collect(),
            preferred_tools: tool_biases_from_tracks(&state.state_vector),
        }
    }
}

fn apply_weighted_guards(state: &CaseState, intent: &ToolIntent, blocked_by: &mut Vec<TrackLabel>) {
    let vector = &state.state_vector;
    if weight(vector, TrackLabel::ParseRecovery) >= 0.80 && is_large_payload(intent) {
        blocked_by.push(TrackLabel::ParseRecovery);
    }
    if weight(vector, TrackLabel::ArtifactDrift) >= 0.75
        && matches!(intent.name.as_str(), "artifact.next" | "artifact.apply")
    {
        blocked_by.push(TrackLabel::ArtifactDrift);
    }
    if weight(vector, TrackLabel::ContextPressure) >= 0.85 && is_mutating(intent) {
        blocked_by.push(TrackLabel::ContextPressure);
    }
    if weight(vector, TrackLabel::QueueInterruption) >= 0.70 && is_mutating(intent) {
        blocked_by.push(TrackLabel::QueueInterruption);
    }
    block_completion_guards(vector, intent, blocked_by);
    if weight(vector, TrackLabel::ModelSpecificNaming) >= 0.60
        && matches!(intent.name.as_str(), "memory.save" | "agent.done")
    {
        blocked_by.push(TrackLabel::ModelSpecificNaming);
    }
    if intent.name == "agent.done" && !check_completion_gates(state).allowed {
        blocked_by.push(TrackLabel::CompletionReadiness);
    }
    if state.repeated_signatures.contains(&intent.signature) {
        blocked_by.push(TrackLabel::RepeatedActionRisk);
    }
}

fn block_completion_guards(
    vector: &StateVector,
    intent: &ToolIntent,
    blocked_by: &mut Vec<TrackLabel>,
) {
    if intent.name != "agent.done" {
        return;
    }
    push_if(vector, TrackLabel::MockContentRisk, 0.70, blocked_by);
    push_if(vector, TrackLabel::StructureConnectivity, 0.60, blocked_by);
    push_if(vector, TrackLabel::MaintenanceNoopRisk, 0.60, blocked_by);
    push_if(vector, TrackLabel::WorkspaceEvidenceRisk, 0.60, blocked_by);
}

fn push_if(
    vector: &StateVector,
    label: TrackLabel,
    threshold: f32,
    blocked_by: &mut Vec<TrackLabel>,
) {
    if weight(vector, label) >= threshold {
        blocked_by.push(label);
    }
}

fn hard_allows(hard: &HardState, intent: &ToolIntent) -> bool {
    !hard.blocked_tools.contains(&intent.name)
        && (hard.allowed_tools.is_empty() || hard.allowed_tools.contains(&intent.name))
}

fn is_large_payload(intent: &ToolIntent) -> bool {
    matches!(intent.name.as_str(), "fs.batch_write" | "artifact.apply") && intent.payload_size > 1
}

fn is_mutating(intent: &ToolIntent) -> bool {
    matches!(
        intent.name.as_str(),
        "fs.write" | "fs.batch_write" | "fs.edit" | "artifact.apply"
    )
}

fn refusal(reason: &str, blocked_by: Vec<TrackLabel>, state: &CaseState) -> ToolAuthorization {
    ToolAuthorization {
        allowed: false,
        reason: reason.to_string(),
        blocked_by,
        preferred_tools: tool_biases_from_tracks(&state.state_vector),
    }
}
