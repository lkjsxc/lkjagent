use crate::completion::missing_requirements;
use crate::guards::evaluate_guard;
use crate::model::GraphDefinition;
use crate::render_guidance::{compaction_instruction, completion_line, recovery_instruction};
use crate::state::TaskGraphState;
use crate::state_track::render_ranked_tracks;

pub fn render_graph_slice(graph: GraphDefinition, state: &TaskGraphState, budget: usize) -> String {
    let allowed = active_allowed(&graph, state);
    let blocked = blocked_tools(&graph, &allowed);
    let text = format!(
        "Graph state:\ncase: {}\nfamily: {}/{}\nphase: {}\nnode: {}\nconfidence: {}\nCurrent state: {}\nActive states: {}\nObjective: v{} {}\nDo not do: {}\nConstraints: {}\nAssumptions: {}\nRisks: {}\nSuccess criteria: {}\nActive plan step: {}\nRequired evidence: {}\nMissing evidence: {}\nAllowed tools now: {}\nBlocked tools now: {}\nPreferred next action: {}\nLegal transitions: {}\nContext packages: {}\nTouched paths: {}\nRecent faults: {}\nRecovery instruction if next action fails: {}\nCompaction instruction if context pressure rises: {}\nCompletion: {}",
        state.case_id.map_or_else(|| "new".to_string(), |id| id.to_string()),
        state.family.as_str(),
        state.subroute,
        state.phase.as_str(),
        state.active_node.0,
        state.confidence,
        state.status_text(),
        render_ranked_tracks(&state.state_tracks, 3),
        state.objective.version,
        state.objective.normalized,
        bounded(&state.objective.non_goals),
        join_constraints(state),
        join_assumptions(state),
        bounded_risks(state),
        bounded_success(state),
        state.plan.active_step_title(),
        state.evidence.requirement_ids().join(", "),
        missing_requirements(state).join(", "),
        allowed.join(", "),
        blocked.join(", "),
        next_action(state),
        legal_transition_line(&graph, state),
        state.context.selected_packages.join(", "),
        state.workspace.touched_paths.join(", "),
        recovery_line(state),
        recovery_instruction(state),
        compaction_instruction(state),
        completion_line(state)
    );
    fit_budget(&with_document_line(text, state), budget)
}

fn active_allowed(graph: &GraphDefinition, state: &TaskGraphState) -> Vec<&'static str> {
    graph
        .nodes
        .iter()
        .find(|node| node.id == state.active_node)
        .map_or_else(Vec::new, |node| node.allowed_actions.to_vec())
}

fn blocked_tools(graph: &GraphDefinition, allowed: &[&str]) -> Vec<&'static str> {
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

fn join_constraints(state: &TaskGraphState) -> String {
    state
        .constraints
        .iter()
        .map(|item| item.summary.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

fn join_assumptions(state: &TaskGraphState) -> String {
    state
        .assumptions
        .iter()
        .map(|item| item.summary.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

fn bounded(values: &[String]) -> String {
    if values.is_empty() {
        return "none".to_string();
    }
    values
        .iter()
        .take(4)
        .cloned()
        .collect::<Vec<_>>()
        .join("; ")
}

fn bounded_risks(state: &TaskGraphState) -> String {
    let values = state
        .risks
        .iter()
        .map(|item| format!("{} -> {}", item.summary, item.mitigation))
        .collect::<Vec<_>>();
    bounded(&values)
}

fn bounded_success(state: &TaskGraphState) -> String {
    let values = state
        .success_criteria
        .iter()
        .map(|item| item.summary.clone())
        .collect::<Vec<_>>();
    bounded(&values)
}

fn legal_transition_line(graph: &GraphDefinition, state: &TaskGraphState) -> String {
    let rows = graph
        .edges
        .iter()
        .filter(|edge| edge.from == state.active_node)
        .map(|edge| {
            let missing = edge
                .guards
                .iter()
                .filter_map(|guard| evaluate_guard(*guard, graph, state).err())
                .collect::<Vec<_>>();
            if missing.is_empty() {
                format!("{}:admitted", edge.to.0)
            } else {
                format!("{}:blocked({})", edge.to.0, missing.join("+"))
            }
        })
        .take(8)
        .collect::<Vec<_>>();
    bounded(&rows)
}

fn recovery_line(state: &TaskGraphState) -> String {
    state.recovery.strategy.clone().unwrap_or_else(|| {
        format!(
            "parse={} params={} tool={} repeat={}",
            state.recovery.parse_failures,
            state.recovery.param_failures,
            state.recovery.tool_failures,
            state.recovery.repeat_failures
        )
    })
}

fn next_action(state: &TaskGraphState) -> &'static str {
    if !state.plan.ready {
        "record graph.plan or inspect candidate files before planning"
    } else if state.context.selected_packages.is_empty() {
        "select context packages with graph.context"
    } else if state.completion.ready {
        "close with agent.done using evidence summary"
    } else {
        "execute the active plan step, record evidence, then verify"
    }
}

fn with_document_line(mut text: String, state: &TaskGraphState) -> String {
    if let Some(document) = &state.document {
        text.push_str(&format!(
            "\ndocument=root={} mode={:?} count={:?} topology={:?} audit={:?}",
            document.root,
            document.count_mode,
            document.requested_count,
            document.topology_status,
            document.audit_status
        ));
    }
    text
}

fn fit_budget(text: &str, budget: usize) -> String {
    if token_estimate(text) <= budget {
        return text.to_string();
    }
    let marker = "\n[graph slice narrowed]";
    let mut out = String::new();
    for ch in text.chars() {
        let candidate = format!("{out}{ch}{marker}");
        if token_estimate(&candidate) > budget {
            break;
        }
        out.push(ch);
    }
    format!("{out}{marker}")
}

fn token_estimate(text: &str) -> usize {
    text.len().saturating_add(3) / 4
}
