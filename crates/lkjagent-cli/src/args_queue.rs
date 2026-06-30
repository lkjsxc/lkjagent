use crate::args::{Command, QueueCommand};
use crate::error::CliError;

pub(super) fn parse_queue(args: Vec<String>) -> Result<Command, CliError> {
    match args.first().map(String::as_str) {
        Some("list") => parse_list(args.into_iter().skip(1).collect()),
        Some("show") => parse_show(args.into_iter().skip(1).collect()),
        _ => Err(CliError::usage("queue requires list or show")),
    }
}

fn parse_list(args: Vec<String>) -> Result<Command, CliError> {
    let mut limit = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--limit" => limit = Some(parse_i64(iter.next().map(String::as_str), "--limit")?),
            other => {
                return Err(CliError::usage(format!(
                    "unknown queue list option: {other}"
                )))
            }
        }
    }
    let limit = match limit {
        Some(value) if value >= 0 => Some(value as usize),
        Some(_) => return Err(CliError::usage("invalid --limit")),
        None => None,
    };
    Ok(Command::Queue(QueueCommand::List { limit }))
}

fn parse_show(args: Vec<String>) -> Result<Command, CliError> {
    match args.as_slice() {
        [id] => Ok(Command::Queue(QueueCommand::Show {
            id: parse_i64(Some(id), "queue show")?,
        })),
        _ => Err(CliError::usage("queue show requires id")),
    }
}

fn parse_i64(value: Option<&str>, name: &str) -> Result<i64, CliError> {
    value
        .ok_or_else(|| CliError::usage(format!("{name} requires a number")))?
        .parse()
        .map_err(|_| CliError::usage(format!("invalid {name}")))
}
