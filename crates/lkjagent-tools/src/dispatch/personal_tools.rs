#[path = "personal_tools/create.rs"]
mod create;
#[path = "personal_tools/list.rs"]
mod list;
#[path = "personal_tools/update.rs"]
mod update;

use std::collections::BTreeMap;

use rusqlite::Connection;

use crate::dispatch::{observe_result, DispatchOutput, DispatchState, ToolRuntime};

pub fn dispatch_diary_record(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        create::diary(conn, params, runtime),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_diary_find(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    list::dispatch(conn, params, "diary", action_text, runtime, state)
}

pub fn dispatch_schedule_add(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        create::schedule(conn, params, runtime),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_schedule_list(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    list::dispatch(conn, params, "schedule", action_text, runtime, state)
}

pub fn dispatch_schedule_update(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        update::status(conn, params, runtime),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_todo_add(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        create::todo(conn, params, runtime),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_todo_list(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    list::dispatch(conn, params, "todo", action_text, runtime, state)
}

pub fn dispatch_todo_update(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        update::status(conn, params, runtime),
        action_text,
        runtime,
        state,
    )
}
