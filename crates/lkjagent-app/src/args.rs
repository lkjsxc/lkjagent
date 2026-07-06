use std::path::PathBuf;

use crate::arg_helpers::{
    no_args, parse_context, parse_json_flag, parse_log, parse_memory, parse_queue, parse_task,
};
use crate::workbench_state::WorkbenchMode;

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
    Workbench {
        mode: WorkbenchMode,
    },
    Doctor {
        json: bool,
    },
    Workspace {
        json: bool,
        rebuild: bool,
    },
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
    if let Some(kind) = crate::record_args::wrapper_kind(command) {
        return crate::record_args::parse_wrapper(kind, rest);
    }
    match command {
        "help" => Ok(Command::Help),
        "run" => no_args(rest, Command::Run),
        "send" => parse_send(rest),
        "status" => no_args(rest, Command::Status),
        "console" => no_args(rest, Command::Console),
        "workbench" => parse_workbench(rest),
        "doctor" => parse_json_flag(command, rest).map(|json| Command::Doctor { json }),
        "workspace" => crate::arg_helpers::parse_workspace(rest),
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

fn parse_workbench(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [] => Ok(Command::Workbench {
            mode: WorkbenchMode::Append,
        }),
        [flag, value] if flag == "--mode" => Ok(Command::Workbench {
            mode: WorkbenchMode::parse(value)?,
        }),
        [flag] if flag == "--pane" => Ok(Command::Workbench {
            mode: WorkbenchMode::Pane,
        }),
        _ => Err("workbench accepts --mode append|pane".to_string()),
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
