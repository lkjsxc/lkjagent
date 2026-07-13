use rusqlite::{params, Transaction};
use serde_json::json;

use crate::error::{StoreError, StoreResult};
use crate::transactions::Effect;

pub(crate) fn insert(tx: &Transaction<'_>, effect: &Effect<'_>) -> StoreResult<()> {
    let matter: String = tx.query_row(
        "SELECT matter_id FROM runtime_decisions WHERE id=?1",
        [effect.decision],
        |row| row.get(0),
    )?;
    let target = effect
        .targets
        .first()
        .ok_or_else(|| StoreError::InvalidState("effect has no target".into()))?;
    let path = std::str::from_utf8(target.path)
        .map_err(|error| StoreError::InvalidState(error.to_string()))?;
    let intended = target
        .intended
        .ok_or_else(|| StoreError::InvalidState("effect has no intended bytes".into()))?;
    let text = std::str::from_utf8(intended)
        .map_err(|error| StoreError::InvalidState(error.to_string()))?;
    let old = target
        .prior
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .unwrap_or("");
    let allowed = effect
        .targets
        .iter()
        .map(|item| String::from_utf8_lossy(item.path).into_owned())
        .collect::<Vec<_>>();
    for (kind, payload) in [
        (
            "workspace-byte",
            json!({"path":path,"sha256":hex(effect.intended_fingerprint)}),
        ),
        (
            "workspace-content",
            json!({"path":path,"old":old,"new":text,"old_count":0,"new_count":1}),
        ),
        ("workspace-collateral", json!({"allowed_paths":allowed})),
    ] {
        let id =
            stable_id(&matter, path, kind).unwrap_or_else(|| format!("{}/{kind}", effect.journal));
        insert_one(tx, &id, &matter, kind, &payload.to_string())?;
    }
    if effect.reason == b"workspace.record" {
        crate::managed_record_obligations::insert(tx, effect, &matter, path, text)?;
    }
    Ok(())
}

pub(super) fn insert_one(
    tx: &Transaction<'_>,
    id: &str,
    matter: &str,
    kind: &str,
    payload: &str,
) -> StoreResult<()> {
    if id.starts_with(matter) && id.contains("artifacts/documents/") {
        tx.execute(
            "INSERT INTO obligations(id,matter_id,predicate_kind,predicate_payload,required,status)
             VALUES(?1,?2,?3,?4,1,'open') ON CONFLICT(id) DO UPDATE SET
             predicate_kind=excluded.predicate_kind,predicate_payload=excluded.predicate_payload,
             required=1,status='open',current_check_id=NULL,invalidating_event_id=NULL",
            params![id, matter, kind, payload.as_bytes()],
        )?;
    } else {
        tx.execute(
            "INSERT INTO obligations(id,matter_id,predicate_kind,predicate_payload,required,status)
             VALUES(?1,?2,?3,?4,1,'open')",
            params![id, matter, kind, payload.as_bytes()],
        )?;
    }
    Ok(())
}

fn stable_id(matter: &str, path: &str, kind: &str) -> Option<String> {
    path.starts_with("artifacts/documents/")
        .then(|| format!("{matter}/{kind}/{path}"))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
