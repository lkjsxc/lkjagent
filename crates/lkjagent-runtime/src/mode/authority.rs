use super::completion::{completion_policy_for, CompletionPolicy};
use super::decision::{endpoint_decision_for, EndpointDecision};
use super::input::TurnAuthorityInput;
use super::model::{ActiveMode, ActiveModePolicy};
use super::policy::policy_for_mode;
use super::render::render_mode_policy;
use super::select::select_active_mode;

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
    let prompt_card = prompt_card(&effective_policy, &valid_example);
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

fn prompt_card(policy: &ActiveModePolicy, valid_example: &str) -> String {
    format!(
        "Active Mode:\nmode={:?}\npolicy_layers={}\nallowed_tools={}\nblocked_tools={}\npreferred_next_action={}\ncompletion_condition={}\nvalid_example:\n{}",
        policy.mode,
        policy_layers(policy),
        join_or_none(&policy.allowed_tools),
        join_or_none(&policy.blocked_tools),
        policy.preferred_next_action,
        policy.completion_condition,
        valid_example,
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
        ActiveMode::OwnerTask | ActiveMode::Recovery => {
            "<act>\n<tool>graph.state</tool>\n</act>".to_string()
        }
        ActiveMode::Maintenance => {
            "<act>\n<tool>memory.find</tool>\n<query>maintenance</query>\n</act>".to_string()
        }
        ActiveMode::Compaction | ActiveMode::ClosedIdle => {
            "runtime action; no model act block".to_string()
        }
    }
}
