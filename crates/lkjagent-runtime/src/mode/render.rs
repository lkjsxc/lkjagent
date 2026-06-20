use super::model::ActiveModePolicy;

pub fn render_mode_policy(policy: &ActiveModePolicy) -> String {
    format!(
        "active_mode={:?}\nallowed_tools={}\nblocked_tools={}\npreferred_next_action={}\ncompletion_condition={}\npolicy_layers={}",
        policy.mode,
        join_or_none(&policy.allowed_tools),
        join_or_none(&policy.blocked_tools),
        policy.preferred_next_action,
        policy.completion_condition,
        policy_layers(policy),
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
