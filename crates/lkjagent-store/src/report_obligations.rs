use rusqlite::{params, Transaction};
use serde_json::{json, Value};

use crate::error::{StoreError, StoreResult};
use crate::report_paths::{child_path, map_path, member_path, semantic_part, short_slug};
use crate::transactions::Effect;

const LONG_LIMIT: usize = 2_048;

pub(crate) fn insert(
    tx: &Transaction<'_>,
    effect: &Effect<'_>,
    matter: &str,
    path: &str,
    text: &str,
) -> StoreResult<()> {
    if let Some(slug) = short_slug(path) {
        let sources = lineage_rows(tx, effect)?
            .into_iter()
            .map(|(kind, revision)| format!("{kind}:{revision}"))
            .collect::<Vec<_>>();
        let payload = json!({
            "path":path,"kind":"report","semantic_key":slug,"slug":slug,
            "source_lineage":sources,"max_token_units":512
        });
        return crate::journal_obligations::insert_one(
            tx,
            &format!("{}/managed-report", effect.journal),
            matter,
            "managed-report",
            &payload.to_string(),
        );
    }
    if let Some((slug, children, minimum_words)) = map_payload(path, text) {
        let map = json!({
            "path":path,"kind":"report","slug":slug,"unit":"index",
            "children":children,"minimum_words":minimum_words,
            "max_token_units":LONG_LIMIT
        });
        upsert(
            tx,
            matter,
            &format!("report-map/{slug}"),
            "managed-report-map",
            &map,
        )?;
        for child in map["children"].as_array().into_iter().flatten() {
            let unit = child.as_str().ok_or_else(invalid)?;
            let payload = json!({
                "path":member_path(&slug, unit),"kind":"report","slug":slug,
                "unit":unit,"map_path":path,"max_token_units":LONG_LIMIT
            });
            upsert(
                tx,
                matter,
                &format!("report-member/{slug}/{unit}"),
                "managed-report-member",
                &payload,
            )?;
        }
        let payload = json!({
            "path":path,"kind":"report","slug":slug,"map_path":path,
            "children":children.iter().map(|unit| json!({"unit":unit,"path":member_path(&slug,unit)})).collect::<Vec<_>>(),
            "paths":std::iter::once(path.to_string()).chain(children.iter().map(|unit| member_path(&slug,unit))).collect::<Vec<_>>(),
            "minimum_words":minimum_words
        });
        return upsert(
            tx,
            matter,
            &format!("report-complete/{slug}"),
            "managed-report-complete",
            &payload,
        );
    }
    if child_path(path).is_some() {
        return Ok(());
    }
    Err(StoreError::InvalidState(
        "report path is not canonical".into(),
    ))
}

fn upsert(
    tx: &Transaction<'_>,
    matter: &str,
    suffix: &str,
    kind: &str,
    payload: &Value,
) -> StoreResult<()> {
    let id = format!("{matter}/{suffix}");
    tx.execute(
        "INSERT INTO obligations(id,matter_id,predicate_kind,predicate_payload,required,status)
         VALUES(?1,?2,?3,?4,1,'open') ON CONFLICT(id) DO NOTHING",
        params![id, matter, kind, payload.to_string().as_bytes()],
    )?;
    Ok(())
}

fn map_payload(path: &str, text: &str) -> Option<(String, Vec<String>, u32)> {
    let slug = map_path(path)?;
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() < 12
        || lines[0] != "---"
        || lines[1] != "kind: report"
        || lines[2] != format!("semantic-key: {slug}")
        || lines[3] != format!("slug: {slug}")
        || lines[4] != "unit: index"
        || !lines[5].starts_with("minimum-words: ")
        || lines[6] != "children:"
    {
        return None;
    }
    let minimum_words = lines[5].split_once(':')?.1.trim().parse().ok()?;
    let mut children = Vec::new();
    let mut index = 7;
    while let Some(line) = lines.get(index) {
        if *line == "source-lineage:" {
            break;
        }
        let unit = line.strip_prefix("- ")?;
        if !semantic_part(unit) || unit == "index" {
            return None;
        }
        children.push(unit.to_string());
        index += 1;
    }
    (children.len() >= 2 && index < lines.len()).then_some((
        slug.to_string(),
        children,
        minimum_words,
    ))
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

fn invalid() -> StoreError {
    StoreError::InvalidState("report topology payload is malformed".into())
}
