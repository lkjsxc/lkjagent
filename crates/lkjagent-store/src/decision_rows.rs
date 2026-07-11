use lkjagent_core::runtime_decision::RuntimeDecision;
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};
use crate::row_support::{fingerprint_error, json_string, json_value};

pub struct ProviderExchangeRow {
    pub id: String,
    pub case_id: String,
    pub decision_id: String,
    pub exchange_ref: String,
    pub outcome_json: String,
    pub context_frame_fingerprint: String,
    pub timeout_seconds: Option<u64>,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[rustfmt::skip]
pub fn insert_provider_exchange(conn: &Connection, row: &ProviderExchangeRow) -> StoreResult<()> {
    conn.execute("INSERT OR REPLACE INTO provider_exchanges
         (id, case_id, decision_id, exchange_ref, outcome_json,
          context_frame_fingerprint, timeout_seconds, started_at, finished_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", params![row.id, row.case_id,
        row.decision_id, row.exchange_ref, row.outcome_json, row.context_frame_fingerprint,
        row.timeout_seconds.map(|value| value as i64), row.started_at, row.finished_at])?;
    Ok(())
}

pub fn next_decision_id(conn: &Connection, case_id: &str) -> StoreResult<String> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM runtime_decisions WHERE case_id = ?1",
        [case_id],
        |row| row.get(0),
    )?;
    Ok(format!("case-{case_id}-decision-{:04}", count + 1))
}

pub fn insert_runtime_decision(
    conn: &Connection,
    decision: &RuntimeDecision,
    status: &str,
    selected_at: &str,
) -> StoreResult<()> {
    let tool_view_fingerprint = decision
        .tool_view_fingerprint()
        .map_err(fingerprint_error)?;
    let evidence_json = json_string(&decision.evidence_requirements)?;
    let decision_json = json_string(decision)?;
    conn.execute(
        "INSERT INTO runtime_decisions
         (id, case_id, operation_key, status, snapshot_fingerprint,
          state_vector_fingerprint, context_frame_fingerprint,
          tool_view_fingerprint, expected_envelope, model_budget_tokens,
          evidence_requirements_json, recovery_policy, decision_json,
          selected_at, settled_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                 ?14, NULL)",
        params![
            decision.id,
            decision.case_id,
            decision.operation.0,
            status,
            decision.snapshot_fingerprint,
            decision.state_vector_fingerprint,
            decision.context_frame_fingerprint,
            tool_view_fingerprint,
            format!("{:?}", decision.expected_envelope),
            decision.model_budget_tokens,
            evidence_json,
            decision.recovery_policy,
            decision_json,
            selected_at,
        ],
    )?;
    Ok(())
}

pub fn unfinished_decisions(conn: &Connection, case_id: &str) -> StoreResult<Vec<RuntimeDecision>> {
    let mut statement = conn.prepare(
        "SELECT decision_json FROM runtime_decisions
         WHERE case_id = ?1 AND status = 'pending'
         ORDER BY selected_at, id",
    )?;
    let rows = statement.query_map([case_id], |row| row.get::<_, String>(0))?;
    let mut decisions = Vec::new();
    for row in rows {
        let decision: RuntimeDecision = json_value(&row?)?;
        if !decision.tool_view.has_current_constraints() {
            return Err(StoreError::InvalidState(
                "pending decision tool constraints are stale".to_string(),
            ));
        }
        decisions.push(decision);
    }
    Ok(decisions)
}

pub fn settle_decision(
    conn: &Connection,
    decision_id: &str,
    status: &str,
    settled_at: &str,
) -> StoreResult<usize> {
    let changed = conn.execute(
        "UPDATE runtime_decisions SET status = ?1, settled_at = ?2 WHERE id = ?3",
        params![status, settled_at, decision_id],
    )?;
    Ok(changed)
}
