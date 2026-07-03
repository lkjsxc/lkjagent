use std::path::Path;

use lkjagent_core::model::{StepState, TaskSnapshot};
use lkjagent_core::parse::Action;
use lkjagent_effects::observation::observation;

pub fn run(workspace: &Path, snapshot: &mut TaskSnapshot, action: &Action) {
    let result = dispatch(workspace, snapshot, action);
    let rendered = match result {
        Ok(content) => observation("ok", &content),
        Err(error) => observation("error", &error),
    };
    if let Some(step) = snapshot
        .steps
        .iter_mut()
        .find(|step| step.state == StepState::Active)
    {
        step.inputs = format!("latest_observation=\n{rendered}");
    }
}

fn dispatch(workspace: &Path, snapshot: &TaskSnapshot, action: &Action) -> Result<String, String> {
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
        "memory.find" => Ok(memory_find(snapshot, param(action, "query")?)),
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

fn memory_find(snapshot: &TaskSnapshot, query: &str) -> String {
    let query = query.to_ascii_lowercase();
    let mut rows = Vec::new();
    if snapshot.task.brief.to_ascii_lowercase().contains(&query) {
        rows.push(format!("task brief: {}", snapshot.task.brief));
    }
    for event in &snapshot.events {
        if event.content.to_ascii_lowercase().contains(&query) {
            rows.push(format!("event {:?}: {}", event.kind, event.content));
        }
    }
    if rows.is_empty() {
        "no matches".to_string()
    } else {
        rows.join("\n")
    }
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
