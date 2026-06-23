use std::collections::BTreeMap;

use crate::dispatch::guards::{guard_shell_command, guard_write_path};
use crate::dispatch::params::{param, parse_u64, parse_usize};
use crate::dispatch::{finish, observe_error, observe_result};
use crate::dispatch::{DispatchOutput, DispatchState, ReadRecord, ToolRuntime};
use crate::error::ToolError;
use crate::observe;
use crate::{fs, shell};
use rusqlite::Connection;

pub fn dispatch_fs_read(
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
    match fs::read(&runtime.workspace, &param(params, "path"), start, count) {
        Err(error) => observe_error(error, action_text, runtime, state),
        Ok(read) => finish_read(read, action_text, runtime, state),
    }
}

pub fn dispatch_fs_write(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let path = param(params, "path");
    if let Err(error) = guard_write_path(state.control.guard, &path) {
        return observe_error(error, action_text, runtime, state);
    }
    let content = param(params, "content");
    let result = fs::write(&runtime.workspace, &path, &content).and_then(|output| {
        let paths = vec![path.clone()];
        crate::artifact_write_support::record_written_paths(conn, &paths, &runtime.now)?;
        Ok(output)
    });
    observe_result(result, action_text, runtime, state)
}

pub fn dispatch_fs_edit(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let path = param(params, "path");
    if let Err(error) = guard_write_path(state.control.guard, &path) {
        return observe_error(error, action_text, runtime, state);
    }
    let result = fs::edit(
        &runtime.workspace,
        &path,
        &param(params, "find"),
        &param(params, "replace"),
    )
    .map(|report| fs::edit_observation(&report));
    observe_result(result, action_text, runtime, state)
}

pub fn dispatch_shell(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let command = param(params, "command");
    if let Err(error) = guard_shell_command(state.control.guard, &command) {
        return observe_error(error, action_text, runtime, state);
    }
    let timeout = match parse_u64(&param(params, "timeout")) {
        Ok(value) if value <= runtime.shell_timeout_max => value,
        Ok(_) => {
            return observe_error(
                ToolError::invalid("timeout must be at most 600 seconds"),
                action_text,
                runtime,
                state,
            )
        }
        Err(error) => return observe_error(error, action_text, runtime, state),
    };
    match shell::run(&runtime.workspace, &command, timeout) {
        Ok(report) if !report.succeeded() => finish(
            state,
            action_text,
            observe::error(
                shell_observation(&report, &command),
                runtime.observation_tokens,
            ),
        ),
        Ok(report) => finish(
            state,
            action_text,
            observe::ok(
                shell::observation(&report),
                runtime.observation_tokens,
                "shell.run with a narrower command",
            ),
        ),
        Err(error) => observe_error(error, action_text, runtime, state),
    }
}

fn shell_observation(report: &shell::ShellReport, command: &str) -> String {
    let mut output = shell::observation(report);
    if command.contains("/workspace") {
        output.push_str("hint=shell.run already starts in the workspace; do not cd /workspace\n");
    } else if command.contains('{') && command.contains('}') {
        output.push_str(
            "hint=/bin/sh does not expand brace lists; spell directories explicitly or loop over words\n",
        );
    }
    output
}

fn finish_read(
    read: fs::FileRead,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    if let Some(record) = duplicate_read(state, &read) {
        return finish(
            state,
            action_text,
            observe::notice(
                "error",
                format!("duplicate read refused; see frame {}", record.frame_ref),
            ),
        );
    }
    let frame = observe::ok(
        fs::read_observation(&read),
        runtime.observation_tokens,
        "fs.read with a narrower range",
    );
    let output = finish(state, action_text, frame);
    state.reads.push(ReadRecord {
        path: read.path,
        start: read.start,
        count: read.count,
        total_lines: read.total_lines,
        body: read.body,
        frame_ref: output.frame_ref,
    });
    output
}

fn duplicate_read<'a>(state: &'a DispatchState, read: &fs::FileRead) -> Option<&'a ReadRecord> {
    state.reads.iter().find(|record| {
        record.path == read.path
            && record.start == read.start
            && record.count == read.count
            && record.total_lines == read.total_lines
            && record.body == read.body
    })
}
