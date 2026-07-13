use lkjagent_core::runtime_context::{ContaminationClass, ContextItem, StalenessClass, TrustClass};
use rusqlite::Connection;
use std::{collections::BTreeSet, path::Path};

type R<T> = Result<T, String>;
const ITEM_LIMIT: usize = 4;
const ITEM_BYTES: usize = 1024;
const ITEM_UNITS: usize = 256;
const TOTAL_BYTES: usize = 2048;
const TOTAL_UNITS: usize = 512;

pub(crate) fn candidates(db: &Path, objective: &str) -> R<Vec<ContextItem>> {
    let connection = Connection::open(db).map_err(err)?;
    let mut query = connection
        .prepare(
            "SELECT CAST(d.current_path AS TEXT),r.id,r.content
         FROM workspace_documents d
         JOIN workspace_revisions r ON r.id=d.current_revision_id
         JOIN effect_journal e ON e.id=r.effect_id
         JOIN runtime_decisions rd ON rd.id=e.decision_id
         JOIN matters m ON m.id=rd.matter_id
         JOIN observations o ON o.id=e.observation_id
         WHERE d.status='active' AND d.managed=1
           AND CAST(d.current_path AS TEXT) GLOB 'knowledge/notes/*.md'
           AND e.status='settled' AND m.lifecycle='closed' AND o.contamination='clean'
         ORDER BY CAST(d.current_path AS TEXT),r.id",
        )
        .map_err(err)?;
    let rows = query
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })
        .map_err(err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(err)?;
    let corrections = correction_keys(objective);
    let mut seen = BTreeSet::new();
    let mut bytes = 0;
    let mut units = 0;
    let mut out = Vec::new();
    for (path, revision, content) in rows {
        let Some((key, body)) = parse(&path, &content) else {
            continue;
        };
        if corrections.contains(&key) || !seen.insert((key.clone(), revision.clone())) {
            continue;
        }
        let body = bounded(body, ITEM_BYTES, ITEM_UNITS);
        let cost = crate::journal_checks::token_units(&body);
        if out.len() == ITEM_LIMIT || bytes + body.len() > TOTAL_BYTES || units + cost > TOTAL_UNITS
        {
            break;
        }
        bytes += body.len();
        units += cost;
        out.push(ContextItem {
            id: format!("memory-{revision}"),
            semantic_key: key,
            body,
            source_type: "memory".into(),
            source_id: path,
            source_fingerprint: revision,
            trust: TrustClass::Memory,
            staleness: StalenessClass::Current,
            contamination: ContaminationClass::Clean,
            artifact_refs: vec![],
            decision_id: None,
            created_at: String::new(),
        });
    }
    Ok(out)
}

pub fn correction_keys(objective: &str) -> BTreeSet<String> {
    objective
        .lines()
        .filter_map(|line| {
            let line = line.trim().to_ascii_lowercase();
            let rest = line
                .strip_prefix("forget ")
                .or_else(|| line.strip_prefix("correct "))?;
            let key = rest.split_once(':')?.0.trim();
            canonical_key(key).then(|| key.to_string())
        })
        .collect()
}

fn parse(path: &str, bytes: &[u8]) -> Option<(String, String)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let slug = path.strip_prefix("knowledge/notes/")?.strip_suffix(".md")?;
    if !canonical_key(slug) || prohibited(text) {
        return None;
    }
    let prefix =
        format!("---\nkind: memory\nsemantic-key: {slug}\nslug: {slug}\nsource-fingerprints:\n");
    let rest = text.strip_prefix(&prefix)?;
    let (_, body) = rest.split_once("\n---\n")?;
    (!body.trim().is_empty()).then(|| (slug.to_string(), body.to_string()))
}

fn canonical_key(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 80
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.as_bytes().windows(2).any(|pair| pair == b"--")
}

fn bounded(mut value: String, bytes: usize, units: usize) -> String {
    while value.len() > bytes || crate::journal_checks::token_units(&value) > units {
        value.pop();
    }
    value
}

fn prohibited(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "password:",
        "password=",
        "secret:",
        "secret=",
        "token:",
        "token=",
        "api_key:",
        "api_key=",
        "authorization: bearer",
        "model-output-rejected",
        "raw harness json",
    ]
    .iter()
    .any(|pattern| lower.contains(pattern))
}

fn err(value: impl std::fmt::Debug) -> String {
    format!("{value:?}")
}
