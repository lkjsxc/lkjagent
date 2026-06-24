use super::completion::{completion_policy_for, CompletionPolicy};
use super::decision::{endpoint_decision_for, EndpointDecision};
use super::input::TurnAuthorityInput;
use super::mission::{select_runtime_mission, RuntimeMission};
use super::model::{ActiveMode, ActiveModePolicy, RuntimeSnapshot};
use super::policy::policy_for_mode;
use super::render::render_mode_policy;
use lkjagent_tools::dispatch::registry_valid_example;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnAuthority {
    pub mission: RuntimeMission,
    pub mode: ActiveMode,
    pub input: TurnAuthorityInput,
    pub snapshot: RuntimeSnapshot,
    pub effective_policy: ActiveModePolicy,
    pub completion_policy: CompletionPolicy,
    pub endpoint_decision: EndpointDecision,
    pub prompt_card: String,
    pub dispatch_card: String,
    pub valid_example: String,
}

pub fn decide_turn_authority(input: TurnAuthorityInput) -> TurnAuthority {
    let snapshot = runtime_snapshot_for_turn(&input);
    decide_turn_authority_for_snapshot(input, snapshot)
}

pub fn decide_turn_authority_for_snapshot(
    input: TurnAuthorityInput,
    mut snapshot: RuntimeSnapshot,
) -> TurnAuthority {
    let mission = select_runtime_mission(&snapshot);
    let mode = mission.active_mode();
    snapshot.active_mode = mode;
    let effective_policy = policy_for_mode(mode);
    let completion_policy = completion_policy_for(mode);
    let endpoint_decision = endpoint_decision_for(mode, &input);
    let valid_example = valid_example_for(mode, endpoint_decision);
    let prompt_card = prompt_card(
        mission,
        &effective_policy,
        &snapshot,
        endpoint_decision,
        &valid_example,
    );
    let dispatch_card = render_mode_policy(&effective_policy);

    TurnAuthority {
        mission,
        mode,
        input,
        snapshot,
        effective_policy,
        completion_policy,
        endpoint_decision,
        prompt_card,
        dispatch_card,
        valid_example,
    }
}

pub fn runtime_snapshot_for_turn(input: &TurnAuthorityInput) -> RuntimeSnapshot {
    let owner_work_exists = input.owner_work_exists();
    let recovery_ladder_active = input.recoverable_owner_case;
    let context_pressure_active = input.compaction_required;
    let maintenance_eligible =
        !owner_work_exists && (input.maintenance_due || input.maintenance_active);
    let mut snapshot = RuntimeSnapshot {
        active_mode: ActiveMode::ClosedIdle,
        case_id: input.case_id.map(|id| id.to_string()),
        graph_node: input.graph_node.clone(),
        graph_phase: input.graph_phase.clone(),
        owner_work_exists,
        recovery_ladder_active,
        context_pressure_active,
        maintenance_eligible,
        required_evidence: input.required_evidence.clone(),
        missing_evidence: input.missing_evidence.clone(),
        active_artifact: input.artifact_root.clone(),
        last_tool_attempt: None,
        latest_fault: None,
        repeated_action: false,
        external_owner_input_required: false,
    };
    snapshot.active_mode = select_runtime_mission(&snapshot).active_mode();
    snapshot
}

fn prompt_card(
    mission: RuntimeMission,
    policy: &ActiveModePolicy,
    snapshot: &RuntimeSnapshot,
    endpoint_decision: EndpointDecision,
    valid_example: &str,
) -> String {
    let mut card = format!(
        "Active Mode:\nmission={}\nmode={:?}\npolicy_layers={}\nallowed_tools={}\nblocked_tools={}\npreferred_next_action={}\ncompletion_condition={}\nvalid_example:\n{}",
        mission.as_str(),
        policy.mode,
        policy_layers(policy),
        join_or_none(&policy.allowed_tools),
        join_or_none(&policy.blocked_tools),
        policy.preferred_next_action,
        policy.completion_condition,
        valid_example,
    );
    if endpoint_decision == EndpointDecision::RuntimeCompact {
        card.push_str(&compaction_resume_card(snapshot, policy));
    }
    card
}

fn compaction_resume_card(snapshot: &RuntimeSnapshot, policy: &ActiveModePolicy) -> String {
    let active_case = if snapshot.owner_work_exists {
        "open-or-recoverable"
    } else {
        "none"
    };
    let recovery_ladder = if snapshot.recovery_ladder_active {
        "active"
    } else {
        "inactive"
    };
    let active_artifact = snapshot.active_artifact.as_deref().unwrap_or("none");
    format!(
        "\nCompaction Snapshot:\nactive_case={active_case}\nmissing_evidence={}\nactive_artifact={active_artifact}\nrecovery_ladder={recovery_ladder}\nnext_valid_action={}",
        join_strings_or_none(&snapshot.missing_evidence),
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

fn join_strings_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn valid_example_for(mode: ActiveMode, endpoint_decision: EndpointDecision) -> String {
    if endpoint_decision != EndpointDecision::CallModel {
        return "runtime action; no model action block".to_string();
    }
    match mode {
        ActiveMode::OwnerTask => rendered_registry_example("graph.state"),
        ActiveMode::Recovery => rendered_registry_example("graph.recover"),
        ActiveMode::Maintenance => rendered_registry_example("memory.find"),
        ActiveMode::Compaction | ActiveMode::ClosedIdle => {
            "runtime action; no model action block".to_string()
        }
    }
}

fn rendered_registry_example(tool: &str) -> String {
    registry_valid_example(tool)
        .unwrap_or_else(|| format!("<action>\n<tool>{tool}</tool>\n</action>"))
}
