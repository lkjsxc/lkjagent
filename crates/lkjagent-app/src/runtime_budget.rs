use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_eligibility::{utc_millis, RuntimeBudget};
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_store::decision_rows::settle_decision;
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::{params, Connection};

pub fn enforce(
    conn: &Connection,
    case_id: &str,
    now: &str,
    pending: &[RuntimeDecision],
) -> Result<bool, String> {
    let used = usage(conn, case_id, now)?;
    let limits = limits(conn)?;
    let reached = used.reached(&limits);
    if reached.is_empty() {
        return Ok(false);
    }
    block(conn, case_id, now, &used, &limits, &reached, pending)?;
    Ok(true)
}

pub fn usage(conn: &Connection, case_id: &str, now: &str) -> Result<RuntimeBudget, String> {
    Ok(RuntimeBudget {
        tokens: tokens(conn, case_id)?,
        active_milliseconds: active(conn, case_id, now)?,
        effects: count(
            conn,
            "SELECT COUNT(*) FROM observations WHERE case_id = ?1",
            case_id,
        )?,
        recovery_cost: count(
            conn,
            "SELECT COUNT(*) FROM state_cells WHERE case_id = ?1
            AND payload_schema IN ('recovery.failure','recovery.no-progress')",
            case_id,
        )?,
    })
}

fn limits(conn: &Connection) -> Result<RuntimeBudget, String> {
    Ok(RuntimeBudget {
        tokens: config(conn, "runtime.case_token_budget")?,
        active_milliseconds: config(conn, "runtime.case_active_milliseconds")?,
        effects: config(conn, "runtime.case_effect_budget")?,
        recovery_cost: config(conn, "runtime.case_recovery_budget")?,
    })
}

fn tokens(conn: &Connection, case_id: &str) -> Result<u64, String> {
    let Ok(task_id) = case_id.parse::<i64>() else {
        return Ok(0);
    };
    let value: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(COALESCE(input_total_tokens,
        prompt_tokens, 0) + COALESCE(output_tokens, completion_tokens, 0)), 0)
        FROM token_usage WHERE task_id = ?1",
            [task_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    u64::try_from(value).map_err(|error| error.to_string())
}

fn active(conn: &Connection, case_id: &str, now: &str) -> Result<u64, String> {
    let mut statement = conn
        .prepare(
            "SELECT selected_at, COALESCE(settled_at, ?2)
        FROM runtime_decisions WHERE case_id = ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![case_id, now], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| error.to_string())?;
    let mut total = 0_u64;
    for row in rows {
        let (start, end) = row.map_err(|error| error.to_string())?;
        if let (Some(start), Some(end)) = (utc_millis(&start), utc_millis(&end)) {
            total = total.saturating_add(end.saturating_sub(start));
        }
    }
    Ok(total)
}

fn count(conn: &Connection, sql: &str, case_id: &str) -> Result<u64, String> {
    let value: i64 = conn
        .query_row(sql, [case_id], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    u64::try_from(value).map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn block(conn: &Connection, case_id: &str, now: &str, used: &RuntimeBudget,
    limits: &RuntimeBudget, reached: &[&str], pending: &[RuntimeDecision]) -> Result<(), String> {
    let mut statement = conn.prepare("SELECT key_label FROM state_cells WHERE case_id = ?1
        AND status = 'Active' AND key_label <> 'completion:blocked'").map_err(|error| error.to_string())?;
    let labels = statement.query_map([case_id], |row| row.get::<_, String>(0)).map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?; drop(statement);
    let tx = conn.unchecked_transaction().map_err(|error| error.to_string())?;
    for label in labels {
        let id = next_event_id(&tx, case_id, "budget-source").map_err(|error| error.to_string())?;
        let key = StateKey::from_label(&label).map_err(|error| error.message)?;
        let event = RuntimeEvent { id, case_id: case_id.to_string(), kind: "state.cell.suppress".to_string(),
            payload: RuntimeEventPayload::SuppressCell { key, reason: "runtime budget exhausted".to_string() },
            source: "runtime-budget".to_string(), created_at: now.to_string(), decision_id: None };
        append_and_apply_event(&tx, &event).map_err(|error| error.to_string())?;
    }
    let id = next_event_id(&tx, case_id, "budget-block").map_err(|error| error.to_string())?;
    let key = StateKey::new("completion", "blocked").map_err(|error| error.message)?;
    let mut cell = StateCell::active(key, id.clone()); cell.payload_schema = "completion.blocked".to_string();
    cell.payload_json = serde_json::json!({ "reason": "runtime budget exhausted", "reached": reached,
        "used": used, "limits": limits, "owner_action": "raise the named case budget or narrow the objective" }).to_string();
    let fingerprint = stable_fingerprint(&(used, limits)).map_err(|error| error.message)?;
    cell.evidence_refs = vec![EvidenceRef { source_type: "runtime_budget".to_string(),
        source_id: case_id.to_string(), fingerprint }]; cell.created_at = now.to_string(); cell.updated_at = now.to_string();
    let event = RuntimeEvent { id, case_id: case_id.to_string(), kind: "completion.blocked".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)), source: "runtime-budget".to_string(),
        created_at: now.to_string(), decision_id: None };
    append_and_apply_event(&tx, &event).map_err(|error| error.to_string())?;
    for decision in pending {
        settle_decision(&tx, &decision.id, "budget-exhausted", now).map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())
}

fn config(conn: &Connection, key: &str) -> Result<u64, String> {
    let value: String = conn
        .query_row("SELECT value FROM config WHERE key = ?1", [key], |row| {
            row.get(0)
        })
        .map_err(|error| error.to_string())?;
    value
        .parse()
        .map_err(|error: std::num::ParseIntError| error.to_string())
}
