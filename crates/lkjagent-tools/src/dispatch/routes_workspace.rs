use std::collections::BTreeMap;

use crate::dispatch::params::{param, parse_usize};
use crate::dispatch::{observe_error, observe_result, DispatchOutput, DispatchState, ToolRuntime};

pub fn dispatch_workspace_summary(
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
        crate::workspace::summary(&runtime.workspace, &param(params, "path"), depth, limit),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_workspace_index(
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
        crate::workspace::index(&runtime.workspace, &param(params, "path"), depth, limit),
        action_text,
        runtime,
        state,
    )
}
