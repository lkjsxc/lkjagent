use lkjagent_core::runtime_operation::RuntimePhase;
use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_store::transactions::NativeStore;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;

const TOKEN_LIMIT: usize = 512;
type R<T> = Result<T, String>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundDecisionContext {
    pub schema: String,
    pub phase: RuntimePhase,
    pub selected_wall_time: String,
    pub workspace_timezone: String,
    pub local_date: String,
}

pub fn bind_context(
    phase: RuntimePhase,
    selected_wall_time: &str,
    timezone: &str,
) -> R<BoundDecisionContext> {
    Ok(BoundDecisionContext {
        schema: "decision-context-v2".into(),
        phase,
        selected_wall_time: selected_wall_time.into(),
        workspace_timezone: timezone.into(),
        local_date: crate::clock::local_date(selected_wall_time, timezone)?,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn dispatch(
    data: &Path,
    db: &Path,
    store: &mut NativeStore,
    matter: &str,
    decision: &str,
    entry: &lkjagent_core::runtime_decision::ToolViewEntry,
    args: &[(String, String)],
    raw: &str,
) -> R<String> {
    if store.effects_in_budget_epoch(matter).map_err(err)? >= 16 {
        return crate::public_loop::exhaust(store, matter, Some(decision), "effects", 16, 16, 0);
    }
    let get = |name: &str| {
        args.iter()
            .find(|item| item.0 == name)
            .map(|item| item.1.as_str())
    };
    let family = get("family").ok_or("record family missing")?;
    let title = get("title").ok_or("record title missing")?;
    let body = get("body").ok_or("record body missing")?;
    if family == "memory" {
        return crate::memory_record::dispatch(
            data, db, store, matter, decision, entry, raw, title, body,
        );
    }
    if family == "report" {
        return crate::report_record::dispatch(
            data, db, store, matter, decision, entry, raw, title, body,
        );
    }
    if family != "journal" {
        return Err("record family is not admitted".into());
    }
    crate::record_validation::authored("journal", title, body)?;
    let context = load_context(db, decision)?;
    let path = canonical_path(&context.local_date)?;
    let rows = source_rows(db, decision)?;
    if !rows.iter().any(|(kind, _)| kind == "owner") {
        return Err("journal source lineage has no current owner context".into());
    }
    let sources = rows
        .into_iter()
        .map(|(_, fingerprint)| fingerprint)
        .collect::<Vec<_>>();
    let rendered = render(&context.local_date, &sources, title, body);
    if crate::journal_checks::token_units(&rendered) > TOKEN_LIMIT {
        return Err("journal document exceeds 512 conservative token units".into());
    }
    let root = crate::config::workspace_root(data)?;
    let root = crate::workspace_root::open(&root)?;
    let workspace = OpenedWorkspace::open(&root).map_err(err)?;
    let prepared = crate::journal_apply::prepare(db, &workspace, &path, &rendered)?;
    crate::journal_apply::apply(
        db, store, &workspace, matter, decision, entry, raw, &path, prepared,
    )
}

fn load_context(db: &Path, decision: &str) -> R<BoundDecisionContext> {
    let connection = Connection::open(db).map_err(err)?;
    let (bytes, wall): (Vec<u8>, String) = connection
        .query_row(
            "SELECT d.context_spec,e.wall_time FROM runtime_decisions d JOIN runtime_events e ON e.id=d.event_id WHERE d.id=?1",
            [decision],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(err)?;
    let context: BoundDecisionContext = serde_json::from_slice(&bytes).map_err(err)?;
    if context.schema != "decision-context-v2" || context.selected_wall_time != wall {
        return Err("journal decision context is not bound to selection".into());
    }
    Ok(context)
}

pub(crate) fn source_rows(db: &Path, decision: &str) -> R<Vec<(String, String)>> {
    let connection = Connection::open(db).map_err(err)?;
    let mut query = connection
        .prepare(
            "SELECT source_kind,CAST(source_revision AS TEXT) FROM context_items WHERE decision_id=?1 ORDER BY rowid",
        )
        .map_err(err)?;
    let rows = query
        .query_map([decision], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(err)?;
    if rows.is_empty()
        || rows
            .iter()
            .any(|(kind, fp)| !lineage_value(kind) || !lineage_value(fp))
    {
        return Err("record source lineage is missing or unsafe".into());
    }
    Ok(rows)
}

fn lineage_value(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b':'))
}

fn canonical_path(date: &str) -> R<String> {
    let parts = date.split('-').collect::<Vec<_>>();
    if parts.len() != 3
        || [4, 2, 2]
            .iter()
            .zip(&parts)
            .any(|(n, part)| part.len() != *n || !part.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err("journal local date is malformed".into());
    }
    Ok(format!(
        "life/journal/{}/{}/{}/entry.md",
        parts[0], parts[1], parts[2]
    ))
}

fn render(date: &str, sources: &[String], title: &str, body: &str) -> String {
    let lineage = sources
        .iter()
        .map(|value| format!("- {value}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nkind: journal\ndate: {date}\nsource-fingerprints:\n{lineage}\n---\n# {title}\n\n{}\n",
        body.trim()
    )
}

fn err(value: impl std::fmt::Debug) -> String {
    format!("{value:?}")
}
