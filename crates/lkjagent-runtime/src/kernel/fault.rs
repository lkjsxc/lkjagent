#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultClass {
    Parse,
    Parameter,
    Schema,
    Tool,
    Repeat,
    Endpoint,
    Budget,
    Context,
    Verification,
    Compaction,
    Payload,
    Completion,
    MaintenanceConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeFault {
    Parse,
    Parameter,
    Schema,
    ToolRuntime,
    Repeat,
    PolicyContradiction,
    PayloadTooLarge,
    ArtifactAuditFailure,
    WeakArtifactContent,
    FalseCompletion,
    VerificationMismatch,
    CompletionRefused,
    CompactionPressure,
    CompactionResumeGap,
    MaintenanceConflict,
    EndpointFault,
    TurnBudgetExhausted,
    ContextInvalid,
}

impl RuntimeFault {
    pub fn class(self) -> FaultClass {
        match self {
            Self::Parse => FaultClass::Parse,
            Self::Parameter => FaultClass::Parameter,
            Self::Schema => FaultClass::Schema,
            Self::ToolRuntime | Self::PolicyContradiction => FaultClass::Tool,
            Self::Repeat => FaultClass::Repeat,
            Self::EndpointFault => FaultClass::Endpoint,
            Self::TurnBudgetExhausted => FaultClass::Budget,
            Self::ContextInvalid => FaultClass::Context,
            Self::VerificationMismatch => FaultClass::Verification,
            Self::CompactionPressure | Self::CompactionResumeGap => FaultClass::Compaction,
            Self::PayloadTooLarge => FaultClass::Payload,
            Self::CompletionRefused | Self::FalseCompletion => FaultClass::Completion,
            Self::MaintenanceConflict => FaultClass::MaintenanceConflict,
            Self::ArtifactAuditFailure | Self::WeakArtifactContent => FaultClass::Verification,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FaultKey {
    pub case_id: Option<String>,
    pub graph_node: Option<String>,
    pub tool: Option<String>,
    pub parameter_shape: Option<String>,
    pub fault_class: FaultClass,
    pub action_fingerprint: Option<String>,
}

impl FaultKey {
    pub fn new(fault_class: FaultClass) -> Self {
        Self {
            case_id: None,
            graph_node: None,
            tool: None,
            parameter_shape: None,
            fault_class,
            action_fingerprint: None,
        }
    }

    pub fn with_case(mut self, case_id: impl Into<String>) -> Self {
        self.case_id = Some(case_id.into());
        self
    }

    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.tool = Some(tool.into());
        self
    }

    pub fn with_parameter_shape(mut self, shape: impl Into<String>) -> Self {
        self.parameter_shape = Some(shape.into());
        self
    }

    pub fn with_action_fingerprint(mut self, fingerprint: impl Into<String>) -> Self {
        self.action_fingerprint = Some(fingerprint.into());
        self
    }
}
