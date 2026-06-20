use crate::case_fields::FieldStatus;
use crate::model::GraphDefinition;
use crate::state::TaskGraphState;

pub(crate) fn active_allowed(graph: &GraphDefinition, state: &TaskGraphState) -> Vec<&'static str> {
    let owner_question = state
        .open_questions
        .iter()
        .any(|question| question.owner_required && question.status == FieldStatus::Open);
    graph
        .nodes
        .iter()
        .find(|node| node.id == state.active_node)
        .map_or_else(Vec::new, |node| {
            node.allowed_actions
                .iter()
                .copied()
                .filter(|tool| *tool != "agent.ask" || owner_question)
                .collect()
        })
}

pub(crate) fn blocked_tools(graph: &GraphDefinition, allowed: &[&str]) -> Vec<&'static str> {
    let mut tools = graph
        .nodes
        .iter()
        .flat_map(|node| node.allowed_actions.iter().copied())
        .collect::<Vec<_>>();
    tools.sort_unstable();
    tools.dedup();
    tools
        .into_iter()
        .filter(|tool| !allowed.contains(tool))
        .take(12)
        .collect()
}
