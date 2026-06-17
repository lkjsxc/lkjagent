use std::collections::BTreeMap;

use rusqlite::Connection;

use crate::dispatch::params::{param, parse_i64, parse_usize};
use crate::dispatch::{observe_error, observe_result, DispatchOutput, DispatchState, ToolRuntime};
use crate::queue;

pub fn dispatch_queue_list(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let filter = match queue::QueueFilter::parse(&param(params, "status")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    let limit = match parse_usize(&param(params, "limit")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        queue::list(conn, filter, limit),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_queue_enqueue(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        queue::enqueue(
            conn,
            &param(params, "content"),
            &param(params, "reason"),
            &runtime.now,
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_queue_edit(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let id = match parse_i64(&param(params, "id")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        queue::edit(
            conn,
            id,
            &param(params, "content"),
            &param(params, "reason"),
            &runtime.now,
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_queue_delete(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let id = match parse_i64(&param(params, "id")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        queue::delete(conn, id, &param(params, "reason"), &runtime.now),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_queue_redeliver(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let id = match parse_i64(&param(params, "id")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    let content = params.get("content").map(String::as_str);
    observe_result(
        queue::redeliver(conn, id, content, &param(params, "reason"), &runtime.now),
        action_text,
        runtime,
        state,
    )
}
