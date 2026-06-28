use crate::kernel::active_mode::ActiveMode;
use crate::kernel::decision::{ActionTemplate, RuntimeDecision, RuntimeDecisionId, RuntimeMission};
use crate::kernel::snapshot::{RuntimeEventId, RuntimeSnapshot, ToolName};
use lkjagent_llm::wire::MAX_TOKENS;
use lkjagent_tools::dispatch::{valid_example_for, ExampleContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptCardData {
    pub mission: RuntimeMission,
    pub active_mode: ActiveMode,
    pub case_id: Option<String>,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub missing_evidence: Vec<String>,
    pub artifact_root: Option<String>,
    pub owner_objective: Option<String>,
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
    EventNotPersisted,
    RuntimeEffectHasNoPrompt,
}

pub fn render_prompt_frame(decision: &RuntimeDecision) -> Result<String, PromptRenderError> {
    let decision_id = match decision.decision_id {
        RuntimeDecisionId::Stored(id) => id,
        RuntimeDecisionId::Pending => return Err(PromptRenderError::DecisionNotPersisted),
    };
    let event_id = match decision.event_id {
        RuntimeEventId(0) => return Err(PromptRenderError::EventNotPersisted),
        RuntimeEventId(id) => id,
    };
    if !decision.active_mode.allows_model_call() {
        return Err(PromptRenderError::RuntimeEffectHasNoPrompt);
    }
    let card = decision.prompt_card.as_ref();
    let next_action = render_next_action(decision);
    Ok(format!(
        "Runtime Authority\n<runtime-card>\n<decision>{decision_id}</decision>\n<event>{event_id}</event>\n<mission>{}</mission>\n<mode>{}</mode>\n<case>{}</case>\n<node>{}</node>\n<phase>{}</phase>\n<root>{}</root>\n<missing>{}</missing>\n<must-use>{}</must-use>\n<blocked>{}</blocked>\n<budget>{MAX_TOKENS} output tokens</budget>\n<authority>{}</authority>\n<staleness>{}</staleness>\n<reason>runtime decision selected one legal next action</reason>\n</runtime-card>\n<next-action>\n{}\n</next-action>",
        decision.mission.as_str(),
        decision.active_mode.as_str(),
        optional(card.and_then(|data| data.case_id.as_deref())),
        optional(decision.graph_node.as_deref()),
        optional(decision.graph_phase.as_deref()),
        optional(card.and_then(|data| data.artifact_root.as_deref())),
        list_or_none(&decision.missing_evidence),
        must_use(decision),
        spaced_tool_list(&decision.admission_view.blocked_tools),
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
        artifact_root: snapshot.artifact.root.clone(),
        owner_objective: snapshot.case.owner_objective.clone(),
        admitted_tools: decision.admission_view.admitted_tools.clone(),
        blocked_tools: decision.admission_view.blocked_tools.clone(),
        next_action: decision.admission_view.exact_next_action.clone(),
        authority_fingerprint: snapshot.authority_fingerprint.as_str().to_string(),
        staleness_fingerprint: snapshot.staleness_fingerprint.as_str().to_string(),
    })
}

pub(crate) fn example_for(tool: &str, snapshot: &RuntimeSnapshot) -> String {
    let root = snapshot
        .artifact
        .root
        .as_deref()
        .unwrap_or("stories/active-artifact");
    let context = ExampleContext {
        artifact_root: Some(root.to_string()),
        owner_objective: snapshot.case.owner_objective.clone(),
        missing_evidence: snapshot.evidence.missing.clone(),
        ..ExampleContext::default()
    };
    valid_example_for(tool, context)
        .map(|example| example.render())
        .unwrap_or_else(|_| format!("<action>\n<tool>{tool}</tool>\n</action>"))
}

fn render_next_action(decision: &RuntimeDecision) -> String {
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { body, .. }) => body.clone(),
        Some(ActionTemplate::RuntimeEffect(_)) => "runtime effect".to_string(),
        Some(ActionTemplate::ExternalOwnerWait) => "wait for owner input".to_string(),
        Some(ActionTemplate::ClosedIdle) | None => "none".to_string(),
    }
}

fn must_use(decision: &RuntimeDecision) -> &str {
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { tool, .. }) => tool.as_str(),
        _ => "none",
    }
}

fn spaced_tool_list(tools: &[ToolName]) -> String {
    let names: Vec<&str> = tools.iter().map(ToolName::as_str).collect();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(" ")
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
