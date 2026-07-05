use crate::args::Command;

pub(crate) fn parse_task(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [one] if one == "list" => Ok(Command::TaskList),
        [one, id] if one == "show" => id
            .parse::<u64>()
            .map(|id| Command::TaskShow { id })
            .map_err(|error| error.to_string()),
        _ => Err("use task list | task show ID".to_string()),
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
    let mut json = false;
    let mut rebuild = false;
    for arg in rest {
        match arg.as_str() {
            "--json" => json = true,
            "--rebuild" => rebuild = true,
            _ => return Err("use workspace [--json] [--rebuild]".to_string()),
        }
    }
    Ok(Command::Workspace { json, rebuild })
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
