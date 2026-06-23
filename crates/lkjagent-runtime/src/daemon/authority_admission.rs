use lkjagent_store::runtime_authority::{record_tool_admission, ToolAdmissionInput};
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::{AuthorityAdmissionView, DispatchState, EffectivePolicy};
use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::mode::TurnAuthority;

pub(super) fn install_authority_view(
    conn: &Connection,
    state: &mut DispatchState,
    authority: &TurnAuthority,
) -> RuntimeResult<()> {
    let Some(policy) = state.effective_policy.clone() else {
        state.authority_view = None;
        return Ok(());
    };
    state.authority_view = Some(AuthorityAdmissionView {
        decision_id: text_state(conn, "authority decision id")?.unwrap_or_else(|| "0".to_string()),
        case_id: text_state(conn, "authority case id")?.unwrap_or_else(|| "0".to_string()),
        authority_fingerprint: text_state(conn, "authority fingerprint")?.unwrap_or_default(),
        active_mission: authority.mission.as_str().to_string(),
        active_node: active_node(state),
        admitted_tools: policy.allowed_tools,
        blocked_tools: policy.blocked_tools,
        shell_allowed: policy.shell_allowed,
        completion_allowed: policy.completion_allowed,
        missing_evidence: state.graph_missing.clone(),
        recovery_route: optional_state(conn, "authority recovery route")?,
        exact_valid_example: authority.valid_example.clone(),
    });
    Ok(())
}

pub(super) fn record_authority_admission(
    conn: &Connection,
    now: &str,
    state: &DispatchState,
    requested_tool: &str,
    exact_valid_example: &str,
) -> RuntimeResult<()> {
    let Some(policy) = state.effective_policy.as_ref() else {
        return record(
            conn,
            now,
            requested_tool,
            false,
            "authority policy missing",
            None,
        );
    };
    let admitted = admitted(policy, requested_tool);
    let reason = reason(policy, requested_tool, admitted);
    record(
        conn,
        now,
        requested_tool,
        admitted,
        &reason,
        Some(exact_valid_example),
    )
}

pub(super) fn record_authority_refusal(
    conn: &Connection,
    now: &str,
    requested_tool: &str,
    refusal_reason: &str,
    exact_valid_example: &str,
) -> RuntimeResult<()> {
    record(
        conn,
        now,
        requested_tool,
        false,
        refusal_reason,
        Some(exact_valid_example),
    )
}

fn record(
    conn: &Connection,
    now: &str,
    requested_tool: &str,
    admitted: bool,
    refusal_reason: &str,
    exact_valid_example: Option<&str>,
) -> RuntimeResult<()> {
    let Some(decision_id) = numeric_state(conn, "authority decision id")? else {
        return Ok(());
    };
    let case_id = numeric_state(conn, "authority case id")?.unwrap_or(0);
    record_tool_admission(
        conn,
        &ToolAdmissionInput {
            decision_id,
            case_id,
            requested_tool,
            admitted,
            refusal_reason,
            exact_valid_example,
            created_at: now,
        },
    )?;
    Ok(())
}

fn admitted(policy: &EffectivePolicy, tool: &str) -> bool {
    if tool == "agent.done" {
        return policy.completion_allowed;
    }
    if tool == "shell.run" && !policy.shell_allowed {
        return false;
    }
    policy.allowed_tools.iter().any(|allowed| allowed == tool)
        && !policy.blocked_tools.iter().any(|blocked| blocked == tool)
}

fn reason(policy: &EffectivePolicy, tool: &str, admitted: bool) -> String {
    if admitted {
        return format!("{tool} admitted by authority decision");
    }
    if tool == "agent.done" && !policy.completion_allowed {
        return "completion not admitted by authority decision".to_string();
    }
    if tool == "shell.run" && !policy.shell_allowed {
        return "shell not admitted by authority decision".to_string();
    }
    policy.reason.clone()
}

fn numeric_state(conn: &Connection, key: &str) -> RuntimeResult<Option<i64>> {
    let Some(value) = text_state(conn, key)? else {
        return Ok(None);
    };
    Ok(value.parse::<i64>().ok())
}

fn text_state(conn: &Connection, key: &str) -> RuntimeResult<Option<String>> {
    Ok(store_state::get(conn, key)?)
}

fn optional_state(conn: &Connection, key: &str) -> RuntimeResult<Option<String>> {
    Ok(text_state(conn, key)?.filter(|value| value != "none"))
}

fn active_node(state: &DispatchState) -> String {
    state
        .graph_policy
        .as_ref()
        .map_or_else(|| "none".to_string(), |policy| policy.active_node.clone())
}
