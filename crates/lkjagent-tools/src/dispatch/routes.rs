use rusqlite::Connection;

use crate::control;
use crate::dispatch::fs_tools::{
    dispatch_fs_edit, dispatch_fs_read, dispatch_fs_write, dispatch_shell,
};
use crate::dispatch::memory_tools::{dispatch_memory_find, dispatch_memory_save};
use crate::dispatch::queue_tools::{
    dispatch_queue_delete, dispatch_queue_edit, dispatch_queue_enqueue, dispatch_queue_list,
    dispatch_queue_redeliver,
};
use crate::dispatch::skill_tools::dispatch_skill_use;
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
        "skill.use" => dispatch_skill_use(&action.params, action_text, runtime, state),
        "agent.done" => observe_result(
            control::done(
                &mut state.control,
                &runtime.workspace,
                &crate::dispatch::params::param(&action.params, "summary"),
            ),
            action_text,
            runtime,
            state,
        ),
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
