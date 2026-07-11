use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, Row};
use serde_json::Value;

use super::io::{err, field, file_hash, file_map, table};

#[rustfmt::skip]
pub(super) fn rebuild_hash(root: &Path) -> Result<String, String> {
    let scratch = env::temp_dir().join(format!("lkjagent-domain-build-{}-{}", std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(err)?.as_nanos()));
    fs::create_dir_all(&scratch).map_err(err)?; let has_git=root.join(".git").exists();
    let source = if has_git { scratch.join("source") } else { root.to_path_buf() };
    if has_git { let added = Command::new("git").args(["worktree", "add", "--detach", "--quiet"])
        .arg(&source).arg("HEAD").current_dir(root).status().map_err(err)?;
        if !added.success() { return Err("independent build worktree failed".into()); } }
    let result = build(&source, &scratch);
    let removed = !has_git || Command::new("git").args(["worktree", "remove", "--force"])
        .arg(&source).current_dir(root).status().map_err(err)?.success();
    let cleaned = fs::remove_dir_all(&scratch).is_ok();
    if !removed || !cleaned { return Err("independent build cleanup failed".into()); } result
}

#[rustfmt::skip]
fn build(source: &Path, scratch: &Path) -> Result<String, String> {
    let cargo_home = scratch.join("cargo-home"); fs::create_dir_all(&cargo_home).map_err(err)?;
    let original = env::var_os("CARGO_HOME").map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo"))).ok_or("cargo home unavailable")?;
    for name in ["registry", "git"] { let input = original.join(name);
        if input.exists() { symlink(input, cargo_home.join(name)).map_err(err)?; } }
    let target = scratch.join("target"); let mut command = Command::new("cargo");
    command.args(["build", "--locked", "--offline", "--quiet", "--release", "-p", "lkjagent-app", "--target-dir"])
        .arg(&target).current_dir(source).env_clear().env("CARGO_HOME", &cargo_home)
        .env("RUSTFLAGS", format!("--remap-path-prefix={}=/lkjagent-source --remap-path-prefix={}=/cargo-home --remap-path-prefix={}=/cargo-home",
            source.display(), original.display(), cargo_home.display()));
    for key in ["PATH", "HOME", "RUSTUP_HOME", "SSL_CERT_FILE", "SSL_CERT_DIR"] {
        if let Some(value) = env::var_os(key) { command.env(key, value); }
    }
    if !command.status().map_err(err)?.success() { return Err("independent locked build failed".into()); }
    file_hash(&target.join("release/lkjagent"))
}

#[rustfmt::skip]
pub(super) fn validate_workspace(conn: &Connection, run: &Path, run_id: &str) -> Result<(), String> {
    let workspace = run.join("workspace"); if workspace.is_symlink() || !workspace.is_dir() { return Err(format!("{run_id} workspace escaped")); }
    let rows = table(&run.join("workspace-manifest.tsv"))?;
    let declared = rows.iter().map(|row| (field(row, "path"), row)).collect::<BTreeMap<_, _>>();
    let actual = file_map(&workspace)?; if rows.len() != declared.len() || actual.len() != declared.len() { return Err(format!("{run_id} workspace manifest coverage mismatch")); }
    let mut statement = conn.prepare("SELECT path, id, fingerprint FROM workspace_records WHERE archived = 0").map_err(err)?;
    let records = statement.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?)))
        .map_err(err)?.collect::<Result<Vec<_>, _>>().map_err(err)?; let mut managed = BTreeMap::new();
    for (path, id, revision) in records { let relative = Path::new(&path);
        if relative.is_absolute() || relative.components().any(|item| !matches!(item, Component::Normal(_)))
            || managed.insert(path, (id, revision)).is_some() { return Err(format!("{run_id} workspace database path invalid")); } }
    if managed.keys().any(|path| !actual.contains_key(path)) { return Err(format!("{run_id} active workspace row missing")); }
    for (path, digest) in actual { let Some(row) = declared.get(path.as_str()) else { return Err(format!("{run_id} workspace path missing")); };
        let expected = managed.get(&path).cloned().unwrap_or_else(|| (format!("external:{digest}"), digest.clone()));
        if expected.1 != digest || field(row, "document_id") != expected.0 || field(row, "revision_id") != expected.1
            || field(row, "sha256") != digest { return Err(format!("{run_id} workspace identity mismatch")); }
    }
    Ok(())
}

