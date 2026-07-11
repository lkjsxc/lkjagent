use std::{collections::BTreeMap, path::Path};

use lkjagent_core::model::{StepState, TaskSnapshot};
use lkjagent_core::parse::Action;
use lkjagent_core::runtime_admission::ModelAction;
use lkjagent_core::runtime_tool_catalog::{effect_for_tool, ToolEffect};
use lkjagent_effects::observation::observation;
use lkjagent_store::memory::{search_memory, MemoryRow};
use rusqlite::Connection;

pub fn run(
    conn: &Connection,
    workspace: &Path,
    snapshot: &mut TaskSnapshot,
    action: &Action,
) -> Result<(), String> {
    let rendered = observation("ok", &dispatch(conn, workspace, action)?);
    if let Some(step) = snapshot
        .steps
        .iter_mut()
        .find(|step| step.state == StepState::Active)
    {
        let action_state = action_state(&step.inputs);
        step.inputs = format!("{action_state}latest_observation=\n{rendered}");
    }
    Ok(())
}

fn action_state(inputs: &str) -> String {
    inputs
        .lines()
        .filter(|line| line.starts_with("last_action") || line.starts_with("count="))
        .map(|line| format!("{line}\n"))
        .collect()
}

fn dispatch(conn: &Connection, workspace: &Path, action: &Action) -> Result<String, String> {
    let Some(effect) = effect_for_tool(&action.tool) else {
        return Err(format!("unknown tool: {}", action.tool));
    };
    match effect {
        ToolEffect::FsRead => lkjagent_effects::workspace::read(
            workspace,
            param(action, "path")?,
            number(action, "offset").unwrap_or(0),
            number(action, "count").unwrap_or(0),
        )
        .map_err(|error| error.to_string()),
        ToolEffect::FsList => lkjagent_effects::workspace::list(
            workspace,
            param_default(action, "path", "."),
            number(action, "depth").unwrap_or(1),
        )
        .map_err(|error| error.to_string()),
        ToolEffect::FsTree => lkjagent_effects::workspace::tree(
            workspace,
            param_default(action, "path", "."),
            number(action, "depth").unwrap_or(2),
        )
        .map_err(|error| error.to_string()),
        ToolEffect::FsSearch => indexed_search(conn, workspace, action),
        ToolEffect::FsWrite => lkjagent_effects::workspace::write(
            workspace,
            param(action, "path")?,
            param(action, "content")?,
        )
        .map_err(|error| error.to_string()),
        ToolEffect::ShellRun => shell(workspace, param(action, "command")?),
        ToolEffect::MemoryFind => memory_find(conn, param(action, "query")?),
        ToolEffect::MemorySave => Ok(format!("saved topic={}", param(action, "topic")?)),
        ToolEffect::PlanNote => Ok(format!("noted: {}", param(action, "note")?)),
    }
}

fn indexed_search(conn: &Connection, workspace: &Path, action: &Action) -> Result<String, String> {
    if param_default(action, "path", ".") != "." {
        return Err("fs.search only supports the workspace root".to_string());
    }
    crate::workspace_search::search(
        conn,
        workspace,
        &crate::workspace_search::Request {
            query: param(action, "query")?.to_string(),
            kind: None,
            state: None,
            project: None,
            date: None,
            mode: "lexical".to_string(),
        },
    )
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
        "memory {} matter={} {} {}",
        row.id, task, row.topic, row.content
    )
}

pub(crate) fn model_action(action: &Action) -> ModelAction {
    ModelAction {
        tool: action.tool.clone(),
        params: action
            .params
            .iter()
            .filter(|(name, _)| name != "tool_name")
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub(crate) fn fingerprint_text(value: &str) -> Result<String, String> {
    lkjagent_core::runtime_fingerprint::stable_fingerprint(&value).map_err(|error| error.message)
}

pub(crate) fn semantic_fingerprints(parsed: &str) -> Result<(String, String), String> {
    Ok((
        fingerprint_text("not-applicable")?,
        fingerprint_text(parsed)?,
    ))
}

pub(crate) fn write_target(action: &Action) -> Option<(&str, &str)> {
    if action.tool != "fs.write" {
        return None;
    }
    Some((param(action, "path").ok()?, param(action, "content").ok()?))
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
