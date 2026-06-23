use super::admission::next_valid_tools;
use super::model::{RecoveryClass, RecoveryPlan, RuntimeFault, RuntimeSnapshot};
use super::recovery_route::{
    blocked_handoff_behavior, escalation_route, fault_class, recovery_route,
};
use lkjagent_tools::dispatch::registry_valid_example;

pub fn recovery_plan_for_fault(snapshot: &RuntimeSnapshot, fault: RuntimeFault) -> RecoveryPlan {
    let fault_class = fault_class(fault);
    let class = recovery_class(fault);
    let forced_tool = forced_tool(snapshot, class);
    RecoveryPlan {
        fault_class,
        recovery_class: class,
        recovery_route: recovery_route(fault_class).to_string(),
        previous_mission: snapshot.active_mission,
        retry_budget: retry_budget(class),
        allowed_observation_tools: observation_tools(class),
        allowed_repair_tools: repair_tools(class),
        forced_next_action: forced_action_text(class, &forced_tool),
        escalation_route: escalation_route(fault_class).to_string(),
        exact_valid_example: valid_example(&forced_tool),
        fallback_action: fallback_action(class),
        blocked_handoff_behavior: blocked_handoff_behavior(fault_class).to_string(),
        partial_handoff: allows_partial_handoff(class),
        forced_tool,
    }
}

fn recovery_class(fault: RuntimeFault) -> RecoveryClass {
    match fault {
        RuntimeFault::Parse => RecoveryClass::ParseFault,
        RuntimeFault::Parameter => RecoveryClass::ParameterFault,
        RuntimeFault::Schema => RecoveryClass::SchemaFault,
        RuntimeFault::PolicyContradiction => RecoveryClass::ToolAdmissionContradiction,
        RuntimeFault::Repeat => RecoveryClass::RepeatActionFault,
        RuntimeFault::PayloadTooLarge => RecoveryClass::PayloadOverflow,
        RuntimeFault::ArtifactAuditFailure => RecoveryClass::ArtifactAuditFailure,
        RuntimeFault::WeakArtifactContent => RecoveryClass::WeakArtifactContent,
        RuntimeFault::FalseCompletion | RuntimeFault::CompletionRefused => {
            RecoveryClass::FalseCompletion
        }
        RuntimeFault::VerificationMismatch => RecoveryClass::VerificationFailure,
        RuntimeFault::CompactionResumeGap | RuntimeFault::CompactionPressure => {
            RecoveryClass::CompactionResumeGap
        }
        RuntimeFault::MaintenanceConflict => RecoveryClass::MaintenancePreemption,
        RuntimeFault::ToolRuntime | RuntimeFault::EndpointFault | RuntimeFault::ContextInvalid => {
            RecoveryClass::EndpointFault
        }
        RuntimeFault::TurnBudgetExhausted => RecoveryClass::TurnBudgetExhaustion,
    }
}

fn forced_tool(snapshot: &RuntimeSnapshot, class: RecoveryClass) -> String {
    match class {
        RecoveryClass::ParseFault | RecoveryClass::ToolAdmissionContradiction => {
            "graph.recover".to_string()
        }
        RecoveryClass::ParameterFault | RecoveryClass::SchemaFault => snapshot
            .last_tool_attempt
            .clone()
            .filter(|tool| registry_valid_example(tool).is_some())
            .unwrap_or_else(|| "graph.recover".to_string()),
        RecoveryClass::RepeatActionFault => alternate_tool(snapshot),
        RecoveryClass::PayloadOverflow => "fs.batch_write".to_string(),
        RecoveryClass::ArtifactAuditFailure | RecoveryClass::WeakArtifactContent => {
            "artifact.next".to_string()
        }
        RecoveryClass::FalseCompletion => "artifact.audit".to_string(),
        RecoveryClass::VerificationFailure => "verify.xtask".to_string(),
        RecoveryClass::CompactionResumeGap => "runtime.compact".to_string(),
        RecoveryClass::MaintenancePreemption => "queue.list".to_string(),
        RecoveryClass::EndpointFault => "workspace.summary".to_string(),
        RecoveryClass::TurnBudgetExhaustion => "runtime.handoff".to_string(),
    }
}

