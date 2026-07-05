use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub data_dir: PathBuf,
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    Run,
    Send {
        text: String,
        force_new: bool,
    },
    Status,
    Console,
    Log {
        limit: usize,
        follow: bool,
    },
    TaskList,
    TaskShow {
        id: u64,
    },
    QueueList,
    QueueShow {
        id: i64,
    },
    ContextResolve {
        case_id: String,
        semantic_key: String,
        winning_item_id: String,
    },
    RecordAdd {
        kind: String,
        title: String,
        body: String,
    },
    RecordList {
        kind: Option<String>,
    },
    RecordShow {
        id: String,
    },
    RecordLink {
        id: String,
        target: String,
    },
    RecordArchive {
        id: String,
    },
    Memory {
        query: String,
    },
    Watch,
}

pub fn parse<I, S>(args: I) -> Result<Invocation, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut data_dir = PathBuf::from("data");
    let mut items = args.into_iter().map(Into::into).collect::<Vec<String>>();
    if items.first().is_some_and(|arg| arg == "--data") {
        if items.len() < 3 {
            return Err("--data requires a directory and command".to_string());
        }
        data_dir = PathBuf::from(items[1].clone());
        items.drain(0..2);
    }
    let Some(command) = items.first().cloned() else {
        return Ok(Invocation {
            data_dir,
            command: Command::Help,
        });
    };
    let rest = items.into_iter().skip(1).collect::<Vec<_>>();
    Ok(Invocation {
        data_dir,
        command: parse_command(&command, rest)?,
    })
}

fn parse_command(command: &str, rest: Vec<String>) -> Result<Command, String> {
    match command {
        "help" => Ok(Command::Help),
        "run" => no_args(rest, Command::Run),
        "send" => parse_send(rest),
        "status" => no_args(rest, Command::Status),
        "console" => no_args(rest, Command::Console),
        "log" => parse_log(rest),
        "task" => parse_task(rest),
        "queue" => parse_queue(rest),
        "context" => parse_context(rest),
        "record" => crate::record_args::parse_record(rest),
        "memory" => parse_memory(rest),
        "watch" => no_args(rest, Command::Watch),
        other => Err(format!("unknown command: {other}")),
    }
}

fn parse_send(rest: Vec<String>) -> Result<Command, String> {
    let mut force_new = false;
    let mut words = Vec::new();
    for arg in rest {
        if arg == "--new" {
            force_new = true;
        } else {
            words.push(arg);
        }
    }
    let text = words.join(" ");
    if text.is_empty() {
        Err("send requires text".to_string())
    } else {
        Ok(Command::Send { text, force_new })
    }
}

fn parse_task(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [one] if one == "list" => Ok(Command::TaskList),
        [one, id] if one == "show" => id
            .parse::<u64>()
            .map(|id| Command::TaskShow { id })
            .map_err(|error| error.to_string()),
        _ => Err("use task list | task show ID".to_string()),
    }
}

fn parse_queue(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [one] if one == "list" => Ok(Command::QueueList),
        [one, id] if one == "show" => id
            .parse::<i64>()
            .map(|id| Command::QueueShow { id })
            .map_err(|error| error.to_string()),
        _ => Err("use queue list | queue show ID".to_string()),
    }
}

fn parse_context(rest: Vec<String>) -> Result<Command, String> {
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

fn parse_log(rest: Vec<String>) -> Result<Command, String> {
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

fn parse_memory(rest: Vec<String>) -> Result<Command, String> {
    let query = rest.join(" ");
    if query.trim().is_empty() {
        Err("memory requires QUERY".to_string())
    } else {
        Ok(Command::Memory { query })
    }
}

fn no_args(rest: Vec<String>, command: Command) -> Result<Command, String> {
    if rest.is_empty() {
        Ok(command)
    } else {
        Err("command takes no arguments".to_string())
    }
}
