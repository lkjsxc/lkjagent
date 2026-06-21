use super::admission::admit_tool;
use super::model::{RuntimeDecision, RuntimeEvent, RuntimeFault, RuntimeSnapshot};

pub fn decide(snapshot: &RuntimeSnapshot, event: RuntimeEvent) -> RuntimeDecision {
    if snapshot.context_pressure_active || event == RuntimeEvent::ContextPressureRaised {
        return RuntimeDecision::StartCompaction;
    }
    match event {
        RuntimeEvent::OwnerMessageQueued | RuntimeEvent::QueueBecameNonEmpty => {
            RuntimeDecision::AskEndpoint
        }
        RuntimeEvent::EndpointActionParsed { tool } => {
            let admission = admit_tool(snapshot, &tool);
            if admission.admitted {
                RuntimeDecision::ExecuteTool(admission)
            } else if tool == "agent.done" {
                RuntimeDecision::BlockCompletion(admission)
            } else if snapshot.recovery_ladder_active {
                RuntimeDecision::ContinueRecovery(admission)
            } else {
                RuntimeDecision::RefuseAction(admission)
            }
        }
        RuntimeEvent::EndpointActionParseFailed => RuntimeDecision::StartRecovery,
        RuntimeEvent::ToolFailed { fault } => decision_for_fault(snapshot, fault),
        RuntimeEvent::ToolSucceeded => {
            if snapshot.missing_evidence.is_empty() {
                RuntimeDecision::StartVerification
            } else {
                RuntimeDecision::AskEndpoint
            }
        }
        RuntimeEvent::VerificationSucceeded => {
            if snapshot.missing_evidence.is_empty() && !snapshot.recovery_ladder_active {
                RuntimeDecision::CloseCase
            } else {
                RuntimeDecision::StartRecovery
            }
        }
        RuntimeEvent::VerificationFailed => RuntimeDecision::StartRecovery,
        RuntimeEvent::MaintenanceTick => {
            if snapshot.owner_work_exists || snapshot.recovery_ladder_active {
                RuntimeDecision::AskEndpoint
            } else if snapshot.maintenance_eligible {
                RuntimeDecision::StartMaintenance
            } else {
                RuntimeDecision::AskEndpoint
            }
        }
        RuntimeEvent::CompletionRequested => {
            let admission = admit_tool(snapshot, "agent.done");
            if admission.admitted {
                RuntimeDecision::CloseCase
            } else {
                RuntimeDecision::BlockCompletion(admission)
            }
        }
        RuntimeEvent::ContextPressureRaised => RuntimeDecision::StartCompaction,
    }
}

fn decision_for_fault(snapshot: &RuntimeSnapshot, fault: RuntimeFault) -> RuntimeDecision {
    match fault {
        RuntimeFault::PayloadTooLarge => {
            RuntimeDecision::ContinueRecovery(admit_tool(snapshot, "fs.batch_write"))
        }
        RuntimeFault::Repeat => {
            RuntimeDecision::ContinueRecovery(admit_tool(snapshot, "graph.recover"))
        }
        RuntimeFault::CompletionRefused => {
            RuntimeDecision::BlockCompletion(admit_tool(snapshot, "agent.done"))
        }
        RuntimeFault::CompactionPressure => RuntimeDecision::StartCompaction,
        RuntimeFault::MaintenanceConflict => RuntimeDecision::AskEndpoint,
        RuntimeFault::Parse
        | RuntimeFault::Parameter
        | RuntimeFault::ToolRuntime
        | RuntimeFault::PolicyContradiction
        | RuntimeFault::VerificationMismatch => RuntimeDecision::StartRecovery,
    }
}
