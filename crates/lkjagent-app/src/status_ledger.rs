use rusqlite::Connection;

pub fn lines(conn: &Connection) -> Result<String, String> {
    let active = count_sql(
        conn,
        "SELECT COUNT(*) FROM state_cells WHERE status = 'Active'",
    )?;
    let conflicts = count_sql(
        conn,
        "SELECT COUNT(*) FROM state_cells WHERE key_label LIKE 'context:conflict/%'",
    )?;
    let admissions = count_table(conn, "tool_admissions")?;
    let observations = count_table(conn, "observations")?;
    let exchanges = count_table(conn, "provider_exchanges")?;
    let artifacts = count_table(conn, "artifacts")?;
    let blocked = count_sql(conn, "SELECT COUNT(*) FROM tasks WHERE state = 'blocked'")?;
    let refused = count_sql(
        conn,
        "SELECT COUNT(*) FROM tool_admissions WHERE status = 'Rejected'",
    )?;
    let stale = count_sql(
        conn,
        "SELECT COUNT(*) FROM state_edges WHERE status = 'Suppressed'",
    )?;
    Ok(format!(
        "state: active={active} conflicts={conflicts}\ndecision: {}\ncontext_lanes: {}\nadmissions: {admissions} observations: {observations} exchanges: {exchanges} artifacts: {artifacts}\nevidence: blocked={blocked} refused={refused} stale_edges={stale}",
        decision_line(conn)?,
        context_lanes(conn)?
    ))
}

fn context_lanes(conn: &Connection) -> Result<String, String> {
    let row = conn.query_row(
        "SELECT reason FROM prompt_cards WHERE kind = 'facts' ORDER BY created_at DESC, id DESC LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    );
    match row {
        Ok(line) => Ok(line),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok("none".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn decision_line(conn: &Connection) -> Result<String, String> {
    let row = conn.query_row(
        "SELECT id, operation_key, status, substr(context_frame_fingerprint, 1, 16),
         substr(tool_view_fingerprint, 1, 16)
         FROM runtime_decisions ORDER BY selected_at DESC, id DESC LIMIT 1",
        [],
        |row| {
            Ok(format!(
                "{} {} status={} ctx={} tools={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?
            ))
        },
    );
    match row {
        Ok(line) => Ok(line),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok("none".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn count_table(conn: &Connection, table: &str) -> Result<i64, String> {
    count_sql(conn, &format!("SELECT COUNT(*) FROM {table}"))
}

fn count_sql(conn: &Connection, sql: &str) -> Result<i64, String> {
    conn.query_row(sql, [], |row| row.get(0))
        .map_err(|error| error.to_string())
}
