use rusqlite::{params, Connection};

pub fn show(conn: &Connection, id: u64) -> Result<String, String> {
    let snapshot = lkjagent_store::plan_hydrate::snapshot_by_id(conn, id as i64)
        .map_err(|error| error.to_string())?;
    let Some(snapshot) = snapshot else {
        return Ok(format!("task {id}: not found"));
    };
    let case_id = id.to_string();
    Ok([
        crate::status::task_show(&snapshot),
        state_line(conn, &case_id)?,
        decision_lines(conn, &case_id)?,
        prompt_line(conn, &case_id)?,
        check_line(conn, id as i64)?,
        artifact_lines(conn, &case_id)?,
        exchange_lines(conn, &case_id)?,
    ]
    .join("\n"))
}

fn state_line(conn: &Connection, case_id: &str) -> Result<String, String> {
    let active = scalar(
        conn,
        "SELECT COUNT(*) FROM state_cells WHERE case_id = ?1 AND status = 'Active'",
        case_id,
    )?;
    let conflicts = scalar(
        conn,
        "SELECT COUNT(*) FROM state_cells WHERE case_id = ?1 AND key_label LIKE 'context:conflict/%'",
        case_id,
    )?;
    Ok(format!("state: active={active} conflicts={conflicts}"))
}

fn decision_lines(conn: &Connection, case_id: &str) -> Result<String, String> {
    let rows = query_lines(
        conn,
        "SELECT id, operation_key, status, substr(context_frame_fingerprint, 1, 16),
         substr(tool_view_fingerprint, 1, 16)
         FROM runtime_decisions WHERE case_id = ?1 ORDER BY selected_at DESC, id DESC LIMIT 5",
        case_id,
        |row| {
            Ok(format!(
                "- {} {} status={} ctx={} tools={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?
            ))
        },
    )?;
    section("decisions", rows)
}

fn prompt_line(conn: &Connection, case_id: &str) -> Result<String, String> {
    Ok(format!(
        "prompt_frames: {}",
        scalar(
            conn,
            "SELECT COUNT(*) FROM prompt_frames WHERE case_id = ?1",
            case_id
        )?
    ))
}

fn check_line(conn: &Connection, task_id: i64) -> Result<String, String> {
    let total: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM check_results c JOIN steps s ON s.id = c.step_id
             WHERE s.task_id = ?1",
            [task_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let passed: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM check_results c JOIN steps s ON s.id = c.step_id
             WHERE s.task_id = ?1 AND c.passed != 0",
            [task_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    Ok(format!(
        "checks: total={total} passed={passed} failed={}",
        total - passed
    ))
}

fn artifact_lines(conn: &Connection, case_id: &str) -> Result<String, String> {
    let count = scalar(
        conn,
        "SELECT COUNT(*) FROM artifacts WHERE case_id = ?1",
        case_id,
    )?;
    let rows = query_lines(
        conn,
        "SELECT kind, path, substr(fingerprint, 1, 16) FROM artifacts
         WHERE case_id = ?1 ORDER BY created_at DESC, id DESC LIMIT 5",
        case_id,
        |row| {
            Ok(format!(
                "- kind={} path={} fp={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?
            ))
        },
    )?;
    count_section("artifacts", count, rows)
}

fn exchange_lines(conn: &Connection, case_id: &str) -> Result<String, String> {
    let count = scalar(
        conn,
        "SELECT COUNT(*) FROM provider_exchanges WHERE case_id = ?1",
        case_id,
    )?;
    let rows = query_lines(
        conn,
        "SELECT decision_id, exchange_ref, COALESCE(timeout_seconds, 0)
         FROM provider_exchanges WHERE case_id = ?1 ORDER BY started_at DESC, id DESC LIMIT 5",
        case_id,
        |row| {
            Ok(format!(
                "- decision={} ref={} timeout={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?
            ))
        },
    )?;
    count_section("exchanges", count, rows)
}

fn scalar(conn: &Connection, sql: &str, value: &str) -> Result<i64, String> {
    conn.query_row(sql, [value], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn query_lines<F>(
    conn: &Connection,
    sql: &str,
    case_id: &str,
    render: F,
) -> Result<Vec<String>, String>
where
    F: Fn(&rusqlite::Row<'_>) -> rusqlite::Result<String>,
{
    let mut statement = conn.prepare(sql).map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![case_id], render)
        .map_err(|error| error.to_string())?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| error.to_string())
}

fn section(title: &str, rows: Vec<String>) -> Result<String, String> {
    if rows.is_empty() {
        Ok(format!("{title}: none"))
    } else {
        Ok(format!("{title}:\n{}", rows.join("\n")))
    }
}

fn count_section(name: &str, count: i64, rows: Vec<String>) -> Result<String, String> {
    if rows.is_empty() {
        Ok(format!("{name}: {count} none"))
    } else {
        Ok(format!("{name}: {count}\n{}", rows.join("\n")))
    }
}
