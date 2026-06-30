use std::path::PathBuf;

#[path = "args_catalog.rs"]
mod args_catalog;
#[path = "args_help.rs"]
mod args_help;
#[path = "args_log.rs"]
mod args_log;
#[path = "args_model_log.rs"]
mod args_model_log;
#[path = "args_personal.rs"]
mod args_personal;
#[path = "args_queue.rs"]
mod args_queue;
#[path = "args_task.rs"]
mod args_task;

use crate::error::CliError;
pub use args_help::{help_text, is_help_arg, is_help_invocation};
use args_log::parse_log;
use args_model_log::parse_model_log;
use args_personal::parse_personal;
use args_queue::parse_queue;
use args_task::parse_task;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub data_dir: PathBuf,
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help {
        topic: Option<String>,
    },
    Run,
    Send {
        text: String,
    },
    Status,
    Log {
        follow: bool,
        full: bool,
        limit: Option<usize>,
    },
    Console,
    Queue(QueueCommand),
    Task(TaskCommand),
    Memory {
        query: String,
    },
    Graph,
    ModelLog(ModelLogCommand),
    Personal(PersonalCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelLogCommand {
    Current { print: bool },
    List { limit: usize },
    Show { case_id: String, turn_id: i64 },
    Export { case_id: String, turn_id: i64 },
    RawCase { case_id: String, limit: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonalCommand {
    List {
        kind: Option<String>,
        status: Option<String>,
        project: Option<String>,
        limit: usize,
    },
    Render,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueCommand {
    List { limit: Option<usize> },
    Show { id: i64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskCommand {
    List {
        status: Option<String>,
        limit: Option<usize>,
    },
    Show {
        id: i64,
    },
}

pub fn parse_args<I, S>(args: I) -> Result<Invocation, CliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut data_dir = PathBuf::from("/data");
    let mut command: Option<String> = None;
    let mut command_args = Vec::new();
    let mut positional = false;
    let mut iter = args.into_iter().map(Into::into);
    while let Some(arg) = iter.next() {
        if !positional && is_help_arg(&arg) {
            return Ok(Invocation {
                data_dir,
                command: Command::Help { topic: None },
            });
        }
        if !positional && arg == "--data" {
            let Some(path) = iter.next() else {
                return Err(CliError::usage("--data requires a directory"));
            };
            data_dir = PathBuf::from(path);
        } else if !positional && arg == "--" {
            positional = true;
        } else if command.is_none() {
            command = Some(arg);
        } else {
            command_args.push(arg);
        }
    }
    let Some(command) = command else {
        return Err(CliError::usage("missing command"));
    };
    Ok(Invocation {
        data_dir,
        command: parse_command(&command, command_args)?,
    })
}

fn parse_command(command: &str, args: Vec<String>) -> Result<Command, CliError> {
    match command {
        "help" => parse_help(args),
        "run" => parse_no_args(args, "run").map(|()| Command::Run),
        "send" => parse_send(args),
        "status" => parse_no_args(args, "status").map(|()| Command::Status),
        "log" => parse_log(args),
        "watch" | "console" => parse_no_args(args, command).map(|()| Command::Console),
        "memory" => parse_memory(args),
        "queue" => parse_queue(args),
        "task" => parse_task(args),
        "graph" => parse_no_args(args, "graph").map(|()| Command::Graph),
        "model-log" => parse_model_log(args),
        "personal" => parse_personal(args),
        other => Err(CliError::usage(format!("unknown command: {other}"))),
    }
}

fn parse_help(args: Vec<String>) -> Result<Command, CliError> {
    match args.as_slice() {
        [] => Ok(Command::Help { topic: None }),
        [topic] => Ok(Command::Help {
            topic: Some(topic.to_string()),
        }),
        _ => Err(CliError::usage("help accepts at most one topic")),
    }
}

fn parse_send(args: Vec<String>) -> Result<Command, CliError> {
    let text = args.join(" ");
    if text.is_empty() {
        Err(CliError::usage("send requires text"))
    } else {
        Ok(Command::Send { text })
    }
}

fn parse_no_args(args: Vec<String>, command: &str) -> Result<(), CliError> {
    if args.is_empty() {
        Ok(())
    } else {
        Err(CliError::usage(format!("{command} takes no arguments")))
    }
}

fn parse_memory(args: Vec<String>) -> Result<Command, CliError> {
    let query = args.join(" ");
    if query.is_empty() {
        Err(CliError::usage("memory requires a query"))
    } else {
        Ok(Command::Memory { query })
    }
}
