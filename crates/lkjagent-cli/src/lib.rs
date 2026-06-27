pub mod accounting;
pub mod args;
pub mod config;
pub mod console;
pub mod env_file;
pub mod error;
pub mod graph;
pub mod log;
pub mod memory;
pub mod model_log;
pub mod paths;
pub mod personal;
pub mod run;
pub mod send;
pub mod status;
pub mod store;

use args::{parse_args, Command};
use error::CliError;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliOutcome {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn run_cli<I, S>(args: I) -> CliOutcome
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    match dispatch(args) {
        Ok(stdout) => CliOutcome {
            code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(error) => CliOutcome {
            code: error.code(),
            stdout: String::new(),
            stderr: error.to_string(),
        },
    }
}

pub fn run_cli_with_dotenv<I, S>(args: I) -> CliOutcome
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    match env_file::load(Path::new(".env")) {
        Ok(()) => run_cli(args),
        Err(error) => CliOutcome {
            code: error.code(),
            stdout: String::new(),
            stderr: error.to_string(),
        },
    }
}

fn dispatch<I, S>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let invocation = parse_args(args)?;
    match invocation.command {
        Command::Run => run::run(&invocation.data_dir),
        Command::Send { text } => send::send(&invocation.data_dir, &text),
        Command::Status => status::status(&invocation.data_dir),
        Command::Log {
            follow,
            full,
            limit,
        } => log::log(&invocation.data_dir, follow, full, limit),
        Command::Console => console::console(&invocation.data_dir),
        Command::Memory { query } => memory::memory(&invocation.data_dir, &query),
        Command::Graph => graph::graph(&invocation.data_dir),
        Command::ModelLog(command) => model_log::model_log(&invocation.data_dir, command),
        Command::Personal(command) => personal::personal(&invocation.data_dir, command),
    }
}
