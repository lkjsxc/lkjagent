use lkjagent_store::runtime_authority::{record_tool_admission, ToolAdmissionInput};
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::{DispatchState, EffectivePolicy};
use rusqlite::Connection;

use crate::error::RuntimeResult;

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
    let Some(value) = store_state::get(conn, key)? else {
        return Ok(None);
    };
    Ok(value.parse::<i64>().ok())
}
