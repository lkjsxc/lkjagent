use lkjagent_core::runtime_admission::ToolAdmission;
use rusqlite::{params, Connection};

use crate::error::StoreResult;
use crate::row_json::json_string;

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
