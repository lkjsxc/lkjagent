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

pub fn tool_biases_from_tracks(vector: &StateVector) -> Vec<String> {
    let mut tools = Vec::new();
    if weight(vector, TrackLabel::ParseRecovery) >= 0.80 {
        tools.extend(names(&[
            "graph.state",
            "doc.audit",
            "fs.list",
            "fs.tree",
            "fs.write",
        ]));
    }
    if weight(vector, TrackLabel::ArtifactDrift) >= 0.75 {
        tools.extend(names(&["artifact.audit", "fs.read", "fs.tree"]));
    }
    if weight(vector, TrackLabel::DocumentStructure) >= 0.60 {
        tools.extend(names(&["doc.audit", "fs.tree", "fs.list"]));
    }
    if weight(vector, TrackLabel::QueueInterruption) >= 0.70 {
        tools.extend(names(&["queue.list", "graph.state"]));
    }
    tools.sort();
    tools.dedup();
    tools
}

pub fn required_context_slices_from_tracks(vector: &StateVector) -> Vec<String> {
    let mut slices = Vec::new();
    if weight(vector, TrackLabel::ParseRecovery) >= 0.80
        || weight(vector, TrackLabel::ActionParamReliability) >= 0.60
    {
        slices.extend(names(&[
            "action grammar",
            "tool schemas",
            "last parser faults",
        ]));
    }
    if weight(vector, TrackLabel::ArtifactDrift) >= 0.75
        || weight(vector, TrackLabel::ArtifactReadiness) >= 0.60
    {
        slices.extend(names(&[
            "owner objective",
            "artifact contract",
            "drifted paths",
        ]));
    }
    if weight(vector, TrackLabel::DocumentStructure) >= 0.60 {
        slices.extend(names(&["doc topology rules", "last doc.audit failures"]));
    }
    if weight(vector, TrackLabel::ContextPressure) >= 0.60 {
        slices.extend(names(&["context budget", "post-compaction checklist"]));
    }
    slices.sort();
    slices.dedup();
    slices
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

pub fn select_recovery(fault: &crate::kernel_events::Fault) -> Vec<String> {
    match fault {
        crate::kernel_events::Fault::ParserFault => {
            names(&["show minimal grammar", "one small action"])
        }
        crate::kernel_events::Fault::ToolParameterFault => {
            names(&["show expected schema", "repair params"])
        }
        crate::kernel_events::Fault::ArtifactDrift => {
            names(&["audit objective", "repair drifted paths"])
        }
        crate::kernel_events::Fault::RepeatedActionRefusal => {
            names(&["choose different tool", "shrink scope"])
        }
        crate::kernel_events::Fault::QueueInterruption => {
            names(&["classify owner task", "preserve active case"])
        }
        _ => names(&["inspect state", "choose smallest safe action"]),
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
    if intent.name == "agent.done" && !check_completion_gates(state).allowed {
        blocked_by.push(TrackLabel::CompletionReadiness);
    }
    if state.repeated_signatures.contains(&intent.signature) {
        blocked_by.push(TrackLabel::RepeatedActionRisk);
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

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
