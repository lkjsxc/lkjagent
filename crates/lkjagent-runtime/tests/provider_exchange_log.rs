use std::fs;
use std::path::PathBuf;

use lkjagent_runtime::model_log::{
    record_provider_error, record_provider_request, record_provider_response, ProviderLogContext,
};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn provider_exchange_writer_persists_files_and_store_rows() -> TestResult<()> {
    let root = temp_root("success")?;
    let conn = memory_store()?;
    let context = context("7", 5);
    let handle = record_provider_request(&conn, &root, &context, "{\"messages\":[]}")?;
    record_provider_response(
        &conn,
        &handle,
        "{\"content\":\"ok\"}",
        "stop",
        Some("{\"total_tokens\":4}"),
        12,
    )?;

    assert!(handle.dir.join("request.json").exists());
    assert!(handle.dir.join("authority.json").exists());
    assert!(handle.dir.join("response.json").exists());
    assert!(handle.dir.join("timing.json").exists());
    let row = lkjagent_store::provider_exchange::latest_for_case_turn(&conn, "7", 5)?
        .ok_or("missing provider exchange row")?;
    assert_eq!(row.status, "succeeded");
    assert_eq!(row.finish_reason.as_deref(), Some("stop"));
    Ok(())
}

#[test]
fn provider_exchange_writer_records_errors_ndjson() -> TestResult<()> {
    let root = temp_root("error")?;
    let conn = memory_store()?;
    let context = context("none", 6);
    let handle = record_provider_request(&conn, &root, &context, "{\"messages\":[]}")?;
    record_provider_error(&conn, &handle, "EndpointError", "offline", 9)?;

    let errors = fs::read_to_string(handle.dir.join("errors.ndjson"))?;
    assert!(errors.contains("EndpointError"));
    let row = lkjagent_store::provider_exchange::latest_for_case_turn(&conn, "none", 6)?
        .ok_or("missing provider exchange row")?;
    assert_eq!(row.status, "failed");
    assert_eq!(row.error_class.as_deref(), Some("EndpointError"));
    Ok(())
}

fn context(case_id: &str, turn_id: i64) -> ProviderLogContext {
    ProviderLogContext {
        case_id: case_id.to_string(),
        turn_id,
        prompt_frame_id: Some("prompt".to_string()),
        authority_decision_id: Some("42".to_string()),
        provider: "openai-compatible".to_string(),
        model: "local-model".to_string(),
        created_at: "1782200000".to_string(),
        authority_json: "{\"active_mode\":\"OwnerTask\"}\n".to_string(),
    }
}

fn memory_store() -> TestResult<Connection> {
    let conn = Connection::open_in_memory()?;
    lkjagent_store::schema::setup(&conn)?;
    Ok(conn)
}

fn temp_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-provider-log-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
