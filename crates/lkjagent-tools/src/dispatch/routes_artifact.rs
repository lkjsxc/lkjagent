use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{observe_result, DispatchOutput, DispatchState, ToolRuntime};

pub fn dispatch_artifact_plan(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::artifact::plan(
            &param(params, "root"),
            &param(params, "title"),
            &param(params, "kind"),
            &param(params, "scale"),
            &param(params, "sections"),
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_artifact_apply(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::artifact::apply(
            &runtime.workspace,
            &param(params, "root"),
            &param(params, "title"),
            &param(params, "kind"),
            &param(params, "mode"),
            &param(params, "sections"),
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_artifact_audit(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::artifact::audit(
            &runtime.workspace,
            &param(params, "root"),
            &param(params, "kind"),
            &param(params, "count"),
            &param(params, "mode"),
        ),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_artifact_next(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::artifact::next(
            &runtime.workspace,
            &param(params, "root"),
            &param(params, "kind"),
        ),
        action_text,
        runtime,
        state,
    )
}
