use std::collections::BTreeMap;

use lkjagent_store::personal::{create, PersonalRecordInput};
use rusqlite::Connection;

use crate::dispatch::params::param;
use crate::dispatch::ToolRuntime;
use crate::error::ToolResult;

pub fn diary(
    conn: &Connection,
    params: &BTreeMap<String, String>,
    runtime: &ToolRuntime,
) -> ToolResult<String> {
    let input = PersonalRecordInput {
        kind: "diary",
        title: &param(params, "title"),
        body: &param(params, "content"),
        status: "open",
        tags: params.get("tags").map_or("", String::as_str),
        timezone: None,
        start_at: params.get("date").map(String::as_str),
        end_at: None,
        due_at: None,
        recurrence: None,
        priority: None,
        project: None,
        source_case_id: None,
        now: &runtime.now,
    };
    created(conn, &input)
}

pub fn schedule(
    conn: &Connection,
    params: &BTreeMap<String, String>,
    runtime: &ToolRuntime,
) -> ToolResult<String> {
    let start = param(params, "start");
    let input = PersonalRecordInput {
        kind: "schedule",
        title: &param(params, "title"),
        body: params.get("notes").map_or("", String::as_str),
        status: "open",
        tags: params.get("tags").map_or("", String::as_str),
        timezone: params.get("timezone").map(String::as_str),
        start_at: Some(start.as_str()),
        end_at: params.get("end").map(String::as_str),
        due_at: None,
        recurrence: params.get("recurrence").map(String::as_str),
        priority: None,
        project: None,
        source_case_id: None,
        now: &runtime.now,
    };
    created(conn, &input)
}

pub fn todo(
    conn: &Connection,
    params: &BTreeMap<String, String>,
    runtime: &ToolRuntime,
) -> ToolResult<String> {
    let input = PersonalRecordInput {
        kind: "todo",
        title: &param(params, "title"),
        body: params.get("details").map_or("", String::as_str),
        status: "open",
        tags: params.get("tags").map_or("", String::as_str),
        timezone: None,
        start_at: None,
        end_at: None,
        due_at: params.get("due").map(String::as_str),
        recurrence: None,
        priority: params.get("priority").map(String::as_str),
        project: params.get("project").map(String::as_str),
        source_case_id: None,
        now: &runtime.now,
    };
    created(conn, &input)
}

fn created(conn: &Connection, input: &PersonalRecordInput<'_>) -> ToolResult<String> {
    let id = create(conn, input)?;
    Ok(format!(
        "personal_record_created\nid={id}\nkind={}\ntitle={}\nstatus={}\nstart_at={}\ndue_at={}",
        input.kind,
        one_line(input.title),
        input.status,
        input.start_at.unwrap_or("none"),
        input.due_at.unwrap_or("none")
    ))
}

fn one_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
