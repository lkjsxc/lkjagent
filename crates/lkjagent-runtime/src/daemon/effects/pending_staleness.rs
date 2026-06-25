use crate::mode::{EndpointDecision, TurnAuthority};
use crate::task::PendingAction;

pub fn persisted_action_refusal(
    pending: &PendingAction,
    current: &TurnAuthority,
    tool: &str,
) -> Option<String> {
    let mut changed_fields = Vec::new();
    if changed(
        pending.prompt_frame_id.as_deref(),
        current.input.prompt_frame_id.as_deref(),
    ) {
        changed_fields.push("prompt_frame_id");
    }
    if changed(
        pending.staleness_fingerprint.as_deref(),
        current.input.staleness_fingerprint.as_deref(),
    ) {
        changed_fields.push("staleness_fingerprint");
    }
    if changed_fields.is_empty() {
        return None;
    }
    Some(format!(
        "stale model action refused\nadmission=refused\nreason=stale_decision\nprevious_decision={}\nactive_mode={:?}\nfailed_tool={tool}\nfailed_gate=stale-persisted-action\nchanged_fields={}\nadmitted_tools={}\nnext_executable_action={}\ndetail=persisted prompt authority no longer matches current runtime authority",
        pending.authority_decision_id.as_deref().unwrap_or("unknown"),
        current.mode,
        changed_fields.join(","),
        join_or_none(&current.effective_policy.allowed_tools),
        current.valid_example
    ))
}

pub fn stale_action_refusal(
    cached: Option<&TurnAuthority>,
    current: &TurnAuthority,
    tool: &str,
) -> Option<String> {
    let cached = cached?;
    let mut changed_fields = changed_fields(cached, current);
    let runtime_only = matches!(
        current.endpoint_decision,
        EndpointDecision::RuntimeCompact | EndpointDecision::ClosedIdle
    );
    if runtime_only && !changed_fields.contains(&"endpoint_decision") {
        changed_fields.push("endpoint_decision");
    }
    if changed_fields.is_empty() {
        return None;
    }
    Some(format!(
        "stale model action refused\nadmission=refused\nreason=stale_decision\nprevious_mode={:?}\nactive_mode={:?}\nfailed_tool={tool}\nfailed_gate=stale-turn-authority\nchanged_fields={}\nadmitted_tools={}\nnext_executable_action={}\ndetail=current runtime authority preempts the cached model action",
        cached.mode,
        current.mode,
        changed_fields.join(","),
        join_or_none(&current.effective_policy.allowed_tools),
        current.valid_example
    ))
}

fn changed_fields(cached: &TurnAuthority, current: &TurnAuthority) -> Vec<&'static str> {
    let mut fields = Vec::new();
    push_if(
        &mut fields,
        "pending_owner_rows",
        cached.input.pending_owner_rows != current.input.pending_owner_rows,
    );
    push_if(
        &mut fields,
        "active_case",
        cached.input.active_owner_case != current.input.active_owner_case,
    );
    push_if(
        &mut fields,
        "compaction_pressure",
        cached.input.compaction_required != current.input.compaction_required,
    );
    push_if(
        &mut fields,
        "maintenance_state",
        maintenance_state(cached) != maintenance_state(current),
    );
    push_if(
        &mut fields,
        "endpoint_retry",
        cached.input.endpoint_retry_pending != current.input.endpoint_retry_pending,
    );
    push_if(
        &mut fields,
        "case_id",
        cached.input.case_id != current.input.case_id,
    );
    push_if(
        &mut fields,
        "graph_node",
        cached.input.graph_node != current.input.graph_node,
    );
    push_if(
        &mut fields,
        "graph_phase",
        cached.input.graph_phase != current.input.graph_phase,
    );
    push_if(
        &mut fields,
        "artifact_root",
        cached.input.artifact_root != current.input.artifact_root,
    );
    push_if(
        &mut fields,
        "required_evidence",
        cached.input.required_evidence != current.input.required_evidence,
    );
    push_if(
        &mut fields,
        "missing_evidence",
        cached.input.missing_evidence != current.input.missing_evidence,
    );
    push_if(
        &mut fields,
        "prompt_frame_id",
        cached.input.prompt_frame_id != current.input.prompt_frame_id,
    );
    fields
}

fn changed(previous: Option<&str>, current: Option<&str>) -> bool {
    matches!((previous, current), (Some(old), Some(new)) if old != new)
}

fn maintenance_state(authority: &TurnAuthority) -> (bool, bool) {
    let due = if authority.input.owner_work_exists() {
        false
    } else {
        authority.input.maintenance_due
    };
    (due, authority.input.maintenance_active)
}

fn push_if(fields: &mut Vec<&'static str>, field: &'static str, changed: bool) {
    if changed {
        fields.push(field);
    }
}

fn join_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
