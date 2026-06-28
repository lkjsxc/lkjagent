use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{
    observe_result, DispatchOutput, DispatchState, GraphEvidenceRecord, ToolRuntime,
};
use rusqlite::Connection;

pub fn dispatch_artifact_plan(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let root = param(params, "root");
    let kind = param(params, "kind");
    let result =
        crate::artifact_address_support::ensure_plan_root(&runtime.workspace, &root, &kind)
            .and_then(|()| {
                crate::artifact::plan(
                    conn,
                    &runtime.now,
                    &root,
                    &param(params, "title"),
                    &kind,
                    &param(params, "scale"),
                    &param(params, "sections"),
                )
            });
    observe_result(result, action_text, runtime, state)
}

pub fn dispatch_artifact_audit(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_artifact_result(
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
    observe_artifact_result(
        crate::artifact::next_with_cursor(
            &runtime.workspace,
            conn,
            &runtime.now,
            &param(params, "root"),
            &param(params, "path"),
            &param(params, "kind"),
        ),
        action_text,
        runtime,
        state,
    )
}

fn observe_artifact_result(
    result: crate::error::ToolResult<String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let output = observe_result(result, action_text, runtime, state);
    if output.content.contains("address_status=") {
        state.graph_evidence.push(GraphEvidenceRecord {
            kind: "tool-address-refusal".to_string(),
            summary: output.content.clone(),
            path: None,
            frame_ref: output.frame_ref,
        });
    }
    output
}
