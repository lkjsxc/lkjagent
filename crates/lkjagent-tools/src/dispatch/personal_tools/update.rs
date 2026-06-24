use std::collections::BTreeMap;

use lkjagent_store::personal::{update as update_record, PersonalRecordUpdate};
use rusqlite::Connection;

use crate::dispatch::params::{param, parse_i64};
use crate::dispatch::ToolRuntime;
use crate::error::ToolResult;

pub fn status(
    conn: &Connection,
    params: &BTreeMap<String, String>,
    runtime: &ToolRuntime,
) -> ToolResult<String> {
    let id = parse_i64(&param(params, "id"))?;
    let update = PersonalRecordUpdate {
        id,
        title: opt(params, "title"),
        body: opt(params, "notes").or_else(|| opt(params, "details")),
        status: opt(params, "status"),
        tags: opt(params, "tags"),
        timezone: None,
        start_at: opt(params, "start"),
        end_at: opt(params, "end"),
        due_at: opt(params, "due"),
        recurrence: None,
        priority: opt(params, "priority"),
        project: opt(params, "project"),
        now: &runtime.now,
    };
    let changed = changed_fields(params);
    update_record(conn, &update)?;
    Ok(format!(
        "personal_record_updated\nid={id}\nchanged_fields={}",
        changed.join(",")
    ))
}

fn opt<'a>(params: &'a BTreeMap<String, String>, name: &str) -> Option<&'a str> {
    params
        .get(name)
        .filter(|value| !value.trim().is_empty())
        .map(String::as_str)
}

fn changed_fields(params: &BTreeMap<String, String>) -> Vec<String> {
    params
        .keys()
        .filter(|key| key.as_str() != "id")
        .cloned()
        .collect()
}
