use super::admission::admit_tool;
use super::model::{RuntimeDecision, RuntimeEvent, RuntimeFault, RuntimeSnapshot};
use super::recovery::recovery_plan_for_fault;

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
                let fault = refusal_fault(snapshot, &tool);
                RuntimeDecision::ContinueRecovery {
                    plan: recovery_plan_for_fault(snapshot, fault),
                    admission,
                }
            } else {
                RuntimeDecision::RefuseAction(admission)
            }
        }
        RuntimeEvent::EndpointActionParseFailed => {
            RuntimeDecision::StartRecovery(recovery_plan_for_fault(snapshot, RuntimeFault::Parse))
        }
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
                RuntimeDecision::StartRecovery(recovery_plan_for_fault(
                    snapshot,
                    RuntimeFault::VerificationMismatch,
                ))
            }
        }
        RuntimeEvent::VerificationFailed => RuntimeDecision::StartRecovery(
            recovery_plan_for_fault(snapshot, RuntimeFault::VerificationMismatch),
        ),
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
        RuntimeEvent::TurnBudgetCheckpoint => checkpoint_decision(snapshot),
        RuntimeEvent::TurnBudgetExhausted => RuntimeDecision::StartRecovery(
            recovery_plan_for_fault(snapshot, RuntimeFault::TurnBudgetExhausted),
        ),
        RuntimeEvent::ContextPressureRaised => RuntimeDecision::StartCompaction,
    }
}

fn checkpoint_decision(snapshot: &RuntimeSnapshot) -> RuntimeDecision {
    if snapshot.recovery_ladder_active {
        return RuntimeDecision::StartRecovery(recovery_plan_for_fault(
            snapshot,
            checkpoint_fault(snapshot),
        ));
    }
    let admission = admit_tool(snapshot, "agent.done");
    if admission.admitted {
        RuntimeDecision::CloseCase
    } else {
        RuntimeDecision::AskEndpoint
    }
}

fn checkpoint_fault(snapshot: &RuntimeSnapshot) -> RuntimeFault {
    if snapshot.repeated_action {
        RuntimeFault::Repeat
    } else if snapshot.last_tool_attempt.is_some() {
        RuntimeFault::ToolRuntime
    } else {
        RuntimeFault::Parse
    }
}

fn decision_for_fault(snapshot: &RuntimeSnapshot, fault: RuntimeFault) -> RuntimeDecision {
    match fault {
        RuntimeFault::CompactionPressure => RuntimeDecision::StartCompaction,
        RuntimeFault::MaintenanceConflict => RuntimeDecision::AskEndpoint,
        RuntimeFault::CompletionRefused | RuntimeFault::FalseCompletion => {
            RuntimeDecision::BlockCompletion(admit_tool(snapshot, "agent.done"))
        }
        other => {
            let plan = recovery_plan_for_fault(snapshot, other);
            let admission = admit_tool(snapshot, &plan.forced_tool);
            RuntimeDecision::ContinueRecovery { plan, admission }
        }
    }
}

fn refusal_fault(snapshot: &RuntimeSnapshot, tool: &str) -> RuntimeFault {
    if snapshot.repeated_action
        && snapshot
            .last_tool_attempt
            .as_deref()
            .is_some_and(|last| last == tool)
    {
        RuntimeFault::Repeat
    } else {
        RuntimeFault::PolicyContradiction
    }
}
