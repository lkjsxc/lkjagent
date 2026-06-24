use crate::args::{Command, ModelLogCommand};
use crate::error::CliError;

pub(super) fn parse_model_log(args: Vec<String>) -> Result<Command, CliError> {
    match args.first().map(String::as_str) {
        Some("list") => parse_list(args.into_iter().skip(1).collect()),
        Some("show") => parse_case_turn(args.into_iter().skip(1).collect(), CommandKind::Show),
        Some("export") => parse_case_turn(args.into_iter().skip(1).collect(), CommandKind::Export),
        Some("raw-case") => parse_raw_case(args.into_iter().skip(1).collect()),
        _ => parse_current(args),
    }
}

enum CommandKind {
    Show,
    Export,
}

fn parse_current(args: Vec<String>) -> Result<Command, CliError> {
    let mut print = false;
    for arg in args {
        match arg.as_str() {
            "--print" => print = true,
            other => return Err(unknown_option("model-log", other)),
        }
    }
    Ok(Command::ModelLog(ModelLogCommand::Current { print }))
}

fn parse_list(args: Vec<String>) -> Result<Command, CliError> {
    let mut limit = 20usize;
    parse_options(args, |key, value| match key {
        "--limit" => {
            limit = parse_usize(value, "--limit")?;
            Ok(())
        }
        other => Err(unknown_option("model-log list", other)),
    })?;
    Ok(Command::ModelLog(ModelLogCommand::List { limit }))
}

fn parse_raw_case(args: Vec<String>) -> Result<Command, CliError> {
    let mut case_id = None;
    let mut limit = 20usize;
    parse_options(args, |key, value| match key {
        "--case" => {
            case_id = value.map(str::to_string);
            Ok(())
        }
        "--limit" => {
            limit = parse_usize(value, "--limit")?;
            Ok(())
        }
        other => Err(unknown_option("model-log raw-case", other)),
    })?;
    let Some(case_id) = case_id else {
        return Err(CliError::usage("model-log raw-case requires --case"));
    };
    Ok(Command::ModelLog(ModelLogCommand::RawCase {
        case_id,
        limit,
    }))
}

fn parse_case_turn(args: Vec<String>, kind: CommandKind) -> Result<Command, CliError> {
    let mut case_id = None;
    let mut turn_id = None;
    let name = match kind {
        CommandKind::Show => "show",
        CommandKind::Export => "export",
    };
    parse_options(args, |key, value| match key {
        "--case" => {
            case_id = value.map(str::to_string);
            Ok(())
        }
        "--turn" => {
            turn_id = Some(parse_i64(value, "--turn"));
            Ok(())
        }
        other => Err(unknown_option(&format!("model-log {name}"), other)),
    })?;
    let Some(case_id) = case_id else {
        return Err(CliError::usage(format!("model-log {name} requires --case")));
    };
    let Some(Ok(turn_id)) = turn_id else {
        return Err(CliError::usage(format!(
            "model-log {name} requires numeric --turn"
        )));
    };
    Ok(Command::ModelLog(match kind {
        CommandKind::Show => ModelLogCommand::Show { case_id, turn_id },
        CommandKind::Export => ModelLogCommand::Export { case_id, turn_id },
    }))
}

fn parse_options<F>(args: Vec<String>, mut apply: F) -> Result<(), CliError>
where
    F: FnMut(&str, Option<&str>) -> Result<(), CliError>,
{
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let value = match arg.as_str() {
            "--case" | "--turn" | "--limit" => iter.next().map(String::as_str),
            _ => None,
        };
        apply(arg, value)?;
    }
    Ok(())
}

fn parse_usize(value: Option<&str>, flag: &str) -> Result<usize, CliError> {
    value
        .ok_or_else(|| CliError::usage(format!("{flag} requires a number")))?
        .parse()
        .map_err(|_| CliError::usage(format!("invalid {flag}")))
}

fn parse_i64(value: Option<&str>, flag: &str) -> Result<i64, CliError> {
    value
        .ok_or_else(|| CliError::usage(format!("{flag} requires a number")))?
        .parse()
        .map_err(|_| CliError::usage(format!("invalid {flag}")))
}

fn unknown_option(command: &str, option: &str) -> CliError {
    CliError::usage(format!("unknown {command} option: {option}"))
}
