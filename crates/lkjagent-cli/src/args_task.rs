use crate::args::{Command, TaskCommand};
use crate::error::CliError;

pub(super) fn parse_task(args: Vec<String>) -> Result<Command, CliError> {
    match args.first().map(String::as_str) {
        Some("list") => parse_list(args.into_iter().skip(1).collect()),
        Some("show") => parse_show(args.into_iter().skip(1).collect()),
        _ => Err(CliError::usage("task requires list or show")),
    }
}

fn parse_list(args: Vec<String>) -> Result<Command, CliError> {
    let mut status = None;
    let mut limit = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--status" => status = Some(required(iter.next().map(String::as_str), "--status")?),
            "--limit" => limit = Some(parse_limit(iter.next().map(String::as_str))?),
            other => {
                return Err(CliError::usage(format!(
                    "unknown task list option: {other}"
                )))
            }
        }
    }
    Ok(Command::Task(TaskCommand::List { status, limit }))
}

fn parse_show(args: Vec<String>) -> Result<Command, CliError> {
    match args.as_slice() {
        [id] => Ok(Command::Task(TaskCommand::Show {
            id: parse_i64(Some(id), "task show")?,
        })),
        _ => Err(CliError::usage("task show requires id")),
    }
}

fn parse_limit(value: Option<&str>) -> Result<usize, CliError> {
    let parsed = parse_i64(value, "--limit")?;
    if parsed < 0 {
        Err(CliError::usage("invalid --limit"))
    } else {
        Ok(parsed as usize)
    }
}

fn parse_i64(value: Option<&str>, name: &str) -> Result<i64, CliError> {
    value
        .ok_or_else(|| CliError::usage(format!("{name} requires a number")))?
        .parse()
        .map_err(|_| CliError::usage(format!("invalid {name}")))
}

fn required(value: Option<&str>, name: &str) -> Result<String, CliError> {
    Ok(value
        .ok_or_else(|| CliError::usage(format!("{name} requires a value")))?
        .to_string())
}
