use crate::kernel::active_mode::ActiveMode;
use crate::kernel::facts::{
    ArtifactFacts, CaseFacts, ContextFacts, EvidenceFacts, GraphFacts, MaintenanceFacts,
    ObservationFacts, QueueFacts,
};
use crate::kernel::fault::RuntimeFault;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeSnapshotId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeEventId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ToolName(String);

impl ToolName {
    pub fn new(value: impl Into<String>) -> Result<Self, KernelTextError> {
        non_empty(value.into()).map(Self)
    }

    pub fn from_static(value: &'static str) -> Self {
        Self(value.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<ToolName> for String {
    fn from(value: ToolName) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthorityFingerprint(String);

impl AuthorityFingerprint {
    pub fn new(value: impl Into<String>) -> Result<Self, KernelTextError> {
        non_empty(value.into()).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StalenessFingerprint(String);

impl StalenessFingerprint {
    pub fn new(value: impl Into<String>) -> Result<Self, KernelTextError> {
        non_empty(value.into()).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelTextError {
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSnapshot {
    pub snapshot_id: RuntimeSnapshotId,
    pub active_mode: ActiveMode,
    pub case: CaseFacts,
    pub graph: GraphFacts,
    pub queue: QueueFacts,
    pub evidence: EvidenceFacts,
    pub artifact: ArtifactFacts,
    pub latest_fault: Option<RuntimeFault>,
    pub retry_count: u32,
    pub prior_action_fingerprint: Option<String>,
    pub parameter_shape_fingerprint: Option<String>,
    pub recovery_route: Option<String>,
    pub observation: ObservationFacts,
    pub context: ContextFacts,
    pub maintenance: MaintenanceFacts,
    pub latest_decision_id: Option<String>,
    pub prompt_frame_fingerprint: Option<String>,
    pub authority_fingerprint: AuthorityFingerprint,
    pub staleness_fingerprint: StalenessFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSnapshotInput {
    pub snapshot_id: RuntimeSnapshotId,
    pub case: CaseFacts,
    pub graph: GraphFacts,
    pub queue: QueueFacts,
    pub evidence: EvidenceFacts,
    pub artifact: ArtifactFacts,
    pub context: ContextFacts,
    pub maintenance: MaintenanceFacts,
    pub authority_fingerprint: AuthorityFingerprint,
    pub staleness_fingerprint: StalenessFingerprint,
}

impl RuntimeSnapshot {
    pub fn new(input: RuntimeSnapshotInput) -> Self {
        Self {
            snapshot_id: input.snapshot_id,
            active_mode: ActiveMode::ClosedIdle,
            case: input.case,
            graph: input.graph,
            queue: input.queue,
            evidence: input.evidence,
            artifact: input.artifact,
            latest_fault: None,
            retry_count: 0,
            prior_action_fingerprint: None,
            parameter_shape_fingerprint: None,
            recovery_route: None,
            observation: ObservationFacts::default(),
            context: input.context,
            maintenance: input.maintenance,
            latest_decision_id: None,
            prompt_frame_fingerprint: None,
            authority_fingerprint: input.authority_fingerprint,
            staleness_fingerprint: input.staleness_fingerprint,
        }
    }

    pub fn owner_work_exists(&self) -> bool {
        self.queue.has_owner_work() || self.case.owner_work_exists()
    }
}

fn non_empty(value: String) -> Result<String, KernelTextError> {
    if value.trim().is_empty() {
        Err(KernelTextError::Empty)
    } else {
        Ok(value)
    }
}
