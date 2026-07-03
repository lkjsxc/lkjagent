use std::path::Path;

use lkjagent_core::model::{StepState, TaskSnapshot};
use lkjagent_core::parse::Action;
use lkjagent_effects::observation::observation;
use lkjagent_store::memory::{search_memory, MemoryRow};
use rusqlite::Connection;

pub fn run(conn: &Connection, workspace: &Path, snapshot: &mut TaskSnapshot, action: &Action) {
    let result = dispatch(conn, workspace, action);
    let rendered = match result {
        Ok(content) => observation("ok", &content),
        Err(error) => observation("error", &error),
    };
    if let Some(step) = snapshot
        .steps
        .iter_mut()
        .find(|step| step.state == StepState::Active)
    {
        let action_state = action_state(&step.inputs);
        step.inputs = format!("{action_state}latest_observation=\n{rendered}");
    }
}

fn action_state(inputs: &str) -> String {
    inputs
        .lines()
        .filter(|line| line.starts_with("last_action") || line.starts_with("count="))
        .map(|line| format!("{line}\n"))
        .collect()
}

fn dispatch(conn: &Connection, workspace: &Path, action: &Action) -> Result<String, String> {
    match action.tool.as_str() {
        "fs.read" => lkjagent_effects::workspace::read(
            workspace,
            param(action, "path")?,
            number(action, "offset").unwrap_or(0),
            number(action, "count").unwrap_or(0),
        )
        .map_err(|error| error.to_string()),
        "fs.list" => lkjagent_effects::workspace::list(
            workspace,
            param_default(action, "path", "."),
            number(action, "depth").unwrap_or(1),
        )
        .map_err(|error| error.to_string()),
        "fs.tree" => lkjagent_effects::workspace::tree(
            workspace,
            param_default(action, "path", "."),
            number(action, "depth").unwrap_or(2),
        )
        .map_err(|error| error.to_string()),
        "fs.search" => lkjagent_effects::workspace::search(
            workspace,
            param_default(action, "path", "."),
            param(action, "query")?,
        )
        .map_err(|error| error.to_string()),
        "fs.write" => lkjagent_effects::workspace::write(
            workspace,
            param(action, "path")?,
            param(action, "content")?,
        )
        .map_err(|error| error.to_string()),
        "shell.run" => shell(workspace, param(action, "command")?),
        "memory.find" => memory_find(conn, param(action, "query")?),
        "memory.save" => Ok(format!("saved topic={}", param(action, "topic")?)),
        "plan.note" => Ok(format!("noted: {}", param(action, "note")?)),
        "finish" => Ok(param(action, "summary")?.to_string()),
        other => Err(format!("unknown tool: {other}")),
    }
}

fn shell(workspace: &Path, command: &str) -> Result<String, String> {
    let report = lkjagent_effects::shell::run(workspace, command, 30).map_err(|e| e.to_string())?;
    Ok(format!(
        "exit={:?} timed_out={}\n{}",
        report.exit_code, report.timed_out, report.output
    ))
}

fn memory_find(conn: &Connection, query: &str) -> Result<String, String> {
    let rows = search_memory(conn, query, 10).map_err(|error| error.to_string())?;
    if rows.is_empty() {
        Ok("no matches".to_string())
    } else {
        Ok(rows
            .iter()
            .map(render_memory)
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

fn render_memory(row: &MemoryRow) -> String {
    let task = row
        .task_id
        .map_or_else(|| "none".to_string(), |task_id| task_id.to_string());
    format!(
        "memory {} task={} {} {}",
        row.id, task, row.topic, row.content
    )
}

fn param<'a>(action: &'a Action, name: &str) -> Result<&'a str, String> {
    action
        .params
        .iter()
        .find(|(param, _)| param == name)
        .map(|(_, value)| value.as_str())
        .ok_or_else(|| format!("missing parameter: {name}"))
}

fn param_default<'a>(action: &'a Action, name: &str, default: &'a str) -> &'a str {
    action
        .params
        .iter()
        .find(|(param, _)| param == name)
        .map_or(default, |(_, value)| value.as_str())
}

fn number(action: &Action, name: &str) -> Result<usize, String> {
    let Some(value) = action
        .params
        .iter()
        .find(|(param, _)| param == name)
        .map(|(_, value)| value)
    else {
        return Ok(0);
    };
    value.parse::<usize>().map_err(|error| error.to_string())
}
