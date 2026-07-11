use std::path::Path;

use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_state::StateKey;
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::Connection;
use sha2::{Digest, Sha256};

const CONDITION_KEY: &str = "runtime.endpoint_condition_fingerprint";

pub fn persist_condition(conn: &Connection, data_dir: &Path) -> Result<String, String> {
    let config = crate::config::load_client(data_dir)?;
    let retry: String = conn
        .query_row(
            "SELECT value FROM config WHERE key = 'runtime.endpoint_retry_limit'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let backoff: String = conn
        .query_row(
            "SELECT value FROM config WHERE key = 'runtime.endpoint_backoff_milliseconds'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let credential = credential_token(config.api_key.as_deref());
    let fingerprint = stable_fingerprint(&(
        config.base_url,
        config.model,
        config.timeout.as_millis(),
        credential,
        retry,
        backoff,
    ))
    .map_err(|error| error.message)?;
    conn.execute(
        "INSERT INTO config (key, value) VALUES (?1, ?2)
        ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        (CONDITION_KEY, &fingerprint),
    )
    .map_err(|error| error.to_string())?;
    Ok(fingerprint)
}

fn credential_token(value: Option<&str>) -> String {
    let Some(value) = value else {
        return "absent".to_string();
    };
    let mut digest = Sha256::new();
    digest.update(b"lkjagent:endpoint-credential:v1\0");
    digest.update(value.as_bytes());
    format!("sha256:{:x}", digest.finalize())
}

pub fn release_changed_waits(conn: &Connection, case_id: &str, now: &str) -> Result<usize, String> {
    let current: String = conn
        .query_row(
            "SELECT value FROM config WHERE key = ?1",
            [CONDITION_KEY],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let mut statement = conn
        .prepare(
            "SELECT key_label, payload_json FROM state_cells
        WHERE case_id = ?1 AND status = 'Active' AND payload_schema = 'recovery.failure'
        AND json_extract(payload_json, '$.next_strategy') = 'wait-external'",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([case_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| error.to_string())?;
    let mut changed = Vec::new();
    for row in rows {
        let (label, payload) = row.map_err(|error| error.to_string())?;
        let value: serde_json::Value =
            serde_json::from_str(&payload).map_err(|error| error.to_string())?;
        let saved = value
            .get("endpoint_condition_fingerprint")
            .and_then(|item| item.as_str());
        if saved != Some(current.as_str()) {
            changed.push(label);
        }
    }
    drop(statement);
    if changed.is_empty() {
        return Ok(0);
    }
    let tx = conn
        .unchecked_transaction()
        .map_err(|error| error.to_string())?;
    for label in &changed {
        let id =
            next_event_id(&tx, case_id, "endpoint-condition").map_err(|error| error.to_string())?;
        let key = StateKey::from_label(label).map_err(|error| error.message)?;
        let event = RuntimeEvent {
            id,
            case_id: case_id.to_string(),
            kind: "state.cell.suppress".to_string(),
            payload: RuntimeEventPayload::SuppressCell {
                key,
                reason: "endpoint configuration fingerprint changed".to_string(),
            },
            source: "endpoint-recovery".to_string(),
            created_at: now.to_string(),
            decision_id: None,
        };
        append_and_apply_event(&tx, &event).map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())?;
    Ok(changed.len())
}
