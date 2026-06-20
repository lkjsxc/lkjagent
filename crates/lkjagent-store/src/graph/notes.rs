use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn record_note(
    conn: &Connection,
    case_id: i64,
    kind: &str,
    summary: &str,
    source: &str,
    now: &str,
) -> StoreResult<()> {
    match kind {
        "constraint" => insert_constraint(conn, case_id, summary, source, now),
        "assumption" => insert_simple(conn, "graph_assumptions", case_id, summary, "open", now),
        "question" => insert_question(conn, case_id, summary, now),
        "risk" => insert_risk(conn, case_id, summary, now),
        "success" => insert_simple(
            conn,
            "graph_success_criteria",
            case_id,
            summary,
            "open",
            now,
        ),
        _ => Ok(()),
    }
}

fn insert_constraint(
    conn: &Connection,
    case_id: i64,
    summary: &str,
    source: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_constraints
         (case_id, kind, summary, source, strength, created_at)
         VALUES (?1, 'constraint', ?2, ?3, 'hard', ?4)",
        params![case_id, summary, source, now],
    )?;
    Ok(())
}

fn insert_simple(
    conn: &Connection,
    table: &str,
    case_id: i64,
    summary: &str,
    status: &str,
    now: &str,
) -> StoreResult<()> {
    let sql = format!(
        "INSERT INTO {table} (case_id, summary, status, created_at) VALUES (?1, ?2, ?3, ?4)"
    );
    conn.execute(&sql, params![case_id, summary, status, now])?;
    Ok(())
}

fn insert_question(conn: &Connection, case_id: i64, question: &str, now: &str) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_questions (case_id, question, status, created_at)
         VALUES (?1, ?2, 'open', ?3)",
        params![case_id, question, now],
    )?;
    Ok(())
}

fn insert_risk(conn: &Connection, case_id: i64, summary: &str, now: &str) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_risks (case_id, summary, mitigation, status, created_at)
         VALUES (?1, ?2, 'track during execution', 'open', ?3)",
        params![case_id, summary, now],
    )?;
    Ok(())
}
