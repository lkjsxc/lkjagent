use crate::model::GraphDefinition;
use crate::state::TaskGraphState;

pub fn render_graph_slice(graph: GraphDefinition, state: &TaskGraphState, budget: usize) -> String {
    let missing = missing(state);
    let transitions = graph
        .edges
        .iter()
        .filter(|edge| edge.from == state.active_node)
        .map(|edge| edge.to.0)
        .collect::<Vec<_>>()
        .join(", ");
    let text = format!(
        "case={}\nfamily={}\nphase={}\nnode={}\nconfidence={}\npackages={}\nmissing_evidence={}\nlegal_transitions={}\nplan={}",
        state.case_id.map_or_else(|| "new".to_string(), |id| id.to_string()),
        state.family.as_str(),
        state.phase.as_str(),
        state.active_node.0,
        state.confidence,
        state.selected_packages.join(", "),
        missing.join(", "),
        transitions,
        state.plan
    );
    fit_budget(&text, budget)
}

fn missing(state: &TaskGraphState) -> Vec<String> {
    state
        .evidence_requirements
        .iter()
        .filter(|requirement| {
            !state
                .evidence
                .iter()
                .any(|evidence| evidence.requirement == **requirement)
        })
        .cloned()
        .collect()
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
