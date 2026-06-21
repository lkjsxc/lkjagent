use super::completion::{completion_policy_for, CompletionPolicy};
use super::decision::{endpoint_decision_for, EndpointDecision};
use super::input::TurnAuthorityInput;
use super::model::{ActiveMode, ActiveModePolicy};
use super::policy::policy_for_mode;
use super::render::render_mode_policy;
use super::select::select_active_mode;
use lkjagent_tools::dispatch::registry_valid_example;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnAuthority {
    pub mode: ActiveMode,
    pub input: TurnAuthorityInput,
    pub effective_policy: ActiveModePolicy,
    pub completion_policy: CompletionPolicy,
    pub endpoint_decision: EndpointDecision,
    pub prompt_card: String,
    pub dispatch_card: String,
    pub valid_example: String,
}

pub fn decide_turn_authority(input: TurnAuthorityInput) -> TurnAuthority {
    let mode = select_active_mode(input.mode_input());
    let effective_policy = policy_for_mode(mode);
    let completion_policy = completion_policy_for(mode);
    let endpoint_decision = endpoint_decision_for(mode, input);
    let valid_example = valid_example_for(mode, endpoint_decision);
    let prompt_card = prompt_card(&effective_policy, input, endpoint_decision, &valid_example);
    let dispatch_card = render_mode_policy(&effective_policy);

    TurnAuthority {
        mode,
        input,
        effective_policy,
        completion_policy,
        endpoint_decision,
        prompt_card,
        dispatch_card,
        valid_example,
    }
}

fn prompt_card(
    policy: &ActiveModePolicy,
    input: TurnAuthorityInput,
    endpoint_decision: EndpointDecision,
    valid_example: &str,
) -> String {
    let mut card = format!(
        "Active Mode:\nmode={:?}\npolicy_layers={}\nallowed_tools={}\nblocked_tools={}\npreferred_next_action={}\ncompletion_condition={}\nvalid_example:\n{}",
        policy.mode,
        policy_layers(policy),
        join_or_none(&policy.allowed_tools),
        join_or_none(&policy.blocked_tools),
        policy.preferred_next_action,
        policy.completion_condition,
        valid_example,
    );
    if endpoint_decision == EndpointDecision::RuntimeCompact {
        card.push_str(&compaction_resume_card(input, policy));
    }
    card
}

fn compaction_resume_card(input: TurnAuthorityInput, policy: &ActiveModePolicy) -> String {
    let active_case = if input.owner_work_exists() {
        "open-or-recoverable"
    } else {
        "none"
    };
    let recovery_ladder = if input.recoverable_owner_case {
        "active"
    } else {
        "inactive"
    };
    format!(
        "\nCompaction Snapshot:\nactive_case={active_case}\nmissing_evidence=artifact-readiness,verification,recovery-resolution\nactive_artifact=pending-or-unknown\nrecovery_ladder={recovery_ladder}\nnext_valid_action={}",
        policy.preferred_next_action
    )
}

fn policy_layers(policy: &ActiveModePolicy) -> String {
    let mut layers = Vec::new();
    if policy.graph_policy_applies {
        layers.push("graph");
    }
    if policy.maintenance_policy_applies {
        layers.push("maintenance");
    }
    if policy.compaction_policy_applies {
        layers.push("compaction");
    }
    if layers.is_empty() {
        "none".to_string()
    } else {
        layers.join(",")
    }
}

fn join_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn valid_example_for(mode: ActiveMode, endpoint_decision: EndpointDecision) -> String {
    if endpoint_decision != EndpointDecision::CallModel {
        return "runtime action; no model act block".to_string();
    }
    match mode {
        ActiveMode::OwnerTask => rendered_registry_example("graph.state"),
        ActiveMode::Recovery => rendered_registry_example("graph.recover"),
        ActiveMode::Maintenance => rendered_registry_example("memory.find"),
        ActiveMode::Compaction | ActiveMode::ClosedIdle => {
            "runtime action; no model act block".to_string()
        }
    }
}

fn rendered_registry_example(tool: &str) -> String {
    registry_valid_example(tool).unwrap_or_else(|| format!("<act>\n<tool>{tool}</tool>\n</act>"))
}
