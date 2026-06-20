use rusqlite::Connection;

use crate::control;
use crate::dispatch::fs_extra_tools::{
    dispatch_fs_batch_write, dispatch_fs_list, dispatch_fs_mkdir, dispatch_fs_search,
    dispatch_fs_stat,
};
use crate::dispatch::fs_tools::{
    dispatch_fs_edit, dispatch_fs_read, dispatch_fs_write, dispatch_shell,
};
use crate::dispatch::graph_evidence_tools::{dispatch_graph_compact, dispatch_graph_evidence};
use crate::dispatch::graph_tools::{
    dispatch_graph_context, dispatch_graph_note, dispatch_graph_plan, dispatch_graph_state,
    dispatch_graph_transition,
};
use crate::dispatch::memory_tools::{dispatch_memory_find, dispatch_memory_save};
use crate::dispatch::queue_tools::{
    dispatch_queue_delete, dispatch_queue_edit, dispatch_queue_enqueue, dispatch_queue_list,
    dispatch_queue_redeliver,
};
use crate::dispatch::routes_doc::{dispatch_doc_audit, dispatch_doc_scaffold};
use crate::dispatch::routes_verify::{dispatch_verify_cargo, dispatch_verify_xtask};
use crate::dispatch::routes_workspace::dispatch_workspace_summary;
use crate::dispatch::validate::ValidatedAction;
use crate::dispatch::{finish, observe_result, DispatchOutput, DispatchState, ToolRuntime};
use crate::observe;

pub fn route(
    action: ValidatedAction,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    match action.tool.as_str() {
        "fs.read" => dispatch_fs_read(&action.params, action_text, runtime, state),
        "fs.write" => dispatch_fs_write(&action.params, action_text, runtime, state),
        "fs.edit" => dispatch_fs_edit(&action.params, action_text, runtime, state),
        "fs.list" => dispatch_fs_list(&action.params, action_text, runtime, state),
        "fs.search" => dispatch_fs_search(&action.params, action_text, runtime, state),
        "fs.stat" => dispatch_fs_stat(&action.params, action_text, runtime, state),
        "fs.mkdir" => dispatch_fs_mkdir(&action.params, action_text, runtime, state),
        "fs.batch_write" => dispatch_fs_batch_write(&action.params, action_text, runtime, state),
        "shell.run" => dispatch_shell(&action.params, action_text, runtime, state),
        "queue.list" => dispatch_queue_list(&action.params, action_text, runtime, conn, state),
        "queue.enqueue" => {
            dispatch_queue_enqueue(&action.params, action_text, runtime, conn, state)
        }
        "queue.edit" => dispatch_queue_edit(&action.params, action_text, runtime, conn, state),
        "queue.delete" => dispatch_queue_delete(&action.params, action_text, runtime, conn, state),
        "queue.redeliver" => {
            dispatch_queue_redeliver(&action.params, action_text, runtime, conn, state)
        }
        "memory.save" => dispatch_memory_save(&action.params, action_text, runtime, conn, state),
        "memory.find" => dispatch_memory_find(&action.params, action_text, runtime, conn, state),
        "graph.state" => dispatch_graph_state(action_text, runtime, state),
        "graph.plan" => dispatch_graph_plan(&action.params, action_text, runtime, state),
        "graph.transition" => {
            dispatch_graph_transition(&action.params, action_text, runtime, state)
        }
        "graph.context" => dispatch_graph_context(&action.params, action_text, runtime, state),
        "graph.note" => dispatch_graph_note(&action.params, action_text, runtime, state),
        "graph.evidence" => dispatch_graph_evidence(&action.params, action_text, runtime, state),
        "graph.compact" => dispatch_graph_compact(action_text, runtime, state),
        "workspace.summary" => {
            dispatch_workspace_summary(&action.params, action_text, runtime, state)
        }
        "verify.cargo" => dispatch_verify_cargo(&action.params, action_text, runtime, state),
        "verify.xtask" => dispatch_verify_xtask(&action.params, action_text, runtime, state),
        "doc.scaffold" => dispatch_doc_scaffold(&action.params, action_text, runtime, state),
        "doc.audit" => dispatch_doc_audit(&action.params, action_text, runtime, state),
        "agent.done" => dispatch_done(&action.params, action_text, runtime, state),
        "agent.ask" => observe_result(
            control::ask(
                &mut state.control,
                &crate::dispatch::params::param(&action.params, "question"),
            ),
            action_text,
            runtime,
            state,
        ),
        other => finish(
            state,
            action_text,
            observe::notice("error", format!("unknown tool after validation: {other}")),
        ),
    }
}

fn dispatch_done(
    params: &std::collections::BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    if state.graph_state.is_some() && !state.graph_completion_ready {
        return observe_result(
            Err(crate::error::ToolError::invalid(done_refusal(
                &state.graph_missing,
            ))),
            action_text,
            runtime,
            state,
        );
    }
    observe_result(
        control::done(
            &mut state.control,
            &runtime.workspace,
            &crate::dispatch::params::param(params, "summary"),
        ),
        action_text,
        runtime,
        state,
    )
}

fn done_refusal(missing: &[String]) -> String {
    let listed = missing.join(", ");
    let first = missing
        .first()
        .cloned()
        .unwrap_or_else(|| "required-evidence".to_string());
    if first == "plan" {
        return format!(
            "graph completion refused; missing: {listed}; next action: graph.plan with steps, checks, paths, and reason"
        );
    }
    format!(
        "graph completion refused; missing: {listed}; next action: graph.evidence kind={first} summary=observed verification path=."
    )
}
