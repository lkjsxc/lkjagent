use std::collections::BTreeMap;

use crate::dispatch::params::{param, parse_u64};
use crate::dispatch::{observe_error, observe_result, DispatchOutput, DispatchState, ToolRuntime};

pub fn dispatch_verify_cargo(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let timeout = match parse_u64(&param(params, "timeout")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        crate::verify::cargo(
            &runtime.workspace,
            &param(params, "gate"),
            &param(params, "package"),
            timeout,
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_verify_xtask(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let timeout = match parse_u64(&param(params, "timeout")) {
        Ok(value) => value,
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    observe_result(
        crate::verify::xtask(&runtime.workspace, &param(params, "gate"), timeout),
        action_text,
        runtime,
        state,
    )
}
