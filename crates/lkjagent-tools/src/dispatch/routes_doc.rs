use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{observe_result, DispatchOutput, DispatchState, ToolRuntime};

pub fn dispatch_doc_audit(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        crate::doc::audit(
            &runtime.workspace,
            &param(params, "root"),
            &param(params, "count"),
            &param(params, "mode"),
        ),
        action_text,
        runtime,
        state,
    )
}
