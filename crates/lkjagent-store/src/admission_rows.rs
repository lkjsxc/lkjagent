use lkjagent_core::runtime_admission::ToolAdmission;
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};
use crate::row_json::json_string;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedEffect {
    pub admission_id: String,
    pub journal_id: String,
    pub command_ordinal: i64,
    pub target_path: Option<String>,
    pub prior_fingerprint: String,
    pub intended_fingerprint: String,
    pub effect_name: String,
}

pub struct EffectPreparation<'a> {
    pub id: &'a str,
    pub case_id: &'a str,
    pub admission: &'a ToolAdmission,
    pub parsed_action_json: &'a str,
    pub journal_id: &'a str,
    pub idempotency_key: &'a str,
    pub command_ordinal: i64,
    pub target_path: Option<&'a str>,
    pub prior_fingerprint: &'a str,
    pub intended_fingerprint: &'a str,
    pub created_at: &'a str,
}

pub fn insert_admission_and_prepare(
    conn: &Connection,
    preparation: &EffectPreparation<'_>,
) -> StoreResult<PreparedEffect> {
    let result_json = json_string(preparation.admission)?;
    let tx = conn.unchecked_transaction()?;
    tx.execute(
        "INSERT INTO tool_admissions
         (id, case_id, decision_id, tool_view_fingerprint, action_tool, status,
          parsed_action_json, result_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            preparation.id,
            preparation.case_id,
            preparation.admission.decision_id,
            preparation.admission.tool_view_fingerprint,
            preparation.admission.action_tool,
            format!("{:?}", preparation.admission.status),
            preparation.parsed_action_json,
            result_json,
            preparation.created_at
        ],
    )?;
    tx.execute(
        "INSERT INTO effect_journal
         (id, admission_id, decision_id, idempotency_key, command_ordinal, target_path, effect_name, state,
          prior_fingerprint, intended_fingerprint, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'prepared', ?8, ?9, ?10, ?10)",
        params![
            preparation.journal_id,
            preparation.id,
            preparation.admission.decision_id,
            preparation.idempotency_key,
            preparation.command_ordinal,
            preparation.target_path,
            preparation.admission.action_tool,
            preparation.prior_fingerprint,
            preparation.intended_fingerprint,
            preparation.created_at
        ],
    )?;
    tx.commit()?;
    Ok(PreparedEffect {
        admission_id: preparation.id.to_string(),
        journal_id: preparation.journal_id.to_string(),
        command_ordinal: preparation.command_ordinal,
        target_path: preparation.target_path.map(str::to_string),
        prior_fingerprint: preparation.prior_fingerprint.to_string(),
        intended_fingerprint: preparation.intended_fingerprint.to_string(),
        effect_name: preparation.admission.action_tool.clone(),
    })
}

pub fn mark_journal(conn: &Connection, id: &str, state: &str, now: &str) -> StoreResult<()> {
    let expected = match state {
        "applying" => "prepared",
        "compensated" => "applying",
        _ => {
            return Err(StoreError::InvalidState(format!(
                "invalid journal transition to {state}"
            )))
        }
    };
    let changed = conn.execute(
        "UPDATE effect_journal SET state = ?2, updated_at = ?3
         WHERE id = ?1 AND state = ?4 AND observation_id IS NULL",
        params![id, state, now, expected],
    )?;
    if changed == 1 {
        Ok(())
    } else {
        Err(StoreError::InvalidState(
            "journal transition was not available".to_string(),
        ))
    }
}

pub fn insert_tool_admission(
    conn: &Connection,
    id: &str,
    case_id: &str,
    admission: &ToolAdmission,
    parsed_action_json: &str,
    created_at: &str,
) -> StoreResult<()> {
    let result_json = json_string(admission)?;
    conn.execute(
        "INSERT INTO tool_admissions
         (id, case_id, decision_id, tool_view_fingerprint, action_tool, status,
          parsed_action_json, result_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            id,
            case_id,
            admission.decision_id,
            admission.tool_view_fingerprint,
            admission.action_tool,
            format!("{:?}", admission.status),
            parsed_action_json,
            result_json,
            created_at,
        ],
    )?;
    Ok(())
}
