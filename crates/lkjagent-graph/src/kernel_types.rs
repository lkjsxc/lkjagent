use crate::kernel_events::{EvidenceLedger, FaultLedger};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaseId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Objective {
    pub raw: String,
    pub normalized: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseLifecycle {
    Queued,
    Active,
    Paused,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateNode {
    Intake,
    Planning,
    Executing,
    Auditing,
    Recovering,
    Compacting,
    CompletionBlocked,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Intake,
    Plan,
    Execute,
    Audit,
    Recover,
    Verify,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardState {
    pub node: StateNode,
    pub phase: Phase,
    pub allowed_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub completion_gates: Vec<CompletionGate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseState {
    pub case_id: CaseId,
    pub lifecycle: CaseLifecycle,
    pub hard_state: HardState,
    pub state_vector: StateVector,
    pub objective: Objective,
    pub evidence: EvidenceLedger,
    pub faults: FaultLedger,
    pub repeated_signatures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateVector {
    pub tracks: Vec<StateTrack>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateTrack {
    pub label: TrackLabel,
    pub posture: Posture,
    pub weight: Weight,
    pub confidence: Confidence,
    pub source: TrackSource,
    pub evidence_gap: Option<String>,
    pub guard: Option<GuardPolicy>,
    pub decay: DecayPolicy,
    pub last_updated: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrackLabel {
    ObjectiveNormalization,
    DocumentationContract,
    StructureSeed,
    StructureExpansion,
    StructureConnectivity,
    SemanticCoverage,
    DuplicationRisk,
    MockContentRisk,
    ModelSpecificNaming,
    PromptContract,
    Planning,
    DocumentStructure,
    ArtifactContract,
    ArtifactReadiness,
    ArtifactDrift,
    ParseRecovery,
    ActionParamReliability,
    ToolExecutionRecovery,
    EvidenceGap,
    ContextPressure,
    ContextSnapshotMismatch,
    QueueInterruption,
    CompletionReadiness,
    ObservabilityLedger,
    RepeatedActionRisk,
    MaintenanceOpportunity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Posture {
    Planning,
    Structuring,
    Executing,
    Auditing,
    Repairing,
    Recovering,
    Verifying,
    Observing,
    Compacting,
    Scheduling,
    Maintaining,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weight(pub f32);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Confidence(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackSource {
    Runtime,
    Parser,
    ToolSchema,
    Audit,
    Artifact,
    Context,
    Queue,
    Completion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardPolicy {
    RestrictLargePayload,
    BlockArtifactMutation,
    BlockMutation,
    RequireQueueClassification,
    BlockCompletion,
    BlockRepeatedSignature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecayPolicy {
    None,
    Slow,
    OnPassingAudit,
    OnValidAction,
    OnClassification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolIntent {
    pub name: String,
    pub signature: String,
    pub payload_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAuthorization {
    pub allowed: bool,
    pub reason: String,
    pub blocked_by: Vec<TrackLabel>,
    pub preferred_tools: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionGate {
    pub name: String,
    pub satisfied: bool,
}
