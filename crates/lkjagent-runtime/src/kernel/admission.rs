use crate::kernel::active_mode::ActiveMode;
use crate::kernel::decision::RuntimeMission;
use crate::kernel::fault::FaultClass;
use crate::kernel::snapshot::{StalenessFingerprint, ToolName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAdmissionView {
    pub decision_id: Option<String>,
    pub prompt_frame_id: Option<String>,
    pub active_mode: ActiveMode,
    pub admitted_tools: Vec<ToolName>,
    pub blocked_tools: Vec<ToolName>,
    pub completion_allowed: bool,
    pub shell_allowed: bool,
    pub missing_evidence: Vec<String>,
    pub staleness_fingerprint: StalenessFingerprint,
    pub exact_next_action: Option<String>,
    pub exhausted_fault_class: Option<FaultClass>,
    pub refused_action_fingerprints: Vec<String>,
}

impl ToolAdmissionView {
    pub fn new(
        active_mode: ActiveMode,
        admitted_tools: Vec<ToolName>,
        blocked_tools: Vec<ToolName>,
        staleness_fingerprint: StalenessFingerprint,
    ) -> Self {
        Self {
            decision_id: None,
            prompt_frame_id: None,
            active_mode,
            admitted_tools,
            blocked_tools,
            completion_allowed: false,
            shell_allowed: false,
            missing_evidence: Vec::new(),
            staleness_fingerprint,
            exact_next_action: None,
            exhausted_fault_class: None,
            refused_action_fingerprints: Vec::new(),
        }
    }

    pub fn admits(&self, tool: &ToolName) -> bool {
        self.admitted_tools.iter().any(|admitted| admitted == tool)
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

    pub fn with_missing_evidence(mut self, missing_evidence: Vec<String>) -> Self {
        self.missing_evidence = missing_evidence;
        self
    }

    pub fn with_completion_allowed(mut self, completion_allowed: bool) -> Self {
        self.completion_allowed = completion_allowed;
        self
    }

    pub fn with_exact_next_action(mut self, exact_next_action: impl Into<String>) -> Self {
        self.exact_next_action = Some(exact_next_action.into());
        self
    }

    pub fn with_exhausted_fault_guard(
        mut self,
        fault_class: FaultClass,
        action_fingerprint: impl Into<String>,
    ) -> Self {
        self.exhausted_fault_class = Some(fault_class);
        self.refused_action_fingerprints
            .push(action_fingerprint.into());
        self
    }
}

pub(crate) fn admitted_tools_for(mission: RuntimeMission) -> Vec<ToolName> {
    match mission {
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle => Vec::new(),
        RuntimeMission::OwnerRecovery => tools(&[
            "graph.state",
            "graph.recover",
            "graph.plan",
            "graph.evidence",
            "graph.transition",
            "artifact.next",
            "artifact.audit",
            "artifact.apply",
            "doc.audit",
            "fs.read",
            "fs.list",
            "fs.stat",
            "fs.write",
            "fs.batch_write",
            "workspace.summary",
        ]),
        RuntimeMission::SchemaRepair => tools(&["fs.batch_write", "artifact.next", "graph.state"]),
        RuntimeMission::ArtifactRepair => tools(&[
            "artifact.next",
            "artifact.audit",
            "doc.audit",
            "fs.batch_write",
            "fs.read",
            "fs.list",
        ]),
        RuntimeMission::VerificationRepair => {
            tools(&["artifact.audit", "doc.audit", "graph.state"])
        }
        RuntimeMission::OwnerExecution => tools(&[
            "graph.state",
            "graph.plan",
            "graph.evidence",
            "graph.note",
            "agent.ask",
            "artifact.apply",
            "artifact.next",
            "artifact.audit",
            "doc.audit",
            "fs.read",
            "fs.list",
            "fs.stat",
            "fs.write",
            "fs.batch_write",
            "workspace.summary",
        ]),
        RuntimeMission::OwnerVerification => tools(&["artifact.audit", "doc.audit"]),
        RuntimeMission::OwnerCompletion => tools(&[
            "artifact.audit",
            "doc.audit",
            "artifact.next",
            "fs.batch_write",
        ]),
        RuntimeMission::IdleMaintenance => tools(&[
            "memory.find",
            "memory.prune",
            "memory.save",
            "agent.done",
            "agent.ask",
        ]),
    }
}

pub(crate) fn blocked_tools_for(mission: RuntimeMission) -> Vec<ToolName> {
    match mission.active_mode() {
        ActiveMode::OwnerTask | ActiveMode::Recovery | ActiveMode::Compaction => {
            tools(&["memory.find", "memory.save"])
        }
        ActiveMode::Maintenance => tools(&["fs.batch_write"]),
        ActiveMode::ClosedIdle => Vec::new(),
    }
}

fn tools(names: &[&'static str]) -> Vec<ToolName> {
    names.iter().copied().map(ToolName::from_static).collect()
}
