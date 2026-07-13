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
    let values = [
        (
            "workspace-byte",
            json!({"path":path,"sha256":hex(effect.intended_fingerprint)}),
        ),
        (
            "workspace-content",
            json!({"path":path,"old":old,"new":text,"old_count":0,"new_count":1}),
        ),
        ("workspace-collateral", json!({"allowed_paths":allowed})),
    ];
    for (kind, payload) in values {
        insert_one(tx, effect.journal, &matter, kind, &payload.to_string())?;
    }
    if effect.reason == b"workspace.record.journal" {
        insert_managed_journal(tx, effect, &matter, path)?;
    }
    Ok(())
}

fn insert_managed_journal(
    tx: &Transaction<'_>,
    effect: &Effect<'_>,
    matter: &str,
    path: &str,
) -> StoreResult<()> {
    let date = journal_date(path)
        .ok_or_else(|| StoreError::InvalidState("journal path is not canonical".into()))?;
    let mut query = tx.prepare(
        "SELECT source_kind,CAST(source_revision AS TEXT) FROM context_items
         WHERE decision_id=?1 ORDER BY source_kind,source_id,source_revision",
    )?;
    let rows = query
        .query_map([effect.decision], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    if !rows.iter().any(|(kind, _)| kind == "owner") {
        return Err(StoreError::InvalidState(
            "journal has no current owner lineage".into(),
        ));
    }
    let sources = rows
        .into_iter()
        .map(|(_, revision)| revision)
        .collect::<Vec<_>>();
    let payload = json!({
        "path":path,"kind":"journal","date":date,"source_fingerprints":sources,
        "max_token_units":512
    });
    insert_one(
        tx,
        effect.journal,
        matter,
        "managed-journal",
        &payload.to_string(),
    )
}

fn journal_date(path: &str) -> Option<String> {
    let parts = path.split('/').collect::<Vec<_>>();
    if parts.len() != 6
        || parts[0..2] != ["life", "journal"]
        || parts[5] != "entry.md"
        || parts[2].len() != 4
        || parts[3].len() != 2
        || parts[4].len() != 2
        || parts[2..5]
            .iter()
            .any(|part| !part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return None;
    }
    let year = parts[2].parse::<u16>().ok()?;
    let month = parts[3].parse::<u8>().ok()?;
    let day = parts[4].parse::<u8>().ok()?;
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let maximum = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    (day >= 1 && day <= maximum).then(|| format!("{}-{}-{}", parts[2], parts[3], parts[4]))
}

fn insert_one(
    tx: &Transaction<'_>,
    journal: &str,
    matter: &str,
    kind: &str,
    payload: &str,
) -> StoreResult<()> {
    let id = format!("{journal}/{kind}");
    tx.execute(
        "INSERT INTO obligations(id,matter_id,predicate_kind,predicate_payload,required,status)
         VALUES(?1,?2,?3,?4,1,'open')",
        params![id, matter, kind, payload.as_bytes()],
    )?;
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
