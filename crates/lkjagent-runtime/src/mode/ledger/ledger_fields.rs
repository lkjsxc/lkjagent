use super::admission::next_valid_tools;
use super::model::{RecoveryPlan, RuntimeDecision, RuntimeSnapshot, ToolAdmission};

pub(super) struct DecisionFields {
    pub(super) admitted_tools: Vec<String>,
    pub(super) recommended_next_actions: Vec<String>,
    pub(super) forced_next_action: String,
    pub(super) exact_valid_example: Option<String>,
}

pub(super) fn decision_fields(
    snapshot: &RuntimeSnapshot,
    decision: &RuntimeDecision,
) -> DecisionFields {
    match decision {
        RuntimeDecision::ExecuteTool(a)
        | RuntimeDecision::RefuseAction(a)
        | RuntimeDecision::BlockCompletion(a) => admission_fields(a),
        RuntimeDecision::ContinueRecovery { plan, admission } => recovery_fields(plan, admission),
        RuntimeDecision::StartRecovery(plan) => plan_fields(plan),
        RuntimeDecision::StartCompaction => {
            runtime_fields("runtime.compact", "runtime-owned compaction snapshot")
        }
        RuntimeDecision::StartMaintenance => {
            tools_fields(snapshot, "bounded maintenance effect or no-op close")
        }
        RuntimeDecision::StartVerification => {
            runtime_fields("verify.xtask", "run admitted verification")
        }
        RuntimeDecision::CloseCase => {
            runtime_fields("agent.done", "close with central completion gate")
        }
        RuntimeDecision::AskEndpoint => {
            tools_fields(snapshot, "call endpoint for next model intent")
        }
    }
}

pub(super) fn blocked_tools(policy_blocked: &[&str], decision: &RuntimeDecision) -> Vec<String> {
    let mut blocked = policy_blocked
        .iter()
        .map(|tool| (*tool).to_string())
        .collect::<Vec<_>>();
    if matches!(decision, RuntimeDecision::BlockCompletion(_)) {
        blocked.push("agent.done".to_string());
    }
    blocked.sort();
    blocked.dedup();
    blocked
}

pub(super) fn completion_refusal(decision: &RuntimeDecision) -> Option<String> {
    if let RuntimeDecision::BlockCompletion(admission) = decision {
        Some(admission.reason.clone())
    } else {
        None
    }
}

pub(super) fn recovery_route(decision: &RuntimeDecision) -> Option<String> {
    match decision {
        RuntimeDecision::StartRecovery(plan) | RuntimeDecision::ContinueRecovery { plan, .. } => {
            Some(plan.recovery_route.clone())
        }
        _ => None,
    }
}

fn admission_fields(admission: &ToolAdmission) -> DecisionFields {
    DecisionFields {
        admitted_tools: admission.next_valid_tools.clone(),
        recommended_next_actions: admission.next_valid_tools.clone(),
        forced_next_action: admission.reason.clone(),
        exact_valid_example: admission.exact_valid_example.clone(),
    }
}

fn recovery_fields(plan: &RecoveryPlan, admission: &ToolAdmission) -> DecisionFields {
    let mut tools = admission.next_valid_tools.clone();
    for tool in plan
        .allowed_repair_tools
        .iter()
        .chain(plan.allowed_observation_tools.iter())
    {
        if !tools.iter().any(|existing| existing == tool) {
            tools.push(tool.clone());
        }
    }
    DecisionFields {
        admitted_tools: tools.clone(),
        recommended_next_actions: tools,
        forced_next_action: plan.forced_next_action.clone(),
        exact_valid_example: Some(plan.exact_valid_example.clone()),
    }
}

fn plan_fields(plan: &RecoveryPlan) -> DecisionFields {
    let mut tools = vec![plan.forced_tool.clone()];
    tools.extend(plan.allowed_repair_tools.clone());
    tools.extend(plan.allowed_observation_tools.clone());
    tools.sort();
    tools.dedup();
    DecisionFields {
        admitted_tools: tools.clone(),
        recommended_next_actions: tools,
        forced_next_action: plan.forced_next_action.clone(),
        exact_valid_example: Some(plan.exact_valid_example.clone()),
    }
}

fn runtime_fields(tool: &str, action: &str) -> DecisionFields {
    DecisionFields {
        admitted_tools: vec![tool.to_string()],
        recommended_next_actions: vec![tool.to_string()],
        forced_next_action: action.to_string(),
        exact_valid_example: None,
    }
}

fn tools_fields(snapshot: &RuntimeSnapshot, action: &str) -> DecisionFields {
    let tools = next_valid_tools(snapshot);
    DecisionFields {
        admitted_tools: tools.clone(),
        recommended_next_actions: tools,
        forced_next_action: action.to_string(),
        exact_valid_example: None,
    }
}
