use crate::model::{GraphDefinition, GraphNodeId};
use crate::state::TaskGraphState;
use crate::transition::{transition_quality, TransitionLegality, TransitionQuality};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionIntent {
    Continue,
    AfterObservation,
    AfterPlan,
    AfterVerification,
    AfterParseFault,
    AfterParamFault,
    AfterToolFault,
    AfterRepeatFault,
    UnderContextPressure,
    AttemptCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionSelection {
    pub target: Option<GraphNodeId>,
    pub legality: TransitionLegality,
    pub reason: String,
    pub missing: Vec<String>,
    pub quality: Option<TransitionQuality>,
    pub forced_action_class: Option<String>,
}

pub fn best_next_transition(
    graph: &GraphDefinition,
    state: &TaskGraphState,
    intent: TransitionIntent,
) -> TransitionSelection {
    let mut best: Option<(i32, usize, TransitionSelection)> = None;
    for (index, edge) in graph
        .edges
        .iter()
        .filter(|edge| edge.from == state.active_node)
        .enumerate()
    {
        let quality = transition_quality(graph, state, edge.to);
        let score = selection_score(intent, edge.to, &quality);
        let selection = selection(edge.to, quality, intent);
        if best
            .as_ref()
            .is_none_or(|(best_score, _, _)| score > *best_score)
        {
            best = Some((score, index, selection));
        }
    }
    best.map(|(_, _, selection)| selection)
        .unwrap_or_else(|| no_transition(state))
}

fn selection(
    target: GraphNodeId,
    quality: TransitionQuality,
    intent: TransitionIntent,
) -> TransitionSelection {
    TransitionSelection {
        target: Some(target),
        legality: quality.legality,
        reason: quality.reason.clone(),
        missing: missing(&quality),
        quality: Some(quality),
        forced_action_class: forced_action_class(intent),
    }
}

fn no_transition(state: &TaskGraphState) -> TransitionSelection {
    TransitionSelection {
        target: None,
        legality: TransitionLegality::Illegal,
        reason: format!("no outgoing transitions from {}", state.active_node.0),
        missing: Vec::new(),
        quality: None,
        forced_action_class: None,
    }
}

fn selection_score(
    intent: TransitionIntent,
    target: GraphNodeId,
    quality: &TransitionQuality,
) -> i32 {
    legality_score(quality.legality)
        + i32::from(quality.evidence_delta) * 4
        + i32::from(quality.context_delta) * 2
        - i32::from(quality.risk_delta)
        - i32::from(quality.repetition_penalty) * 2
        + intent_bonus(intent, target, quality.legality)
}

fn legality_score(legality: TransitionLegality) -> i32 {
    match legality {
        TransitionLegality::Legal => 1_000,
        TransitionLegality::Blocked => 0,
        TransitionLegality::Illegal => -1_000,
    }
}

fn intent_bonus(
    intent: TransitionIntent,
    target: GraphNodeId,
    legality: TransitionLegality,
) -> i32 {
    match intent {
        TransitionIntent::AfterParseFault if target == GraphNodeId("recover-parse") => 500,
        TransitionIntent::AfterParamFault if target == GraphNodeId("recover-params") => 500,
        TransitionIntent::AfterToolFault if target == GraphNodeId("recover-tool") => 500,
        TransitionIntent::AfterRepeatFault if target == GraphNodeId("recover-repeat") => 500,
        TransitionIntent::AfterObservation if target == GraphNodeId("verify") => 140,
        TransitionIntent::AfterPlan if target == GraphNodeId("execute") => 140,
        TransitionIntent::AfterPlan if target == GraphNodeId("review-plan") => 120,
        TransitionIntent::AfterVerification if target == GraphNodeId("complete") => 180,
        TransitionIntent::AttemptCompletion if completion_target(target) => 400,
        TransitionIntent::UnderContextPressure
            if compaction_target(target) && legality == TransitionLegality::Legal =>
        {
            300
        }
        TransitionIntent::UnderContextPressure if compaction_target(target) => -300,
        _ => 0,
    }
}

fn completion_target(target: GraphNodeId) -> bool {
    matches!(
        target.0,
        "completion-audit" | "completion-evidence" | "completion-memory" | "complete"
    )
}

fn compaction_target(target: GraphNodeId) -> bool {
    matches!(
        target.0,
        "compact-soft" | "compact-hard" | "compact-boundary" | "rebuild-context"
    )
}

fn missing(quality: &TransitionQuality) -> Vec<String> {
    if quality.legality != TransitionLegality::Blocked {
        return Vec::new();
    }
    quality
        .reason
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn forced_action_class(intent: TransitionIntent) -> Option<String> {
    match intent {
        TransitionIntent::AfterParseFault => Some("valid-act".to_string()),
        TransitionIntent::AfterParamFault => Some("exact-schema-example".to_string()),
        TransitionIntent::AfterToolFault => Some("alternate-native-tool".to_string()),
        TransitionIntent::AfterRepeatFault => Some("different-action-class".to_string()),
        TransitionIntent::UnderContextPressure => Some("runtime-compaction".to_string()),
        _ => None,
    }
}
