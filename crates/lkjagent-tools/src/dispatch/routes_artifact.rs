use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{observe_result, DispatchOutput, DispatchState, ToolRuntime};
use rusqlite::Connection;

pub fn dispatch_artifact_plan(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::artifact::plan(
            conn,
            &runtime.now,
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
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let root = param(params, "root");
    let title = param(params, "title");
    let kind = param(params, "kind");
    let mode = param(params, "mode");
    let sections = param(params, "sections");
    observe_result(
        crate::artifact::apply(crate::artifact::ApplyRequest {
            workspace: &runtime.workspace,
            conn,
            now: &runtime.now,
            root: &root,
            title: &title,
            kind: &kind,
            mode: &mode,
            sections: &sections,
        }),
        action_text,
        runtime,
        state,
    )
}

pub fn dispatch_artifact_audit(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::artifact::audit(
            &runtime.workspace,
            conn,
            &runtime.now,
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
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::artifact::next_with_cursor(
            &runtime.workspace,
            conn,
            &runtime.now,
            &param(params, "root"),
            &param(params, "kind"),
        ),
        action_text,
        runtime,
        state,
    )
}
