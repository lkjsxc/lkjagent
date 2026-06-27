use crate::kernel::event_kind::RuntimeEventKind;
use crate::kernel::fault::{FaultClass, FaultKey, RuntimeFault};
use crate::kernel::snapshot::ToolName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeEvent {
    OwnerMessageReceived,
    QueueChanged,
    CaseOpened,
    CaseResumed,
    PromptFrameRendered,
    PromptFrameSkipped,
    EndpointCallRequested,
    EndpointResponseReceived,
    EndpointFault {
        fault: RuntimeFault,
    },
    ProviderAnomaly {
        class: String,
    },
    ModelActionParsed {
        tool: ToolName,
    },
    ParseFault {
        fault_key: Option<FaultKey>,
    },
    ImplicitEnvelopeNormalized,
    SchemaFault {
        fault_key: Option<FaultKey>,
    },
    AdmissionRequested {
        tool: ToolName,
    },
    AdmissionAccepted {
        tool: ToolName,
    },
    AdmissionRefused {
        tool: ToolName,
        fault_class: FaultClass,
    },
    StaleActionRefused {
        tool: ToolName,
    },
    RepeatedActionRefused {
        fault_key: FaultKey,
    },
    ToolStarted {
        tool: ToolName,
    },
    ToolSucceeded {
        tool: ToolName,
    },
    ToolFailed {
        tool: ToolName,
        fault: RuntimeFault,
    },
    RepeatActionDetected {
        fault_key: FaultKey,
    },
    PayloadOverflowDetected {
        fault_key: Option<FaultKey>,
    },
    EvidenceAdded,
    ArtifactPlanned,
    ArtifactRootMissing,
    ArtifactApplied,
    ArtifactAudited,
    ArtifactAuditPassed,
    ArtifactAuditFailed,
    ArtifactWeakPathFound,
    ArtifactBatchCursorAdvanced,
    VerificationRequested,
    VerificationPassed,
    VerificationFailed,
    CompletionRequested,
    CompletionAccepted,
    CompletionRefused,
    CompletionBlocked,
    CaseClosed,
    ContextPressureDetected,
    CompactionStarted,
    CompactionCompleted,
    MaintenanceTick,
    MaintenanceDue,
    MaintenanceStarted,
    MaintenanceDeferred,
    MaintenanceNoop,
    MaintenanceNoopCooldownRecorded,
    MaintenanceCompleted,
    TurnBudgetCheckpoint,
    TurnBudgetExhausted,
    OwnerInputRequired,
    BlockedHandoffRecorded,
    ClosedIdleSelected,
}

