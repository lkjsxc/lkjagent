use std::collections::BTreeMap;

use crate::dispatch::guards::guard_write_path;
use crate::dispatch::params::{param, parse_usize};
use crate::dispatch::{observe_error, observe_result};
use crate::dispatch::{DispatchOutput, DispatchState, ToolRuntime};
use crate::{fs, fs_tree};

pub fn dispatch_fs_read_many(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let start = match parse_usize(&param(params, "start")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    let count = match parse_usize(&param(params, "count")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    let total = match parse_usize(&param(params, "total")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        fs::read_many(
            &runtime.workspace,
            &param(params, "paths"),
            start,
            count,
            total,
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_fs_patch(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let path = param(params, "path");
    if let Err(error) = guard_write_path(state.control.guard, &path) {
        return observe_error(error, action_text, runtime, state);
    }
    let result = fs::patch(&runtime.workspace, &path, &param(params, "patch"))
        .map(|report| fs::patch_observation(&report));
    observe_result(result, action_text, runtime, state)
}

pub fn dispatch_fs_tree(
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
        fs_tree::tree(&runtime.workspace, &param(params, "path"), depth, limit),
        action_text,
        runtime,
        state,
    )
}
