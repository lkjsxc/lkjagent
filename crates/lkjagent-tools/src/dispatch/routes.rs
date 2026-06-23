use rusqlite::Connection;

use crate::control;
use crate::dispatch::fs_extra_tools::{
    dispatch_fs_batch_write, dispatch_fs_list, dispatch_fs_mkdir, dispatch_fs_search,
    dispatch_fs_stat,
};
use crate::dispatch::fs_more_tools::{dispatch_fs_patch, dispatch_fs_read_many, dispatch_fs_tree};
use crate::dispatch::fs_tools::{
    dispatch_fs_edit, dispatch_fs_read, dispatch_fs_write, dispatch_shell,
};
use crate::dispatch::graph_evidence_tools::{dispatch_graph_compact, dispatch_graph_evidence};
use crate::dispatch::graph_inspect_tools::{
    dispatch_graph_audit, dispatch_graph_next, dispatch_graph_recover,
};
use crate::dispatch::graph_note_tools::dispatch_graph_note;
use crate::dispatch::graph_tools::{
    dispatch_graph_context, dispatch_graph_plan, dispatch_graph_state, dispatch_graph_transition,
};
use crate::dispatch::memory_tools::{
    dispatch_memory_find, dispatch_memory_prune, dispatch_memory_save,
};
use crate::dispatch::queue_tools::{
    dispatch_queue_delete, dispatch_queue_edit, dispatch_queue_enqueue, dispatch_queue_list,
    dispatch_queue_redeliver,
};
use crate::dispatch::routes_artifact::{
    dispatch_artifact_apply, dispatch_artifact_audit, dispatch_artifact_next,
    dispatch_artifact_plan,
};
use crate::dispatch::routes_doc::{dispatch_doc_audit, dispatch_doc_scaffold};
use crate::dispatch::routes_verify::{dispatch_verify_cargo, dispatch_verify_xtask};
use crate::dispatch::routes_workspace::{dispatch_workspace_index, dispatch_workspace_summary};
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
        "fs.read_many" => dispatch_fs_read_many(&action.params, action_text, runtime, state),
        "fs.write" => dispatch_fs_write(&action.params, action_text, runtime, conn, state),
        "fs.edit" => dispatch_fs_edit(&action.params, action_text, runtime, state),
        "fs.patch" => dispatch_fs_patch(&action.params, action_text, runtime, state),
        "fs.list" => dispatch_fs_list(&action.params, action_text, runtime, state),
        "fs.tree" => dispatch_fs_tree(&action.params, action_text, runtime, state),
        "fs.search" => dispatch_fs_search(&action.params, action_text, runtime, state),
        "fs.stat" => dispatch_fs_stat(&action.params, action_text, runtime, state),
        "fs.mkdir" => dispatch_fs_mkdir(&action.params, action_text, runtime, state),
        "fs.batch_write" => {
            dispatch_fs_batch_write(&action.params, action_text, runtime, conn, state)
        }
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
        "memory.prune" => dispatch_memory_prune(action_text, runtime, conn, state),
        "graph.state" => dispatch_graph_state(action_text, runtime, state),
        "graph.next" => dispatch_graph_next(action_text, runtime, state),
        "graph.audit" => dispatch_graph_audit(action_text, runtime, state),
        "graph.recover" => dispatch_graph_recover(action_text, runtime, state),
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
        "workspace.index" => dispatch_workspace_index(&action.params, action_text, runtime, state),
        "verify.cargo" => dispatch_verify_cargo(&action.params, action_text, runtime, state),
        "verify.xtask" => dispatch_verify_xtask(&action.params, action_text, runtime, state),
        "doc.scaffold" => dispatch_doc_scaffold(&action.params, action_text, runtime, state),
        "doc.audit" => dispatch_doc_audit(&action.params, action_text, runtime, state),
        "artifact.plan" => {
            dispatch_artifact_plan(&action.params, action_text, runtime, conn, state)
        }
        "artifact.apply" => {
            dispatch_artifact_apply(&action.params, action_text, runtime, conn, state)
        }
        "artifact.audit" => {
            dispatch_artifact_audit(&action.params, action_text, runtime, conn, state)
        }
        "artifact.next" => {
            dispatch_artifact_next(&action.params, action_text, runtime, conn, state)
        }
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
            Err(crate::error::ToolError::invalid(done_refusal(state))),
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

fn done_refusal(state: &DispatchState) -> String {
    let listed = state.graph_missing.join(", ");
    let first = state
        .graph_missing
        .first()
        .cloned()
        .unwrap_or_else(|| "required-evidence".to_string());
    let next = next_completion_action(&first);
    let graph_line = state
        .graph_state
        .as_deref()
        .and_then(|text| text.lines().find(|line| !line.trim().is_empty()))
        .unwrap_or("graph_state=unavailable");
    format!(
        "graph completion refused\npartial_handoff=blocked-with-evidence\nattempted=agent.done\nfailed_gate=completion\nmissing={listed}\nexisting_graph={graph_line}\nnext_executable_action={}\nvalid_example:\n{}",
        next.label, next.example
    )
}

struct CompletionNextAction {
    label: &'static str,
    example: String,
}

fn next_completion_action(first: &str) -> CompletionNextAction {
    if first == "artifact-readiness" {
        return CompletionNextAction {
            label: "run artifact.audit before retrying agent.done",
            example:
                "<act>\n<tool>artifact.audit</tool>\n<root>stories/example-story</root>\n</act>"
                    .to_string(),
        };
    }
    if first == "document-structure" {
        return CompletionNextAction {
            label: "run doc.audit before retrying agent.done",
            example: "<act>\n<tool>doc.audit</tool>\n<root>docs</root>\n</act>".to_string(),
        };
    }
    if first == "plan" {
        return CompletionNextAction {
            label: "record graph.plan with steps, checks, paths, and reason",
            example: "<act>\n<tool>graph.plan</tool>\n<objective>Finish the owner task</objective>\n<steps>inspect current state; run verification; record evidence</steps>\n<checks>verification evidence exists before completion</checks>\n<paths>.</paths>\n<reason>completion gate is missing plan evidence</reason>\n</act>".to_string(),
        };
    }
    CompletionNextAction {
        label: "record missing graph.evidence before retrying agent.done",
        example: format!(
            "<act>\n<tool>graph.evidence</tool>\n<kind>{first}</kind>\n<summary>Observed required completion evidence</summary>\n<path>.</path>\n</act>"
        ),
    }
}
