use super::args_catalog::{CommandDoc, GROUPS};
use crate::error::CliError;

pub fn help_text(topic: Option<&str>) -> Result<String, CliError> {
    match topic {
        Some(topic) => group_help(topic),
        None => Ok(root_help()),
    }
}

pub fn is_help_arg(arg: &str) -> bool {
    matches!(arg, "--help" | "-h")
}

pub fn is_help_invocation(args: &[String]) -> bool {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--" {
            return false;
        }
        if is_help_arg(arg) || arg == "help" {
            return true;
        }
        if arg == "--data" {
            iter.next();
        }
    }
    false
}

fn root_help() -> String {
    let mut out = String::from("usage: lkjagent [--data DIR] <command> [args]\n\ncommands:\n");
    for group in GROUPS {
        out.push_str(&format!("  {:<10} {}\n", group.name, group.summary));
        for command in group.commands {
            out.push_str(&command_line(command));
        }
    }
    out.push_str(
        "\nglobal options:\n  --data DIR  runtime data directory, accepted before or after command\n  -h, --help  print this help\n\nUse 'lkjagent help <group>' for group help. Use -- when text starts with --.",
    );
    out
}

fn group_help(topic: &str) -> Result<String, CliError> {
    let Some(group) = GROUPS.iter().find(|group| group.name == topic) else {
        return Err(CliError::usage(format!("unknown help topic: {topic}")));
    };
    let mut out = format!(
        "usage: lkjagent {topic} <command> [args]\n\n{}:\n",
        group.summary
    );
    for command in group.commands {
        out.push_str(&command_line(command));
    }
    Ok(out)
}

fn command_line(command: &CommandDoc) -> String {
    format!("  {:<42} {}\n", command.usage, command.summary)
}
