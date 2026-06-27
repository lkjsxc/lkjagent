use std::path::PathBuf;

#[path = "args_help.rs"]
mod args_help;
#[path = "args_model_log.rs"]
mod args_model_log;
#[path = "args_personal.rs"]
mod args_personal;

use crate::error::CliError;
pub use args_help::{help_text, is_help_arg, is_help_invocation};
use args_model_log::parse_model_log;
use args_personal::parse_personal;

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
    },
    Status,
    Log {
        follow: bool,
        full: bool,
        limit: Option<usize>,
    },
    Console,
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
                command: Command::Help,
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
        "help" => parse_no_args(args, "help").map(|()| Command::Help),
        "run" => parse_no_args(args, "run").map(|()| Command::Run),
        "send" => parse_send(args),
        "status" => parse_no_args(args, "status").map(|()| Command::Status),
        "log" => parse_log(args),
        "console" => parse_no_args(args, "console").map(|()| Command::Console),
        "memory" => parse_memory(args),
        "graph" => parse_no_args(args, "graph").map(|()| Command::Graph),
        "model-log" => parse_model_log(args),
        "personal" => parse_personal(args),
        other => Err(CliError::usage(format!("unknown command: {other}"))),
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

fn parse_log(args: Vec<String>) -> Result<Command, CliError> {
    let mut follow = false;
    let mut full = false;
    let mut limit = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--follow" => follow = true,
            "--full" => full = true,
            "--limit" => {
                let Some(value) = iter.next() else {
                    return Err(CliError::usage("--limit requires a number"));
                };
                limit = Some(parse_limit(&value)?);
            }
            other => return Err(CliError::usage(format!("unknown log option: {other}"))),
        }
    }
    Ok(Command::Log {
        follow,
        full,
        limit,
    })
}

fn parse_limit(value: &str) -> Result<usize, CliError> {
    value
        .parse()
        .map_err(|_| CliError::usage(format!("invalid --limit: {value}")))
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
