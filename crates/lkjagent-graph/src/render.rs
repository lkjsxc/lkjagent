use crate::completion::missing_requirements;
use crate::model::GraphDefinition;
use crate::state::TaskGraphState;

pub fn render_graph_slice(graph: GraphDefinition, state: &TaskGraphState, budget: usize) -> String {
    let allowed = active_allowed(&graph, state);
    let blocked = blocked_tools(&graph, &allowed);
    let transitions = graph
        .edges
        .iter()
        .filter(|edge| edge.from == state.active_node)
        .map(|edge| edge.to.0)
        .collect::<Vec<_>>()
        .join(", ");
    let text = format!(
        "case={} family={} phase={} node={} confidence={}\nobjective={}\nconstraints={}\nassumptions={}\nactive_step={}\nmissing_evidence={}\nallowed_tools={}\nblocked_tools={}\nlegal_transitions={}\npackages={}\ntouched_paths={}\nrecovery={}\ncompletion_ready={}\nnext={}",
        state.case_id.map_or_else(|| "new".to_string(), |id| id.to_string()),
        state.family.as_str(),
        state.phase.as_str(),
        state.active_node.0,
        state.confidence,
        state.objective.normalized,
        join_constraints(state),
        join_assumptions(state),
        state.plan.active_step_title(),
        missing_requirements(state).join(", "),
        allowed.join(", "),
        blocked.join(", "),
        transitions,
        state.context.selected_packages.join(", "),
        state.workspace.touched_paths.join(", "),
        recovery_line(state),
        state.completion.ready,
        next_action(state)
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

fn recovery_line(state: &TaskGraphState) -> String {
    state.recovery.strategy.clone().unwrap_or_else(|| {
        format!(
            "parse={} tool={} repeat={}",
            state.recovery.parse_failures,
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