#[rustfmt::skip]
pub(super) fn validate_exports(conn: &Connection, run: &Path, run_id: &str) -> Result<(), String> {
    let mut statement = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").map_err(err)?;
    let names = statement.query_map([], |row| row.get::<_,String>(0)).map_err(err)?.collect::<Result<Vec<_>,_>>().map_err(err)?;
    let actual = fs::read_dir(run.join("tables")).map_err(err)?.map(|entry| entry.map(|item| item.file_name().to_string_lossy().into_owned()).map_err(err))
        .collect::<Result<BTreeSet<_>,_>>()?; let expected = names.iter().map(|name| format!("{name}.jsonl")).collect::<BTreeSet<_>>();
    if actual != expected { return Err(format!("{run_id} table export coverage mismatch")); }
    for name in names { let path=run.join("tables").join(format!("{name}.jsonl")); if path.is_symlink() { return Err(format!("{run_id} table export symlink")); }
        let source=fs::read_to_string(path).map_err(err)?; let lines=source.lines().collect::<Vec<_>>();
        let escaped=name.replace('"', "\"\""); let count:i64=conn.query_row(&format!("SELECT COUNT(*) FROM \"{escaped}\""),[],|row|row.get(0)).map_err(err)?;
        let mut columns=conn.prepare(&format!("PRAGMA table_info(\"{escaped}\")")).map_err(err)?;
        let columns=columns.query_map([],|row|row.get::<_,String>(1)).map_err(err)?.collect::<Result<Vec<_>,_>>().map_err(err)?;
        let header:Value=serde_json::from_str(lines.first().ok_or("table export header missing")?).map_err(err)?;
        if header.get("_columns") != Some(&serde_json::json!(columns)) || lines.len()!=usize::try_from(count).map_err(err)?+1 {
            return Err(format!("{run_id} table export mismatch")); }
        let mut query=conn.prepare(&format!("SELECT * FROM \"{escaped}\"")).map_err(err)?; let mut db_rows=query.query([]).map_err(err)?;
        for line in lines.iter().skip(1) { let exported:Value=serde_json::from_str(line).map_err(err)?;
            let object=exported.as_object().ok_or("table export row is not an object")?; let row=db_rows.next().map_err(err)?.ok_or("table export row missing")?;
            if object.len()!=columns.len() || columns.iter().enumerate().any(|(index,column)| sql_value(row,index).ok().as_ref()!=object.get(column)) {
                return Err(format!("{run_id} table export values differ")); } }
    }
    Ok(())
}

fn sql_value(row: &Row<'_>, index: usize) -> Result<Value, String> {
    Ok(match row.get_ref(index).map_err(err)? {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(value) => Value::from(value),
        ValueRef::Real(value) => serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or("invalid SQLite real")?,
        ValueRef::Text(value) => Value::String(String::from_utf8(value.to_vec()).map_err(err)?),
        ValueRef::Blob(value) => Value::String(format!(
            "hex:{}",
            value
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        )),
    })
}

