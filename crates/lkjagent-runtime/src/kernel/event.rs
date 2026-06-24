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
    EndpointCallRequested,
    EndpointResponseReceived,
    EndpointFault {
        fault: RuntimeFault,
    },
    ModelActionParsed {
        tool: ToolName,
    },
    ParseFault {
        fault_key: Option<FaultKey>,
    },
    SchemaFault {
        fault_key: Option<FaultKey>,
    },
    AdmissionRequested {
        tool: ToolName,
    },
    AdmissionRefused {
        tool: ToolName,
        fault_class: FaultClass,
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
    ArtifactApplied,
    ArtifactAudited,
    ArtifactWeakPathFound,
    VerificationRequested,
    VerificationPassed,
    VerificationFailed,
    CompletionRequested,
    CompletionBlocked,
    CaseClosed,
    ContextPressureDetected,
    CompactionStarted,
    CompactionCompleted,
    MaintenanceTick,
    MaintenanceStarted,
    MaintenanceNoop,
    MaintenanceCompleted,
    TurnBudgetCheckpoint,
    TurnBudgetExhausted,
    OwnerInputRequired,
    BlockedHandoffRecorded,
}

impl RuntimeEvent {
    pub fn kind(&self) -> RuntimeEventKind {
        match self {
            Self::OwnerMessageReceived => RuntimeEventKind::OwnerMessageReceived,
            Self::QueueChanged => RuntimeEventKind::QueueChanged,
            Self::CaseOpened => RuntimeEventKind::CaseOpened,
            Self::CaseResumed => RuntimeEventKind::CaseResumed,
            Self::PromptFrameRendered => RuntimeEventKind::PromptFrameRendered,
            Self::EndpointCallRequested => RuntimeEventKind::EndpointCallRequested,
            Self::EndpointResponseReceived => RuntimeEventKind::EndpointResponseReceived,
            Self::EndpointFault { .. } => RuntimeEventKind::EndpointFault,
            Self::ModelActionParsed { .. } => RuntimeEventKind::ModelActionParsed,
            Self::ParseFault { .. } => RuntimeEventKind::ParseFault,
            Self::SchemaFault { .. } => RuntimeEventKind::SchemaFault,
            Self::AdmissionRequested { .. } => RuntimeEventKind::AdmissionRequested,
            Self::AdmissionRefused { .. } => RuntimeEventKind::AdmissionRefused,
            Self::ToolStarted { .. } => RuntimeEventKind::ToolStarted,
            Self::ToolSucceeded { .. } => RuntimeEventKind::ToolSucceeded,
            Self::ToolFailed { .. } => RuntimeEventKind::ToolFailed,
            Self::RepeatActionDetected { .. } => RuntimeEventKind::RepeatActionDetected,
            Self::PayloadOverflowDetected { .. } => RuntimeEventKind::PayloadOverflowDetected,
            Self::EvidenceAdded => RuntimeEventKind::EvidenceAdded,
            Self::ArtifactPlanned => RuntimeEventKind::ArtifactPlanned,
            Self::ArtifactApplied => RuntimeEventKind::ArtifactApplied,
            Self::ArtifactAudited => RuntimeEventKind::ArtifactAudited,
            Self::ArtifactWeakPathFound => RuntimeEventKind::ArtifactWeakPathFound,
            Self::VerificationRequested => RuntimeEventKind::VerificationRequested,
            Self::VerificationPassed => RuntimeEventKind::VerificationPassed,
            Self::VerificationFailed => RuntimeEventKind::VerificationFailed,
            Self::CompletionRequested => RuntimeEventKind::CompletionRequested,
            Self::CompletionBlocked => RuntimeEventKind::CompletionBlocked,
            Self::CaseClosed => RuntimeEventKind::CaseClosed,
            Self::ContextPressureDetected => RuntimeEventKind::ContextPressureDetected,
            Self::CompactionStarted => RuntimeEventKind::CompactionStarted,
            Self::CompactionCompleted => RuntimeEventKind::CompactionCompleted,
            Self::MaintenanceTick => RuntimeEventKind::MaintenanceTick,
            Self::MaintenanceStarted => RuntimeEventKind::MaintenanceStarted,
            Self::MaintenanceNoop => RuntimeEventKind::MaintenanceNoop,
            Self::MaintenanceCompleted => RuntimeEventKind::MaintenanceCompleted,
            Self::TurnBudgetCheckpoint => RuntimeEventKind::TurnBudgetCheckpoint,
            Self::TurnBudgetExhausted => RuntimeEventKind::TurnBudgetExhausted,
            Self::OwnerInputRequired => RuntimeEventKind::OwnerInputRequired,
            Self::BlockedHandoffRecorded => RuntimeEventKind::BlockedHandoffRecorded,
        }
    }
}
