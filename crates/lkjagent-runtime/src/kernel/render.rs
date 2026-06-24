use crate::kernel::active_mode::ActiveMode;
use crate::kernel::decision::{RuntimeDecision, RuntimeMission};
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
