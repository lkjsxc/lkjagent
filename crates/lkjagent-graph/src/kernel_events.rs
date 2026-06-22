#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub kind: EvidenceKind,
    pub owner: EvidenceOwner,
    pub fresh: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceKind {
    Plan,
    Observation,
    DocumentStructure,
    RelationConnectivity,
    SemanticQuality,
    ModelNameSanitization,
    ArtifactReadiness,
    ObjectiveMatch,
    Verification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceOwner {
    Runtime,
    DocAudit,
    RelationAudit,
    SemanticAudit,
    ModelNameAudit,
    ArtifactAudit,
    Verification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceLedger(pub Vec<Evidence>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultLedger(pub Vec<Fault>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fault {
    ParserFault,
    ToolParameterFault,
    ToolExecutionFault,
    AuditFailure,
    ArtifactProfileMismatch,
    ArtifactDrift,
    ArtifactReadinessFailure,
    RepeatedActionRefusal,
    ContextPressure,
    ContextSnapshotMismatch,
    CaseConflict,
    QueueInterruption,
    MaintenanceNoop,
    WorkspaceClaimFromMemory,
    EvidenceOwnershipViolation,
    CompletionGateFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditResult {
    pub kind: String,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolObservation {
    pub tool: String,
    pub succeeded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseEvent {
    ParseFault { consecutive: u8 },
    ParsedAction,
    ToolParameterFault { expected: String, received: String },
    RepeatedInvalidAction { signature: String },
    DocAudit { passed: bool },
    RelationAudit { passed: bool },
    MockContentAudit { passed: bool },
    ModelNameAudit { passed: bool },
    ArtifactObjectiveMismatch { reason: String },
    ArtifactAudit { passed: bool },
    ContextUsage { hard: bool },
    PostCompaction { matched: bool },
    OwnerTaskArrived,
    QueueClassified,
    MaintenanceCycleNoop { suppression_created: bool },
    WorkspaceClaimFromMemory,
    CompletionEvidenceReady,
}
