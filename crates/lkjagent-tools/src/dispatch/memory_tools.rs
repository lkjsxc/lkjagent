use std::collections::BTreeMap;

use rusqlite::Connection;

use crate::dispatch::params::{param, parse_usize};
use crate::dispatch::{observe_error, observe_result, DispatchOutput, DispatchState, ToolRuntime};
use crate::memory;

pub fn dispatch_memory_save(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        memory::save(
            conn,
            &param(params, "kind"),
            &param(params, "title"),
            params.get("tags").map_or("", String::as_str),
            &param(params, "content"),
            &runtime.now,
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_memory_find(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let limit = match parse_usize(&param(params, "limit")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        memory::find(conn, &param(params, "query"), limit),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_memory_prune(
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(memory::prune(conn), action_text, runtime, state)
}
