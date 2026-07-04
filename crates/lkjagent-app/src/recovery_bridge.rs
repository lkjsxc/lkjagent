use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_store::decision_rows::settle_decision;
use rusqlite::Connection;

pub fn recover_or_reuse(
    conn: &Connection,
    unfinished: &[RuntimeDecision],
    now: &str,
) -> Result<Option<RuntimeDecision>, String> {
    for decision in unfinished {
        if has_external_evidence(conn, &decision.id)? {
            settle_decision(conn, &decision.id, "recovered", now)
                .map_err(|error| error.to_string())?;
        } else {
            return Ok(Some(decision.clone()));
        }
    }
    Ok(None)
}

fn has_external_evidence(conn: &Connection, decision_id: &str) -> Result<bool, String> {
    let sql = "SELECT
        (SELECT COUNT(*) FROM provider_exchanges WHERE decision_id = ?1) +
        (SELECT COUNT(*) FROM observations WHERE decision_id = ?1) +
        (SELECT COUNT(*) FROM tool_admissions WHERE decision_id = ?1)";
    let count: i64 = conn
        .query_row(sql, [decision_id], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    Ok(count > 0)
}
