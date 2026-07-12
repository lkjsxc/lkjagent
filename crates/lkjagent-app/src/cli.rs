use crate::args::{parse, Command};

pub fn run<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let invocation = parse(args)?;
    match invocation.command {
        Command::Help => Ok(help()),
        Command::Send { text, force_new } => {
            crate::public_loop::send(&invocation.data_dir, &text, force_new)
        }
        Command::Status => crate::public_loop::status(&invocation.data_dir),
        Command::Doctor { json } => crate::public_loop::doctor(&invocation.data_dir, json),
        Command::Run { once: true } => {
            let mut endpoint = crate::endpoint::LlmEndpoint::new(&invocation.data_dir);
            crate::public_loop::run_once(&invocation.data_dir, &mut endpoint)
        }
        Command::Run { once: false } => {
            let mut endpoint = crate::endpoint::LlmEndpoint::new(&invocation.data_dir);
            crate::public_loop::run(&invocation.data_dir, &mut endpoint)?;
            Ok(String::new())
        }
    }
}

pub fn help() -> String {
    [
        "lkjagent commands:",
        "  help",
        "  send [--new] TEXT",
        "  run [--once]",
        "  status",
        "  doctor [--json]",
    ]
    .join("\n")
}
