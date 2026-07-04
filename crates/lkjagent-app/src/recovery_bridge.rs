use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{StateCell, StateKey, StateStatus};
use lkjagent_store::decision_rows::settle_decision;
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::Connection;

pub fn recover_or_reuse(
    conn: &Connection,
    unfinished: &[RuntimeDecision],
    now: &str,
) -> Result<Option<RuntimeDecision>, String> {
    for decision in unfinished {
        let evidence_count = external_evidence_count(conn, &decision.id)?;
        if evidence_count > 0 {
            settle_decision(conn, &decision.id, "recovered", now)
                .map_err(|error| error.to_string())?;
            record_recovery_cell(conn, decision, evidence_count, now)?;
        } else {
            return Ok(Some(decision.clone()));
        }
    }
    Ok(None)
}

fn record_recovery_cell(
    conn: &Connection,
    decision: &RuntimeDecision,
    evidence_count: i64,
    now: &str,
) -> Result<(), String> {
    let event_id =
        next_event_id(conn, &decision.case_id, "recovery").map_err(|error| error.to_string())?;
    let key = StateKey::new("recovery", format!("recovered/{}", decision.id))
        .map_err(|error| error.message)?;
    let mut cell = StateCell::active(key, event_id.clone());
    cell.status = StateStatus::Resolved;
    cell.payload_schema = "recovery.report.v1".to_string();
    cell.payload_json = serde_json::json!({
        "decision_id": decision.id,
        "evidence_count": evidence_count,
    })
    .to_string();
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    let event = RuntimeEvent {
        id: event_id,
        case_id: decision.case_id.clone(),
        kind: "recovery.report".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "recovery-bridge".to_string(),
        created_at: now.to_string(),
        decision_id: Some(decision.id.clone()),
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

fn external_evidence_count(conn: &Connection, decision_id: &str) -> Result<i64, String> {
    let sql = "SELECT
        (SELECT COUNT(*) FROM provider_exchanges WHERE decision_id = ?1) +
        (SELECT COUNT(*) FROM observations WHERE decision_id = ?1) +
        (SELECT COUNT(*) FROM tool_admissions WHERE decision_id = ?1)";
    let count: i64 = conn
        .query_row(sql, [decision_id], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    Ok(count)
}
