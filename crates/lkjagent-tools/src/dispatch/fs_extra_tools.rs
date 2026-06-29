use std::collections::BTreeMap;

use crate::dispatch::params::{param, parse_usize};
use crate::dispatch::{observe_error, observe_result, DispatchOutput, DispatchState, ToolRuntime};
use rusqlite::Connection;

pub fn dispatch_fs_list(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let depth = match parse_usize(&param(params, "depth")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    let limit = match parse_usize(&param(params, "limit")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        crate::fs_list::list(
            &runtime.workspace,
            &param(params, "path"),
            depth,
            &param(params, "kind"),
            limit,
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_fs_search(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let context = match parse_usize(&param(params, "context")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    let limit = match parse_usize(&param(params, "limit")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        crate::fs_search::search(
            &runtime.workspace,
            &param(params, "path"),
            &param(params, "query"),
            &param(params, "include"),
            &param(params, "case"),
            context,
            limit,
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_fs_stat(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::fs_stat::stat(&runtime.workspace, &param(params, "path")),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_fs_mkdir(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::fs_batch::mkdir(&runtime.workspace, &param(params, "path")),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_fs_batch_write(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let files = param(params, "files");
    let result = (|| {
        let paths = crate::fs_batch::paths(&files)?;
        crate::artifact_write_support::validate_paths_against_contract(conn, &paths)?;
        match crate::fs_batch::batch_write(&runtime.workspace, &files, 20) {
            Ok(output) => {
                crate::artifact_write_support::record_written_paths(conn, &paths, &runtime.now)?;
                crate::artifact_ledger_state::record_write_progress(conn, &paths, &runtime.now)?;
                Ok(output)
            }
            Err(error) => {
                crate::artifact_write_support::record_failed_paths(conn, &paths, &runtime.now)?;
                Err(error)
            }
        }
    })();
    observe_result(result, action_text, runtime, state)
}
