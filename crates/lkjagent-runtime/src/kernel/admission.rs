use crate::kernel::active_mode::ActiveMode;
use crate::kernel::decision::RuntimeMission;
use crate::kernel::snapshot::{StalenessFingerprint, ToolName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAdmissionView {
    pub active_mode: ActiveMode,
    pub admitted_tools: Vec<ToolName>,
    pub blocked_tools: Vec<ToolName>,
    pub completion_allowed: bool,
    pub shell_allowed: bool,
    pub missing_evidence: Vec<String>,
    pub staleness_fingerprint: StalenessFingerprint,
    pub exact_next_action: Option<String>,
}

impl ToolAdmissionView {
    pub fn new(
        active_mode: ActiveMode,
        admitted_tools: Vec<ToolName>,
        blocked_tools: Vec<ToolName>,
        staleness_fingerprint: StalenessFingerprint,
    ) -> Self {
        Self {
            active_mode,
            admitted_tools,
            blocked_tools,
            completion_allowed: false,
            shell_allowed: false,
            missing_evidence: Vec::new(),
            staleness_fingerprint,
            exact_next_action: None,
        }
    }

    pub fn admits(&self, tool: &ToolName) -> bool {
        self.admitted_tools.iter().any(|admitted| admitted == tool)
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
}

pub(crate) fn admitted_tools_for(mission: RuntimeMission) -> Vec<ToolName> {
    match mission {
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle => Vec::new(),
        RuntimeMission::OwnerRecovery => tools(&["graph.state", "artifact.next", "fs.list"]),
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
        RuntimeMission::OwnerExecution => {
            tools(&["graph.state", "artifact.next", "fs.batch_write"])
        }
        RuntimeMission::OwnerVerification => tools(&["artifact.audit", "doc.audit"]),
        RuntimeMission::OwnerCompletion => tools(&[
            "artifact.audit",
            "doc.audit",
            "artifact.next",
            "fs.batch_write",
        ]),
        RuntimeMission::IdleMaintenance => tools(&["memory.find", "memory.prune"]),
    }
}

pub(crate) fn blocked_tools_for(mission: RuntimeMission) -> Vec<ToolName> {
    match mission.active_mode() {
        ActiveMode::OwnerTask | ActiveMode::Recovery | ActiveMode::Compaction => {
            tools(&["memory.find", "memory.save"])
        }
        ActiveMode::Maintenance => tools(&["agent.done", "fs.batch_write"]),
        ActiveMode::ClosedIdle => Vec::new(),
    }
}

fn tools(names: &[&'static str]) -> Vec<ToolName> {
    names.iter().copied().map(ToolName::from_static).collect()
}
