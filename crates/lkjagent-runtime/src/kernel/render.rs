use crate::kernel::active_mode::ActiveMode;
use crate::kernel::decision::{ActionTemplate, RuntimeDecision, RuntimeDecisionId, RuntimeMission};
use crate::kernel::snapshot::{RuntimeSnapshot, ToolName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptCardData {
    pub mission: RuntimeMission,
    pub active_mode: ActiveMode,
    pub case_id: Option<String>,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub missing_evidence: Vec<String>,
    pub admitted_tools: Vec<ToolName>,
    pub blocked_tools: Vec<ToolName>,
    pub next_action: Option<String>,
    pub authority_fingerprint: String,
    pub staleness_fingerprint: String,
}

impl PromptCardData {
    pub fn admitted_tool_names(&self) -> Vec<&str> {
        self.admitted_tools.iter().map(ToolName::as_str).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptRenderError {
    DecisionNotPersisted,
    RuntimeEffectHasNoPrompt,
}

pub fn render_prompt_frame(decision: &RuntimeDecision) -> Result<String, PromptRenderError> {
    let decision_id = match decision.decision_id {
        RuntimeDecisionId::Stored(id) => id,
        RuntimeDecisionId::Pending => return Err(PromptRenderError::DecisionNotPersisted),
    };
    if !decision.active_mode.allows_model_call() {
        return Err(PromptRenderError::RuntimeEffectHasNoPrompt);
    }
    let next_action = render_next_action(decision);
    Ok(format!(
        "Runtime Authority\ndecision_id={decision_id}\nmission={}\nmode={}\ngraph_node={}\ngraph_phase={}\nmissing_evidence={}\nadmitted_tools={}\nblocked_tools={}\nauthority_fingerprint={}\nstaleness_fingerprint={}\nnext_action:\n{}",
        decision.mission.as_str(),
        decision.active_mode.as_str(),
        optional(decision.graph_node.as_deref()),
        optional(decision.graph_phase.as_deref()),
        list_or_none(&decision.missing_evidence),
        tool_list(&decision.admission_view.admitted_tools),
        tool_list(&decision.admission_view.blocked_tools),
        decision.authority_fingerprint.as_str(),
        decision.staleness_fingerprint.as_str(),
        next_action,
    ))
}

pub(crate) fn prompt_card_for(
    snapshot: &RuntimeSnapshot,
    mission: RuntimeMission,
    active_mode: ActiveMode,
    decision: &RuntimeDecision,
) -> Option<PromptCardData> {
    if !active_mode.allows_model_call() {
        return None;
    }
    Some(PromptCardData {
        mission,
        active_mode,
        case_id: snapshot.case.case_id.clone(),
        graph_node: snapshot.graph.node.clone(),
        graph_phase: snapshot.graph.phase.clone(),
        missing_evidence: snapshot.evidence.missing.clone(),
        admitted_tools: decision.admission_view.admitted_tools.clone(),
        blocked_tools: decision.admission_view.blocked_tools.clone(),
        next_action: decision.admission_view.exact_next_action.clone(),
        authority_fingerprint: snapshot.authority_fingerprint.as_str().to_string(),
        staleness_fingerprint: snapshot.staleness_fingerprint.as_str().to_string(),
    })
}

pub(crate) fn example_for(tool: &str, snapshot: &RuntimeSnapshot) -> String {
    match (tool, snapshot.artifact.root.as_deref()) {
        ("fs.batch_write", Some(root)) => format!(
            "<act>\n<tool>fs.batch_write</tool>\n<files>\npath: {root}/README.md\ncontent:\n# Title\n\nConcrete content.\n</files>\n</act>"
        ),
        _ => format!("<act>\n<tool>{tool}</tool>\n</act>"),
    }
}

fn render_next_action(decision: &RuntimeDecision) -> String {
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { body, .. }) => body.clone(),
        Some(ActionTemplate::RuntimeEffect(_)) => "runtime effect".to_string(),
        Some(ActionTemplate::ExternalOwnerWait) => "wait for owner input".to_string(),
        Some(ActionTemplate::ClosedIdle) | None => "none".to_string(),
    }
}

fn tool_list(tools: &[ToolName]) -> String {
    let names: Vec<&str> = tools.iter().map(ToolName::as_str).collect();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(",")
    }
}

fn list_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn optional(value: Option<&str>) -> &str {
    value.unwrap_or("none")
}
