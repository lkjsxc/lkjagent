use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub data_dir: PathBuf,
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    Run { once: bool },
    Send { text: String, force_new: bool },
    Status,
    Doctor { json: bool },
}

pub fn parse<I, S>(args: I) -> Result<Invocation, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut data_dir = PathBuf::from("data");
    let mut items = args.into_iter().map(Into::into).collect::<Vec<String>>();
    if items.first().is_some_and(|arg| arg == "--data") {
        if items.len() < 3 {
            return Err("--data requires a directory and command".to_string());
        }
        data_dir = PathBuf::from(items[1].clone());
        items.drain(0..2);
    }
    let Some(command) = items.first().cloned() else {
        return Ok(Invocation {
            data_dir,
            command: Command::Help,
        });
    };
    let rest = items.into_iter().skip(1).collect::<Vec<_>>();
    Ok(Invocation {
        data_dir,
        command: parse_command(&command, rest)?,
    })
}

fn parse_command(command: &str, rest: Vec<String>) -> Result<Command, String> {
    match command {
        "help" => no_args(rest, Command::Help),
        "run" => parse_run(rest),
        "send" => parse_send(rest),
        "status" => no_args(rest, Command::Status),
        "doctor" => parse_doctor(rest),
        other => Err(format!("unknown command: {other}")),
    }
}

fn parse_run(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [] => Ok(Command::Run { once: false }),
        [flag] if flag == "--once" => Ok(Command::Run { once: true }),
        _ => Err("run accepts --once".to_string()),
    }
}

fn parse_send(rest: Vec<String>) -> Result<Command, String> {
    let mut force_new = false;
    let mut words = Vec::new();
    for arg in rest {
        if arg == "--new" {
            force_new = true;
        } else {
            words.push(arg);
        }
    }
    let text = words.join(" ");
    if text.is_empty() {
        Err("send requires text".to_string())
    } else {
        Ok(Command::Send { text, force_new })
    }
}

fn parse_doctor(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [] => Ok(Command::Doctor { json: false }),
        [flag] if flag == "--json" => Ok(Command::Doctor { json: true }),
        _ => Err("use doctor [--json]".to_string()),
    }
}

fn no_args(rest: Vec<String>, command: Command) -> Result<Command, String> {
    if rest.is_empty() {
        Ok(command)
    } else {
        Err("command takes no arguments".to_string())
    }
}
