use std::collections::BTreeMap;

use lkjagent_store::personal::{list, search, PersonalListFilter, PersonalRecord};
use rusqlite::Connection;

use crate::dispatch::params::{param, parse_usize};
use crate::dispatch::{observe_error, observe_result, DispatchOutput, DispatchState, ToolRuntime};
use crate::error::ToolResult;

pub fn dispatch(
    conn: &Connection,
    params: &BTreeMap<String, String>,
    kind: &str,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    match find(conn, params, kind) {
        Ok(records) => observe_result(Ok(render(&records)), action_text, runtime, state),
        Err(error) => observe_error(error, action_text, runtime, state),
    }
}

fn find(
    conn: &Connection,
    params: &BTreeMap<String, String>,
    kind: &str,
) -> ToolResult<Vec<PersonalRecord>> {
    let limit = parse_usize(&param(params, "limit"))?;
    if let Some(query) = params.get("query").filter(|value| !value.trim().is_empty()) {
        return Ok(search(conn, query, limit)?
            .into_iter()
            .filter(|record| record.kind == kind)
            .collect());
    }
    let status = params.get("status").filter(|value| value.as_str() != "all");
    Ok(list(
        conn,
        &PersonalListFilter {
            kind: Some(kind),
            status: status.map(String::as_str),
            project: params.get("project").map(String::as_str),
            start: params.get("start").map(String::as_str),
            end: params.get("end").map(String::as_str),
            limit,
        },
    )?)
}

fn render(records: &[PersonalRecord]) -> String {
    let mut lines = vec![format!("personal_records\nreturned={}", records.len())];
    for record in records {
        lines.push(format!(
            "- id={} kind={} status={} title={} start_at={} due_at={}",
            record.id,
            record.kind,
            record.status,
            one_line(&record.title),
            record.start_at.as_deref().unwrap_or("none"),
            record.due_at.as_deref().unwrap_or("none")
        ));
    }
    lines.join("\n")
}

fn one_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
