use crate::args::Command;
use crate::error::CliError;

pub(super) fn parse_log(args: Vec<String>) -> Result<Command, CliError> {
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
