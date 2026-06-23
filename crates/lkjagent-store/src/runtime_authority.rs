use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, Copy)]
pub struct AuthorityEventInput<'a> {
    pub case_id: i64,
    pub event_kind: &'a str,
    pub event_payload: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct AuthorityDecisionInput<'a> {
    pub case_id: i64,
    pub event_id: i64,
    pub mission: &'a str,
    pub active_mode: &'a str,
    pub active_node: &'a str,
    pub admitted_tools: &'a [String],
    pub blocked_tools: &'a [String],
    pub missing_evidence: &'a [String],
    pub forced_next_action: &'a str,
    pub exact_valid_example: Option<&'a str>,
    pub completion_allowed: bool,
    pub completion_refusal: Option<&'a str>,
    pub recovery_route: Option<&'a str>,
    pub compaction_required: bool,
    pub maintenance_allowed: bool,
    pub authority_fingerprint: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct ToolAdmissionInput<'a> {
    pub decision_id: i64,
    pub case_id: i64,
    pub requested_tool: &'a str,
    pub admitted: bool,
    pub refusal_reason: &'a str,
    pub exact_valid_example: Option<&'a str>,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityDecisionRow {
    pub id: i64,
    pub case_id: i64,
    pub event_id: i64,
    pub mission: String,
    pub active_mode: String,
    pub active_node: String,
    pub admitted_tools: String,
    pub blocked_tools: String,
    pub missing_evidence: String,
    pub forced_next_action: String,
    pub exact_valid_example: Option<String>,
    pub completion_allowed: bool,
    pub completion_refusal: Option<String>,
    pub recovery_route: Option<String>,
    pub compaction_required: bool,
    pub maintenance_allowed: bool,
    pub authority_fingerprint: String,
}

pub fn record_event(conn: &Connection, input: &AuthorityEventInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_authority_events
         (case_id, event_kind, event_payload, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            input.case_id,
            input.event_kind,
            input.event_payload,
            input.created_at
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_decision(conn: &Connection, input: &AuthorityDecisionInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_authority_decisions
         (case_id, event_id, mission, active_mode, active_node, admitted_tools,
          blocked_tools, missing_evidence, forced_next_action, exact_valid_example,
          completion_allowed, completion_refusal, recovery_route, compaction_required,
          maintenance_allowed, authority_fingerprint, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
        params![
            input.case_id,
            input.event_id,
            input.mission,
            input.active_mode,
            input.active_node,
            join(input.admitted_tools),
            join(input.blocked_tools),
            join(input.missing_evidence),
            input.forced_next_action,
            input.exact_valid_example,
            as_i64(input.completion_allowed),
            input.completion_refusal,
            input.recovery_route,
            as_i64(input.compaction_required),
            as_i64(input.maintenance_allowed),
            input.authority_fingerprint,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_tool_admission(
    conn: &Connection,
    input: &ToolAdmissionInput<'_>,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_tool_admissions
         (decision_id, case_id, requested_tool, admitted, refusal_reason,
          exact_valid_example, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            input.decision_id,
            input.case_id,
            input.requested_tool,
            as_i64(input.admitted),
            input.refusal_reason,
            input.exact_valid_example,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn latest_decision(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<AuthorityDecisionRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, event_id, mission, active_mode, active_node,
         admitted_tools, blocked_tools, missing_evidence, forced_next_action,
         exact_valid_example, completion_allowed, completion_refusal, recovery_route,
         compaction_required, maintenance_allowed, authority_fingerprint
         FROM runtime_authority_decisions WHERE case_id = ?1 ORDER BY id DESC LIMIT 1",
    )?;
    let mut rows = statement.query(params![case_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(AuthorityDecisionRow {
        id: row.get(0)?,
        case_id: row.get(1)?,
        event_id: row.get(2)?,
        mission: row.get(3)?,
        active_mode: row.get(4)?,
        active_node: row.get(5)?,
        admitted_tools: row.get(6)?,
        blocked_tools: row.get(7)?,
        missing_evidence: row.get(8)?,
        forced_next_action: row.get(9)?,
        exact_valid_example: row.get(10)?,
        completion_allowed: row.get::<_, i64>(11)? != 0,
        completion_refusal: row.get(12)?,
        recovery_route: row.get(13)?,
        compaction_required: row.get::<_, i64>(14)? != 0,
        maintenance_allowed: row.get::<_, i64>(15)? != 0,
        authority_fingerprint: row.get(16)?,
    }))
}

fn join(values: &[String]) -> String {
    values.join(",")
}

fn as_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}
