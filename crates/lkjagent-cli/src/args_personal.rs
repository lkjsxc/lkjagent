use crate::args::{Command, PersonalCommand};
use crate::error::CliError;

pub(super) fn parse_personal(args: Vec<String>) -> Result<Command, CliError> {
    match args.first().map(String::as_str) {
        Some("list") => parse_list(args.into_iter().skip(1).collect()),
        Some("render") => parse_render(args.into_iter().skip(1).collect()),
        _ => Err(CliError::usage("personal requires list or render")),
    }
}

fn parse_list(args: Vec<String>) -> Result<Command, CliError> {
    let mut kind = None;
    let mut status = None;
    let mut project = None;
    let mut limit = 20usize;
    parse_options(args, |key, value| match key {
        "--kind" => {
            kind = Some(valid_kind(value)?);
            Ok(())
        }
        "--status" => {
            status = Some(valid_status(value)?);
            Ok(())
        }
        "--project" => {
            project = Some(required(value, "--project")?.to_string());
            Ok(())
        }
        "--limit" => {
            limit = parse_usize(value, "--limit")?;
            Ok(())
        }
        other => Err(CliError::usage(format!(
            "unknown personal list option: {other}"
        ))),
    })?;
    Ok(Command::Personal(PersonalCommand::List {
        kind,
        status,
        project,
        limit,
    }))
}

fn parse_render(args: Vec<String>) -> Result<Command, CliError> {
    if args.is_empty() {
        Ok(Command::Personal(PersonalCommand::Render))
    } else {
        Err(CliError::usage("personal render takes no arguments"))
    }
}

fn parse_options<F>(args: Vec<String>, mut apply: F) -> Result<(), CliError>
where
    F: FnMut(&str, Option<&str>) -> Result<(), CliError>,
{
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let value = match arg.as_str() {
            "--kind" | "--status" | "--project" | "--limit" => iter.next().map(String::as_str),
            _ => None,
        };
        apply(arg, value)?;
    }
    Ok(())
}

fn valid_kind(value: Option<&str>) -> Result<String, CliError> {
    let value = required(value, "--kind")?;
    match value {
        "diary" | "schedule" | "todo" => Ok(value.to_string()),
        other => Err(CliError::usage(format!("unknown personal kind: {other}"))),
    }
}

fn valid_status(value: Option<&str>) -> Result<String, CliError> {
    let value = required(value, "--status")?;
    match value {
        "open" | "doing" | "waiting" | "done" | "canceled" => Ok(value.to_string()),
        other => Err(CliError::usage(format!("unknown personal status: {other}"))),
    }
}

fn parse_usize(value: Option<&str>, flag: &str) -> Result<usize, CliError> {
    required(value, flag)?
        .parse()
        .map_err(|_| CliError::usage(format!("invalid {flag}")))
}

fn required<'a>(value: Option<&'a str>, flag: &str) -> Result<&'a str, CliError> {
    value.ok_or_else(|| CliError::usage(format!("{flag} requires a value")))
}
