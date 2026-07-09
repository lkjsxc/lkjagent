use std::path::Path;

use rusqlite::{params, Connection};
use serde_json::json;

use crate::config::{
    endpoint_state, file_count, join_bools, join_counts, join_or_none, missing_dirs,
};

pub fn doctor(conn: &Connection, data_dir: &Path, as_json: bool) -> Result<String, String> {
    let missing_tables = missing_tables(conn)?;
    let counts = table_counts(conn)?;
    let workspace_root = crate::config::workspace_root(data_dir)?;
    let prompt_cap = crate::config::prompt_max_context_tokens(data_dir)?;
    let live_seconds = crate::config::live_campaign_seconds(data_dir)?;
    let missing_dirs = missing_dirs(data_dir);
    let endpoint = endpoint_state(data_dir);
    let unfinished = count_where(conn, "runtime_decisions", "status != 'settled'")?;
    let orphan_prompts = scalar_count(
        conn,
        "SELECT COUNT(*) FROM prompt_frames pf
         LEFT JOIN runtime_decisions rd ON rd.id = pf.decision_id
         WHERE rd.id IS NULL",
    )?;
    let mut warnings = Vec::new();
    if !missing_tables.is_empty() {
        warnings.push("missing-tables".to_string());
    }
    if !missing_dirs.is_empty() {
        warnings.push("missing-workspace-dirs".to_string());
    }
    if orphan_prompts > 0 {
        warnings.push("orphan-prompt-refs".to_string());
    }
    if as_json {
        return Ok(json!({
            "schema": if missing_tables.is_empty() { "ok" } else { "warn" },
            "missing_tables": missing_tables,
            "table_counts": counts,
            "lease": crate::lease_status::line(conn)?,
            "endpoint": endpoint,
            "workspace_root": workspace_root.display().to_string(),
            "prompt_max_context_tokens": prompt_cap,
            "live_campaign_seconds": live_seconds,
            "missing_dirs": missing_dirs,
            "unfinished_decisions": unfinished,
            "orphan_prompt_refs": orphan_prompts,
            "warnings": warnings,
        })
        .to_string());
    }
    Ok([
        format!(
            "doctor: {}",
            if warnings.is_empty() { "ok" } else { "warn" }
        ),
        format!(
            "schema: tables={} missing={}",
            counts.len(),
            join_or_none(&missing_tables)
        ),
        format!("table_counts: {}", join_counts(&counts)),
        crate::lease_status::line(conn)?,
        format!("endpoint: {endpoint}"),
        format!(
            "workspace: root={} missing={} prompt_cap={} live_seconds={}",
            workspace_root.display(),
            join_or_none(&missing_dirs),
            option_number(prompt_cap),
            option_number(live_seconds)
        ),
        format!("decisions: unfinished={unfinished}"),
        format!("prompt_refs: orphan={orphan_prompts}"),
        format!("warnings: {}", join_or_none(&warnings)),
    ]
    .join("\n"))
}

pub fn workspace(conn: &Connection, data_dir: &Path, as_json: bool) -> Result<String, String> {
    let root = crate::config::workspace_root(data_dir)?;
    let total = count_where(conn, "workspace_records", "1=1")?;
    let archived = count_where(conn, "workspace_records", "archived != 0")?;
    let artifacts = count_where(conn, "artifacts", "1=1")?;
    let index_files = file_count(&root.join("indexes"));
    let readmes = vec![
        ("workspace", root.join("README.md").exists()),
        ("records", root.join("records/README.md").exists()),
    ];
    let missing = missing_dirs(data_dir);
    if as_json {
        return Ok(json!({
            "root": root.display().to_string(),
            "records": { "total": total, "archived": archived },
            "artifacts": artifacts,
            "indexes": { "files": index_files },
            "readmes": readmes,
            "missing_dirs": missing,
        })
        .to_string());
    }
    Ok([
        format!("workspace: root={}", root.display()),
        format!("records: total={total} archived={archived}"),
        format!("artifacts: total={artifacts}"),
        format!("indexes: files={index_files}"),
        format!("readmes: {}", join_bools(&readmes)),
        format!("missing: {}", join_or_none(&missing)),
    ]
    .join("\n"))
}

fn option_number(value: Option<u64>) -> String {
    value.map_or_else(|| "unknown".to_string(), |number| number.to_string())
}

fn missing_tables(conn: &Connection) -> Result<Vec<String>, String> {
    lkjagent_store::plan_schema::APPLICATION_TABLES
        .iter()
        .filter_map(|name| match table_exists(conn, name) {
            Ok(true) => None,
            Ok(false) => Some(Ok((*name).to_string())),
            Err(error) => Some(Err(error)),
        })
        .collect()
}

fn table_counts(conn: &Connection) -> Result<Vec<(String, i64)>, String> {
    lkjagent_store::plan_schema::APPLICATION_TABLES
        .iter()
        .map(|name| count_where(conn, name, "1=1").map(|count| ((*name).to_string(), count)))
        .collect()
}

fn table_exists(conn: &Connection, name: &str) -> Result<bool, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
        params![name],
        |row| row.get::<_, i64>(0),
    )
    .map(|count| count > 0)
    .map_err(|error| error.to_string())
}

fn count_where(conn: &Connection, table: &str, condition: &str) -> Result<i64, String> {
    scalar_count(
        conn,
        &format!("SELECT COUNT(*) FROM {table} WHERE {condition}"),
    )
}

fn scalar_count(conn: &Connection, sql: &str) -> Result<i64, String> {
    conn.query_row(sql, [], |row| row.get::<_, i64>(0))
        .map_err(|error| error.to_string())
}
