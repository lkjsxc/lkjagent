use crate::kernel::active_mode::ActiveMode;
use crate::kernel::admission::ToolAdmissionView;
use crate::kernel::snapshot::{StalenessFingerprint, ToolName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionRequest {
    pub requested_tool: ToolName,
    pub decision_id: Option<String>,
    pub prompt_frame_id: Option<String>,
    pub staleness_fingerprint: StalenessFingerprint,
    pub action_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionRefusalKind {
    StaleDecision,
    BlockedTool,
    ToolNotAdmitted,
    CompletionBlocked,
    RepeatFingerprintExhausted,
    DecisionNotCurrent,
    PromptFrameNotCurrent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionDecision {
    pub admitted: bool,
    pub requested_tool: ToolName,
    pub active_mode: ActiveMode,
    pub refusal_kind: Option<AdmissionRefusalKind>,
    pub reason: String,
    pub admitted_tools: Vec<ToolName>,
    pub blocked_tools: Vec<ToolName>,
    pub exact_next_action: Option<String>,
}

impl AdmissionRequest {
    pub fn new(
        requested_tool: ToolName,
        staleness_fingerprint: StalenessFingerprint,
        action_fingerprint: impl Into<String>,
    ) -> Self {
        Self {
            requested_tool,
            decision_id: None,
            prompt_frame_id: None,
            staleness_fingerprint,
            action_fingerprint: action_fingerprint.into(),
        }
    }

    pub fn with_current_ids(
        mut self,
        decision_id: impl Into<String>,
        prompt_frame_id: impl Into<String>,
    ) -> Self {
        self.decision_id = Some(decision_id.into());
        self.prompt_frame_id = Some(prompt_frame_id.into());
        self
    }
}

pub fn admit_requested_tool(
    view: &ToolAdmissionView,
    request: AdmissionRequest,
) -> AdmissionDecision {
    if view.decision_id.is_some() && request.decision_id != view.decision_id {
        return refused(
            view,
            request,
            AdmissionRefusalKind::DecisionNotCurrent,
            "decision id is not current",
        );
    }
    if view.prompt_frame_id.is_some() && request.prompt_frame_id != view.prompt_frame_id {
        return refused(
            view,
            request,
            AdmissionRefusalKind::PromptFrameNotCurrent,
            "prompt frame id is not current",
        );
    }
    if request.staleness_fingerprint != view.staleness_fingerprint {
        return refused(
            view,
            request,
            AdmissionRefusalKind::StaleDecision,
            "stale decision",
        );
    }
    if view
        .refused_action_fingerprints
        .iter()
        .any(|fingerprint| fingerprint == &request.action_fingerprint)
    {
        let reason = view.exhausted_fault_class.map_or_else(
            || "action fingerprint exhausted".to_string(),
            |class| format!("action fingerprint exhausted for {class:?}"),
        );
        return refused(
            view,
            request,
            AdmissionRefusalKind::RepeatFingerprintExhausted,
            &reason,
        );
    }
    if view
        .blocked_tools
        .iter()
        .any(|tool| tool == &request.requested_tool)
    {
        return refused(
            view,
            request,
            AdmissionRefusalKind::BlockedTool,
            "tool blocked by authority",
        );
    }
    if request.requested_tool.as_str() == "agent.done" && !view.completion_allowed {
        return refused(
            view,
            request,
            AdmissionRefusalKind::CompletionBlocked,
            "completion blocked",
        );
    }
    if !view.admits(&request.requested_tool) {
        return refused(
            view,
            request,
            AdmissionRefusalKind::ToolNotAdmitted,
            "tool not admitted",
        );
    }
    AdmissionDecision {
        admitted: true,
        requested_tool: request.requested_tool,
        active_mode: view.active_mode,
        refusal_kind: None,
        reason: "admitted".to_string(),
        admitted_tools: view.admitted_tools.clone(),
        blocked_tools: view.blocked_tools.clone(),
        exact_next_action: view.exact_next_action.clone(),
    }
}

fn refused(
    view: &ToolAdmissionView,
    request: AdmissionRequest,
    kind: AdmissionRefusalKind,
    reason: &str,
) -> AdmissionDecision {
    AdmissionDecision {
        admitted: false,
        requested_tool: request.requested_tool,
        active_mode: view.active_mode,
        refusal_kind: Some(kind),
        reason: reason.to_string(),
        admitted_tools: view.admitted_tools.clone(),
        blocked_tools: view.blocked_tools.clone(),
        exact_next_action: view.exact_next_action.clone(),
    }
}
