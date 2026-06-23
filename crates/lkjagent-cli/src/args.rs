use std::path::PathBuf;

use crate::error::CliError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub data_dir: PathBuf,
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelLogCommand {
    Current { print: bool },
    List { limit: usize },
    Show { case_id: String, turn_id: i64 },
}

pub fn parse_args<I, S>(args: I) -> Result<Invocation, CliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut data_dir = PathBuf::from("/data");
    let mut command: Option<String> = None;
    let mut command_args = Vec::new();
    let mut iter = args.into_iter().map(Into::into).peekable();
    while let Some(arg) = iter.next() {
        if arg == "--data" {
            let Some(path) = iter.next() else {
                return Err(CliError::usage("--data requires a directory"));
            };
            data_dir = PathBuf::from(path);
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
        "run" => Ok(Command::Run),
        "send" => parse_send(args),
        "status" => Ok(Command::Status),
        "log" => parse_log(args),
        "console" => parse_no_args(args, "console").map(|()| Command::Console),
        "memory" => parse_memory(args),
        "graph" => parse_no_args(args, "graph").map(|()| Command::Graph),
        "model-log" => parse_model_log(args),
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

fn parse_model_log(args: Vec<String>) -> Result<Command, CliError> {
    if args.first().is_some_and(|arg| arg == "list") {
        return parse_model_log_list(args.into_iter().skip(1).collect());
    }
    if args.first().is_some_and(|arg| arg == "show") {
        return parse_model_log_show(args.into_iter().skip(1).collect());
    }
    let mut print = false;
    for arg in args {
        match arg.as_str() {
            "--print" => print = true,
            other => return Err(unknown_option("model-log", other)),
        }
    }
    model_log_command(ModelLogCommand::Current { print })
}

fn parse_model_log_list(args: Vec<String>) -> Result<Command, CliError> {
    let mut limit = 20usize;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--limit" => {
                let Some(value) = iter.next() else {
                    return Err(CliError::usage("--limit requires a number"));
                };
                limit = parse_limit(&value)?;
            }
            other => return Err(unknown_option("model-log list", other)),
        }
    }
    model_log_command(ModelLogCommand::List { limit })
}

fn parse_model_log_show(args: Vec<String>) -> Result<Command, CliError> {
    let mut case_id = None;
    let mut turn_id = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--case" => case_id = iter.next(),
            "--turn" => turn_id = iter.next().map(|value| value.parse()),
            other => return Err(unknown_option("model-log show", other)),
        }
    }
    let Some(case_id) = case_id else {
        return Err(CliError::usage("model-log show requires --case"));
    };
    let Some(Ok(turn_id)) = turn_id else {
        return Err(CliError::usage("model-log show requires numeric --turn"));
    };
    model_log_command(ModelLogCommand::Show { case_id, turn_id })
}

fn model_log_command(command: ModelLogCommand) -> Result<Command, CliError> {
    Ok(Command::ModelLog(command))
}

fn unknown_option(command: &str, option: &str) -> CliError {
    CliError::usage(format!("unknown {command} option: {option}"))
}