impl RuntimeEvent {
    pub fn kind(&self) -> RuntimeEventKind {
        match self {
            Self::OwnerMessageReceived => RuntimeEventKind::OwnerMessageReceived,
            Self::QueueChanged => RuntimeEventKind::QueueChanged,
            Self::CaseOpened => RuntimeEventKind::CaseOpened,
            Self::CaseResumed => RuntimeEventKind::CaseResumed,
            Self::PromptFrameRendered => RuntimeEventKind::PromptFrameRendered,
            Self::PromptFrameSkipped => RuntimeEventKind::PromptFrameSkipped,
            Self::EndpointCallRequested => RuntimeEventKind::EndpointCallRequested,
            Self::EndpointResponseReceived => RuntimeEventKind::EndpointResponseReceived,
            Self::EndpointFault { .. } => RuntimeEventKind::EndpointFault,
            Self::ProviderAnomaly { .. } => RuntimeEventKind::ProviderAnomaly,
            Self::ModelActionParsed { .. } => RuntimeEventKind::ModelActionParsed,
            Self::ParseFault { .. } => RuntimeEventKind::ParseFault,
            Self::ImplicitEnvelopeNormalized => RuntimeEventKind::ImplicitEnvelopeNormalized,
            Self::SchemaFault { .. } => RuntimeEventKind::SchemaFault,
            Self::AdmissionRequested { .. } => RuntimeEventKind::AdmissionRequested,
            Self::AdmissionAccepted { .. } => RuntimeEventKind::AdmissionAccepted,
            Self::AdmissionRefused { .. } => RuntimeEventKind::AdmissionRefused,
            Self::StaleActionRefused { .. } => RuntimeEventKind::StaleActionRefused,
            Self::RepeatedActionRefused { .. } => RuntimeEventKind::RepeatedActionRefused,
            Self::ToolStarted { .. } => RuntimeEventKind::ToolStarted,
            Self::ToolSucceeded { .. } => RuntimeEventKind::ToolSucceeded,
            Self::ToolFailed { .. } => RuntimeEventKind::ToolFailed,
            Self::RepeatActionDetected { .. } => RuntimeEventKind::RepeatActionDetected,
            Self::PayloadOverflowDetected { .. } => RuntimeEventKind::PayloadOverflowDetected,
            Self::EvidenceAdded => RuntimeEventKind::EvidenceAdded,
            Self::ArtifactPlanned => RuntimeEventKind::ArtifactPlanned,
            Self::ArtifactRootMissing => RuntimeEventKind::ArtifactRootMissing,
            Self::ArtifactApplied => RuntimeEventKind::ArtifactApplied,
            Self::ArtifactAudited => RuntimeEventKind::ArtifactAudited,
            Self::ArtifactAuditPassed => RuntimeEventKind::ArtifactAuditPassed,
            Self::ArtifactAuditFailed => RuntimeEventKind::ArtifactAuditFailed,
            Self::ArtifactWeakPathFound => RuntimeEventKind::ArtifactWeakPathFound,
            Self::ArtifactBatchCursorAdvanced => RuntimeEventKind::ArtifactBatchCursorAdvanced,
            Self::VerificationRequested => RuntimeEventKind::VerificationRequested,
            Self::VerificationPassed => RuntimeEventKind::VerificationPassed,
            Self::VerificationFailed => RuntimeEventKind::VerificationFailed,
            Self::CompletionRequested => RuntimeEventKind::CompletionRequested,
            Self::CompletionAccepted => RuntimeEventKind::CompletionAccepted,
            Self::CompletionRefused => RuntimeEventKind::CompletionRefused,
            Self::CompletionBlocked => RuntimeEventKind::CompletionBlocked,
            Self::CaseClosed => RuntimeEventKind::CaseClosed,
            Self::ContextPressureDetected => RuntimeEventKind::ContextPressureDetected,
            Self::CompactionStarted => RuntimeEventKind::CompactionStarted,
            Self::CompactionCompleted => RuntimeEventKind::CompactionCompleted,
            Self::MaintenanceTick => RuntimeEventKind::MaintenanceTick,
            Self::MaintenanceDue => RuntimeEventKind::MaintenanceDue,
            Self::MaintenanceStarted => RuntimeEventKind::MaintenanceStarted,
            Self::MaintenanceDeferred => RuntimeEventKind::MaintenanceDeferred,
            Self::MaintenanceNoop => RuntimeEventKind::MaintenanceNoop,
            Self::MaintenanceNoopCooldownRecorded => {
                RuntimeEventKind::MaintenanceNoopCooldownRecorded
            }
            Self::MaintenanceCompleted => RuntimeEventKind::MaintenanceCompleted,
            Self::TurnBudgetCheckpoint => RuntimeEventKind::TurnBudgetCheckpoint,
            Self::TurnBudgetExhausted => RuntimeEventKind::TurnBudgetExhausted,
            Self::OwnerInputRequired => RuntimeEventKind::OwnerInputRequired,
            Self::BlockedHandoffRecorded => RuntimeEventKind::BlockedHandoffRecorded,
            Self::ClosedIdleSelected => RuntimeEventKind::ClosedIdleSelected,
        }
    }
}
