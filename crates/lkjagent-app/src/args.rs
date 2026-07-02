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
    Send { text: String, force_new: bool },
    Status,
    Log { limit: usize },
    TaskList,
    TaskShow { id: u64 },
    QueueList,
    QueueShow { id: i64 },
    Memory { query: String },
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
        "log" => Ok(Command::Log {
            limit: parse_limit(rest, 20)?,
        }),
        "task" => parse_task(rest),
        "queue" => parse_queue(rest),
        "memory" => Ok(Command::Memory {
            query: rest.join(" "),
        }),
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

fn parse_limit(rest: Vec<String>, default: usize) -> Result<usize, String> {
    match rest.as_slice() {
        [] => Ok(default),
        [flag, value] if flag == "--limit" => value.parse::<usize>().map_err(|e| e.to_string()),
        _ => Err("use --limit N".to_string()),
    }
}

fn no_args(rest: Vec<String>, command: Command) -> Result<Command, String> {
    if rest.is_empty() {
        Ok(command)
    } else {
        Err("command takes no arguments".to_string())
    }
}
