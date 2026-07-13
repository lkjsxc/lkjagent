use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_store::transactions::NativeStore;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;
use std::path::Path;

type R<T> = Result<T, String>;
const SHORT_LIMIT: usize = 512;
const LONG_LIMIT: usize = 2_048;

#[allow(clippy::too_many_arguments)]
pub(crate) fn dispatch(
    data: &Path,
    db: &Path,
    store: &mut NativeStore,
    matter: &str,
    decision: &str,
    entry: &lkjagent_core::runtime_decision::ToolViewEntry,
    raw: &str,
    args: &[(String, String)],
) -> R<String> {
    let get = |name: &str| {
        args.iter()
            .find(|item| item.0 == name)
            .map(|item| item.1.as_str())
    };
    let title = get("title").ok_or("record title missing")?;
    let body = get("body").ok_or("record body missing")?;
    crate::record_validation::authored("report", title, body)?;
    let sources = crate::journal_dispatch::source_rows(db, decision)?;
    let (path, rendered, limit, mode) = match crate::report_shape::parse(args)? {
        crate::report_shape::Shape::Short { slug } => (
            format!("artifacts/reports/{slug}.md"),
            crate::report_shape::short(&slug, &sources, title, body),
            SHORT_LIMIT,
            0o644,
        ),
        crate::report_shape::Shape::Map {
            slug,
            children,
            minimum_words,
        } => {
            map_guard(db, matter, &slug, &children, minimum_words)?;
            (
                format!("artifacts/documents/{slug}/README.md"),
                crate::report_shape::map(&slug, &sources, title, body, &children, minimum_words),
                LONG_LIMIT,
                0o600,
            )
        }
        crate::report_shape::Shape::Child { slug, unit } => {
            child_guard(db, matter, &slug, &unit)?;
            (
                format!("artifacts/documents/{slug}/{unit}.md"),
                crate::report_shape::child(&slug, &unit, &sources, title, body),
                LONG_LIMIT,
                0o644,
            )
        }
    };
    if crate::journal_checks::token_units(&rendered) > limit {
        return Err(format!(
            "report document exceeds {limit} conservative token units"
        ));
    }
    let root = crate::config::workspace_root(data)?;
    let root = crate::workspace_root::open(&root)?;
    let workspace = OpenedWorkspace::open(&root).map_err(err)?;
    let prepared = crate::journal_apply::prepare_mode(db, &workspace, &path, &rendered, mode)?;
    crate::journal_apply::apply(
        db, store, &workspace, matter, decision, entry, raw, &path, prepared,
    )
}

fn map_guard(
    db: &Path,
    matter: &str,
    slug: &str,
    children: &[String],
    minimum_words: u32,
) -> R<()> {
    let connection = Connection::open(db).map_err(err)?;
    let row: Option<String> = connection.query_row(
        "SELECT CAST(predicate_payload AS TEXT) FROM obligations WHERE matter_id=?1 AND required=1 AND predicate_kind='managed-report-map' AND json_extract(CAST(predicate_payload AS TEXT),'$.slug')=?2",
        params![matter, slug], |row| row.get(0)).optional().map_err(err)?;
    let Some(row) = row else { return Ok(()) };
    let value: Value = serde_json::from_str(&row).map_err(err)?;
    let same = value["children"].as_array().is_some_and(|items| {
        items
            .iter()
            .filter_map(Value::as_str)
            .eq(children.iter().map(String::as_str))
    }) && value["minimum_words"].as_u64() == Some(u64::from(minimum_words));
    if !same {
        return Err("report topology change is not admitted".into());
    }
    let passed: i64 = connection.query_row(
        "SELECT count(*) FROM obligations o JOIN checks c ON c.id=o.current_check_id WHERE o.matter_id=?1 AND o.required=1 AND o.predicate_kind='managed-report-map' AND json_extract(CAST(o.predicate_payload AS TEXT),'$.slug')=?2 AND o.status='passed' AND c.current=1 AND c.passed=1",
        params![matter, slug], |row| row.get(0)).map_err(err)?;
    (passed == 0)
        .then_some(())
        .ok_or("report map replacement is not admitted".into())
}

fn child_guard(db: &Path, matter: &str, slug: &str, unit: &str) -> R<()> {
    let connection = Connection::open(db).map_err(err)?;
    let map: i64 = connection.query_row(
        "SELECT count(*) FROM obligations o JOIN checks c ON c.id=o.current_check_id WHERE o.matter_id=?1 AND o.required=1 AND o.predicate_kind='managed-report-map' AND json_extract(CAST(o.predicate_payload AS TEXT),'$.slug')=?2 AND o.status='passed' AND c.current=1 AND c.passed=1",
        params![matter, slug], |row| row.get(0)).map_err(err)?;
    let member: i64 = connection.query_row(
        "SELECT count(*) FROM obligations WHERE matter_id=?1 AND required=1 AND predicate_kind='managed-report-member' AND json_extract(CAST(predicate_payload AS TEXT),'$.slug')=?2 AND json_extract(CAST(predicate_payload AS TEXT),'$.unit')=?3",
        params![matter, slug, unit], |row| row.get(0)).map_err(err)?;
    (map == 1 && member == 1)
        .then_some(())
        .ok_or("report child does not match a pending current map".into())
}

fn err(value: impl std::fmt::Debug) -> String {
    format!("{value:?}")
}
