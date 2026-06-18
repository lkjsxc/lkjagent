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
    Send { text: String },
    Status,
    Log { follow: bool, full: bool },
    Console,
    Memory { query: String },
    Skills,
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
        "skills" => Ok(Command::Skills),
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
    for arg in args {
        match arg.as_str() {
            "--follow" => follow = true,
            "--full" => full = true,
            other => return Err(CliError::usage(format!("unknown log option: {other}"))),
        }
    }
    Ok(Command::Log { follow, full })
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
