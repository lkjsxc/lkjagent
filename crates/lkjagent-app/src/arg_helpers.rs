use crate::args::Command;

pub(crate) fn parse_matter(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [one] if one == "list" => Ok(Command::MatterList),
        [one, id] if one == "show" => id
            .parse::<u64>()
            .map(|id| Command::MatterShow { id })
            .map_err(|error| error.to_string()),
        _ => Err("use matter list | matter show REF".to_string()),
    }
}

pub(crate) fn parse_queue(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [one] if one == "list" => Ok(Command::QueueList),
        [one, id] if one == "show" => id
            .parse::<i64>()
            .map(|id| Command::QueueShow { id })
            .map_err(|error| error.to_string()),
        _ => Err("use queue list | queue show ID".to_string()),
    }
}

pub(crate) fn parse_context(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [action, case_id, semantic_key, winning_item_id] if action == "resolve" => {
            Ok(Command::ContextResolve {
                case_id: case_id.clone(),
                semantic_key: semantic_key.clone(),
                winning_item_id: winning_item_id.clone(),
            })
        }
        _ => Err("use context resolve CASE_ID KEY WINNING_ITEM_ID".to_string()),
    }
}

pub(crate) fn parse_log(rest: Vec<String>) -> Result<Command, String> {
    let mut limit = 20;
    let mut follow = false;
    let mut index = 0;
    while index < rest.len() {
        match rest[index].as_str() {
            "--follow" => {
                follow = true;
                index += 1;
            }
            "--limit" => {
                let value = rest
                    .get(index + 1)
                    .ok_or_else(|| "use log [--limit N] [--follow]".to_string())?;
                limit = value.parse::<usize>().map_err(|e| e.to_string())?;
                index += 2;
            }
            _ => return Err("use log [--limit N] [--follow]".to_string()),
        }
    }
    Ok(Command::Log { limit, follow })
}

pub(crate) fn parse_workspace(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [cmd, values @ ..] if cmd == "search" => parse_workspace_search(values),
        [cmd, flags @ ..] if cmd == "plan-rebalance" => Ok(Command::WorkspacePlanRebalance {
            json: only_json(flags)?,
        }),
        [cmd, flags @ ..] if cmd == "apply-rebalance" => Ok(Command::WorkspaceApplyRebalance {
            json: only_json(flags)?,
        }),
        [cmd, flags @ ..] if cmd == "validate" => Ok(Command::WorkspaceValidate {
            json: only_json(flags)?,
        }),
        _ => parse_workspace_report(rest),
    }
}

fn parse_workspace_search(values: &[String]) -> Result<Command, String> {
    let Some(query) = values.first() else {
        return Err(workspace_usage());
    };
    let mut kind = None;
    let mut state = None;
    let mut project = None;
    let mut date = None;
    let mut mode = "lexical".to_string();
    let mut index = 1;
    while index < values.len() {
        let value = values.get(index + 1).ok_or_else(workspace_usage)?;
        match values[index].as_str() {
            "--kind" => kind = Some(value.clone()),
            "--state" => state = Some(value.clone()),
            "--project" => project = Some(value.clone()),
            "--date" => date = Some(value.clone()),
            "--mode" => mode = value.clone(),
            _ => return Err(workspace_usage()),
        }
        index += 2;
    }
    Ok(Command::WorkspaceSearch {
        query: query.clone(),
        kind,
        state,
        project,
        date,
        mode,
    })
}

fn parse_workspace_report(rest: Vec<String>) -> Result<Command, String> {
    let mut json = false;
    let mut rebuild = false;
    for arg in rest {
        match arg.as_str() {
            "--json" => json = true,
            "--rebuild" => rebuild = true,
            _ => return Err(workspace_usage()),
        }
    }
    Ok(Command::Workspace { json, rebuild })
}

fn only_json(flags: &[String]) -> Result<bool, String> {
    match flags {
        [] => Ok(false),
        [flag] if flag == "--json" => Ok(true),
        _ => Err(workspace_usage()),
    }
}

fn workspace_usage() -> String {
    "use workspace [--json] [--rebuild] | search QUERY [--kind KIND] [--state STATE] [--project PROJECT] [--date DATE] [--mode lexical|trigram] | plan-rebalance [--json] | apply-rebalance [--json] | validate [--json]".to_string()
}

pub(crate) fn parse_json_flag(command: &str, rest: Vec<String>) -> Result<bool, String> {
    match rest.as_slice() {
        [] => Ok(false),
        [flag] if flag == "--json" => Ok(true),
        _ => Err(format!("use {command} [--json]")),
    }
}

pub(crate) fn parse_memory(rest: Vec<String>) -> Result<Command, String> {
    let query = rest.join(" ");
    if query.trim().is_empty() {
        Err("memory requires QUERY".to_string())
    } else {
        Ok(Command::Memory { query })
    }
}

pub(crate) fn no_args(rest: Vec<String>, command: Command) -> Result<Command, String> {
    if rest.is_empty() {
        Ok(command)
    } else {
        Err("command takes no arguments".to_string())
    }
}
