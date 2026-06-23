use super::model::{FaultClass, RuntimeFault};

pub(super) fn fault_class(fault: RuntimeFault) -> FaultClass {
    match fault {
        RuntimeFault::Parse => FaultClass::Parse,
        RuntimeFault::Parameter | RuntimeFault::Schema => FaultClass::Parameter,
        RuntimeFault::ToolRuntime | RuntimeFault::PolicyContradiction => FaultClass::Tool,
        RuntimeFault::Repeat => FaultClass::Repeat,
        RuntimeFault::EndpointFault => FaultClass::Endpoint,
        RuntimeFault::TurnBudgetExhausted => FaultClass::Budget,
        RuntimeFault::ContextInvalid | RuntimeFault::MaintenanceConflict => FaultClass::Context,
        RuntimeFault::VerificationMismatch => FaultClass::Verification,
        RuntimeFault::CompactionPressure | RuntimeFault::CompactionResumeGap => {
            FaultClass::Compaction
        }
        RuntimeFault::PayloadTooLarge => FaultClass::Payload,
        RuntimeFault::ArtifactAuditFailure
        | RuntimeFault::WeakArtifactContent
        | RuntimeFault::FalseCompletion
        | RuntimeFault::CompletionRefused => FaultClass::Completion,
    }
}

pub(super) fn recovery_route(class: FaultClass) -> &'static str {
    match class {
        FaultClass::Parse => "recover-parse",
        FaultClass::Parameter => "recover-params",
        FaultClass::Tool => "recover-tool",
        FaultClass::Repeat => "recover-repeat",
        FaultClass::Endpoint => "recover-endpoint",
        FaultClass::Budget => "recover-budget",
        FaultClass::Context => "recover-context",
        FaultClass::Verification => "recover-verification",
        FaultClass::Compaction => "recover-compaction",
        FaultClass::Payload => "recover-by-bounded-write",
        FaultClass::Completion => "recover-completion",
    }
}

pub(super) fn escalation_route(class: FaultClass) -> &'static str {
    match class {
        FaultClass::Payload => "smaller-batch",
        FaultClass::Repeat => "blocked-handoff-or-smaller-scope",
        FaultClass::Completion => "blocked-completion-handoff",
        FaultClass::Endpoint
        | FaultClass::Budget
        | FaultClass::Context
        | FaultClass::Compaction => "blocked-handoff",
        _ => "alternate-native-tool-or-smaller-scope",
    }
}

pub(super) fn blocked_handoff_behavior(class: FaultClass) -> &'static str {
    match class {
        FaultClass::Completion => "preserve open case with missing gates",
        FaultClass::Budget | FaultClass::Endpoint | FaultClass::Compaction => {
            "write partial handoff with exact next action"
        }
        _ => "continue independent admitted work before handoff",
    }
}