#[rustfmt::skip]
pub(super) fn validate_exchange_logs(conn: &Connection, run: &Path, run_id: &str) -> Result<(), String> {
    let raw = file_map(&run.join("data/logs"))?; let redacted = file_map(&run.join("logs-redacted"))?;
    if raw.keys().collect::<BTreeSet<_>>() != redacted.keys().collect::<BTreeSet<_>>() { return Err(format!("{run_id} redacted log coverage mismatch")); }
    for relative in redacted.keys() { let value:Value=serde_json::from_str(&fs::read_to_string(run.join("logs-redacted").join(relative)).map_err(err)?).map_err(err)?;
        if !redaction_valid(&value, false) { return Err(format!("{run_id} redacted log leaked")); } }
    let refs:(i64,i64,i64)=conn.query_row("SELECT COUNT(*),COUNT(DISTINCT exchange_ref),SUM(CASE WHEN finished_at IS NULL THEN 1 ELSE 0 END) FROM provider_exchanges",[],
        |row|Ok((row.get(0)?,row.get(1)?,row.get::<_,Option<i64>>(2)?.unwrap_or(0)))).map_err(err)?;
    if refs.0!=refs.1 || refs.2!=0 { return Err(format!("{run_id} exchange ref reused or unfinished")); }
    let mut statement = conn.prepare("SELECT p.exchange_ref,p.decision_id,p.outcome_json,p.context_frame_fingerprint,
        p.timeout_seconds,d.tool_view_fingerprint,f.prompt_fingerprint,
        CAST(ROUND((julianday(p.finished_at)-julianday(p.started_at))*86400000) AS INTEGER) FROM provider_exchanges p
        JOIN runtime_decisions d ON d.id=p.decision_id JOIN prompt_frames f ON f.decision_id=p.decision_id
        ORDER BY p.started_at,p.id").map_err(err)?;
    let rows = statement.query_map([], |row| Ok((row.get::<_,String>(0)?,row.get::<_,String>(1)?,row.get::<_,String>(2)?,
        row.get::<_,String>(3)?,row.get::<_,Option<i64>>(4)?,row.get::<_,String>(5)?,row.get::<_,String>(6)?,row.get::<_,i64>(7)?)))
        .map_err(err)?.collect::<Result<Vec<_>,_>>().map_err(err)?; let mut references = BTreeSet::new();
    for (reference, decision, outcome_json, context, timeout, tool_view, prompt, db_duration) in rows {
        if !references.insert(reference.clone()) { return Err(format!("{run_id} exchange ref reused")); }
        let relative = Path::new(&reference);
        if relative.is_absolute() || relative.components().any(|part| !matches!(part, Component::Normal(_))) { return Err(format!("{run_id} exchange ref escaped")); }
        let base = run.join("data").join(relative); for name in ["request.json","response.json","outcome.json","timing.json"] {
            let path=base.join(name); if path.is_symlink() || !path.is_file() { return Err(format!("{run_id} exchange log missing")); } }
        let request: Value = serde_json::from_str(&fs::read_to_string(base.join("request.json")).map_err(err)?).map_err(err)?;
        let response: Value = serde_json::from_str(&fs::read_to_string(base.join("response.json")).map_err(err)?).map_err(err)?;
        let outcome: Value = serde_json::from_str(&fs::read_to_string(base.join("outcome.json")).map_err(err)?).map_err(err)?;
        let timing: Value = serde_json::from_str(&fs::read_to_string(base.join("timing.json")).map_err(err)?).map_err(err)?;
        let kind=outcome.get("outcome").and_then(Value::as_str); let duration=timing.get("duration_ms").and_then(Value::as_i64);
        let response_matches = if kind==Some("endpoint_error") { response.get("error")==outcome.get("diagnosis") }
            else { response.get("finish_reason").and_then(Value::as_str).is_some_and(|value| value!="scripted")
                && response.get("content").and_then(Value::as_str).is_some_and(|value| !value.is_empty()) };
        if request.get("decision_id").and_then(Value::as_str)!=Some(decision.as_str()) || request.get("fingerprint").and_then(Value::as_str)!=Some(prompt.as_str())
            || request.get("context_frame_fingerprint").and_then(Value::as_str)!=Some(context.as_str())
            || request.get("tool_view_fingerprint").and_then(Value::as_str)!=Some(tool_view.as_str())
            || request.get("timeout_seconds").and_then(Value::as_i64)!=timeout || timing.get("timeout_seconds").and_then(Value::as_i64)!=timeout
            || duration.is_none_or(|value| value<0 || (value-db_duration).abs()>2_000)
            || outcome != serde_json::from_str::<Value>(&outcome_json).map_err(err)?
            || !matches!(kind, Some("parsed" | "parse_fault" | "endpoint_error")) || !response_matches {
            return Err(format!("{run_id} exchange evidence differs")); }
    }
    Ok(())
}

fn redaction_valid(value: &Value, sensitive: bool) -> bool {
    match value {
        Value::Object(items) => items.iter().all(|(key, value)| {
            redaction_valid(
                value,
                sensitive
                    || matches!(
                        key.as_str(),
                        "content"
                            | "system"
                            | "user"
                            | "error"
                            | "diagnosis"
                            | "detail"
                            | "message"
                            | "body"
                            | "preview"
                            | "anomaly"
                            | "raw"
                    ),
            )
        }),
        Value::Array(items) => items.iter().all(|value| redaction_valid(value, sensitive)),
        Value::String(value) if sensitive => {
            value.starts_with("[redacted sha256:") && value.ends_with(']') && value.len() == 82
        }
        _ => true,
    }
}
