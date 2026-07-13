use rusqlite::Transaction;
use serde_json::json;

use crate::error::{StoreError, StoreResult};
use crate::transactions::Effect;

pub(crate) fn insert(
    tx: &Transaction<'_>,
    effect: &Effect<'_>,
    matter: &str,
    path: &str,
    text: &str,
) -> StoreResult<()> {
    if text.starts_with("---\nkind: journal\n") {
        insert_journal(tx, effect, matter, path)
    } else if text.starts_with("---\nkind: memory\n") {
        insert_memory(tx, effect, matter, path)
    } else if text.starts_with("---\nkind: report\n") {
        crate::report_obligations::insert(tx, effect, matter, path, text)
    } else {
        Err(StoreError::InvalidState(
            "record kind is not admitted".into(),
        ))
    }
}

fn insert_journal(
    tx: &Transaction<'_>,
    effect: &Effect<'_>,
    matter: &str,
    path: &str,
) -> StoreResult<()> {
    let date = journal_date(path)
        .ok_or_else(|| StoreError::InvalidState("journal path is not canonical".into()))?;
    let rows = lineage_rows(tx, effect)?;
    if !rows.iter().any(|(kind, _)| kind == "owner") {
        return Err(StoreError::InvalidState(
            "journal has no current owner lineage".into(),
        ));
    }
    let payload = json!({
        "path":path,"kind":"journal","date":date,
        "source_fingerprints":rows.into_iter().map(|(_, revision)| revision).collect::<Vec<_>>(),
        "max_token_units":512
    });
    insert_one(tx, effect, matter, "managed-journal", payload)
}

fn insert_memory(
    tx: &Transaction<'_>,
    effect: &Effect<'_>,
    matter: &str,
    path: &str,
) -> StoreResult<()> {
    let slug = record_slug(path, "knowledge/notes/")
        .ok_or_else(|| StoreError::InvalidState("memory path is not canonical".into()))?;
    let sources = lineage_rows(tx, effect)?
        .into_iter()
        .filter_map(|(kind, revision)| (kind == "owner").then_some(revision))
        .collect::<Vec<_>>();
    if sources.is_empty() {
        return Err(StoreError::InvalidState(
            "memory has no current owner lineage".into(),
        ));
    }
    let payload = json!({
        "path":path,"kind":"memory","semantic_key":slug,"slug":slug,
        "source_fingerprints":sources,"max_token_units":512
    });
    insert_one(tx, effect, matter, "managed-memory", payload)
}

fn insert_one(
    tx: &Transaction<'_>,
    effect: &Effect<'_>,
    matter: &str,
    kind: &str,
    payload: serde_json::Value,
) -> StoreResult<()> {
    crate::journal_obligations::insert_one(
        tx,
        &format!("{}/{}", effect.journal, kind),
        matter,
        kind,
        &payload.to_string(),
    )
}

fn lineage_rows(tx: &Transaction<'_>, effect: &Effect<'_>) -> StoreResult<Vec<(String, String)>> {
    let mut query = tx.prepare(
        "SELECT source_kind,CAST(source_revision AS TEXT) FROM context_items
         WHERE decision_id=?1 ORDER BY rowid",
    )?;
    let rows = query
        .query_map([effect.decision], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err(StoreError::InvalidState(
            "record has no selected source lineage".into(),
        ));
    }
    Ok(rows)
}

fn record_slug<'a>(path: &'a str, prefix: &str) -> Option<&'a str> {
    path.strip_prefix(prefix)
        .and_then(|value| value.strip_suffix(".md"))
        .filter(|value| semantic_slug(value))
}

fn semantic_slug(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 80
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.as_bytes().windows(2).any(|pair| pair == b"--")
}

fn journal_date(path: &str) -> Option<String> {
    let parts = path.split('/').collect::<Vec<_>>();
    if parts.len() != 6
        || parts[0..2] != ["life", "journal"]
        || parts[5] != "entry.md"
        || [4, 2, 2].iter().zip(&parts[2..5]).any(|(length, part)| {
            part.len() != *length || !part.bytes().all(|b| b.is_ascii_digit())
        })
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
