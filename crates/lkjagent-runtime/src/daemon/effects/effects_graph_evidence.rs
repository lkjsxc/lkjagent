use rusqlite::Connection;

use crate::error::RuntimeResult;

pub(super) fn record_evidence(
    conn: &Connection,
    now: &str,
    case_id: i64,
    requirement: String,
    kind: String,
    summary: String,
    path: Option<String>,
) -> RuntimeResult<()> {
    let evidence = lkjagent_store::graph::GraphEvidenceRow {
        requirement,
        kind,
        summary,
        path,
    };
    lkjagent_store::graph::record_evidence(conn, case_id, &evidence, now)?;
    update_pending_checks(conn, now, case_id, &evidence.requirement)?;
    Ok(())
}

fn update_pending_checks(
    conn: &Connection,
    now: &str,
    case_id: i64,
    requirement: &str,
) -> RuntimeResult<()> {
    let Some(row) = lkjagent_store::graph::active_case(conn)? else {
        return Ok(());
    };
    if row.id != case_id {
        return Ok(());
    }
    let mut pending = row.pending_checks;
    pending.retain(|check| !satisfied_check(requirement, check));
    lkjagent_store::graph::update_pending_checks(conn, case_id, &pending, now)?;
    Ok(())
}

fn satisfied_check(requirement: &str, check: &str) -> bool {
    matches!(
        (requirement, check),
        ("verification", "focused verification")
            | ("document-structure", "document audit")
            | ("artifact-readiness", "artifact readiness audit")
    )
}