fn alternate_tool(snapshot: &RuntimeSnapshot) -> String {
    let repeated = snapshot.last_tool_attempt.as_deref();
    next_valid_tools(snapshot)
        .into_iter()
        .find(|tool| Some(tool.as_str()) != repeated)
        .unwrap_or_else(|| "graph.recover".to_string())
}

fn retry_budget(class: RecoveryClass) -> u8 {
    match class {
        RecoveryClass::ParseFault | RecoveryClass::ParameterFault | RecoveryClass::SchemaFault => 1,
        RecoveryClass::RepeatActionFault => 0,
        RecoveryClass::PayloadOverflow
        | RecoveryClass::ArtifactAuditFailure
        | RecoveryClass::WeakArtifactContent
        | RecoveryClass::VerificationFailure => 2,
        RecoveryClass::TurnBudgetExhaustion => 0,
        _ => 1,
    }
}

fn observation_tools(class: RecoveryClass) -> Vec<String> {
    match class {
        RecoveryClass::ArtifactAuditFailure | RecoveryClass::WeakArtifactContent => {
            tools(&["artifact.audit", "doc.audit", "fs.read", "fs.tree"])
        }
        RecoveryClass::PayloadOverflow => tools(&["artifact.next", "fs.stat", "fs.tree"]),
        RecoveryClass::VerificationFailure => tools(&["verify.xtask", "verify.cargo"]),
        RecoveryClass::MaintenancePreemption => tools(&["queue.list"]),
        _ => tools(&["workspace.summary", "graph.state"]),
    }
}

fn repair_tools(class: RecoveryClass) -> Vec<String> {
    match class {
        RecoveryClass::PayloadOverflow => tools(&["artifact.next", "fs.batch_write"]),
        RecoveryClass::ArtifactAuditFailure | RecoveryClass::WeakArtifactContent => tools(&[
            "artifact.next",
            "artifact.apply",
            "fs.write",
            "fs.batch_write",
        ]),
        RecoveryClass::FalseCompletion => tools(&["artifact.audit", "doc.audit"]),
        RecoveryClass::ParameterFault | RecoveryClass::SchemaFault => tools(&["graph.recover"]),
        RecoveryClass::RepeatActionFault => tools(&["graph.transition", "artifact.next"]),
        RecoveryClass::TurnBudgetExhaustion => tools(&["runtime.handoff"]),
        _ => tools(&["graph.recover"]),
    }
}

fn forced_action_text(class: RecoveryClass, tool: &str) -> String {
    match class {
        RecoveryClass::ParseFault => "emit exactly one valid action block".to_string(),
        RecoveryClass::ParameterFault | RecoveryClass::SchemaFault => {
            format!("retry {tool} with the schema example")
        }
        RecoveryClass::RepeatActionFault => format!("change action shape to {tool}"),
        RecoveryClass::PayloadOverflow => {
            "switch from raw write to bounded batch write".to_string()
        }
        RecoveryClass::TurnBudgetExhaustion => "write a blocked partial handoff".to_string(),
        _ => format!("run {tool}"),
    }
}

fn fallback_action(class: RecoveryClass) -> String {
    match class {
        RecoveryClass::TurnBudgetExhaustion => "persist blocked handoff".to_string(),
        RecoveryClass::RepeatActionFault => "select smaller scope or blocked handoff".to_string(),
        RecoveryClass::PayloadOverflow => "fall back to one-file write or handoff".to_string(),
        _ => "route to blocked handoff when retries are exhausted".to_string(),
    }
}

fn allows_partial_handoff(class: RecoveryClass) -> bool {
    matches!(
        class,
        RecoveryClass::TurnBudgetExhaustion
            | RecoveryClass::EndpointFault
            | RecoveryClass::CompactionResumeGap
    )
}

fn valid_example(tool: &str) -> String {
    registry_valid_example(tool).unwrap_or_else(|| format!("runtime action: {tool}"))
}

fn tools(values: &[&str]) -> Vec<String> {
    values.iter().map(|tool| (*tool).to_string()).collect()
}
