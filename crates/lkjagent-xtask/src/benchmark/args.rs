use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchmarkCommand {
    List,
    CheckCorpus,
    Judge {
        task: String,
        workspace: PathBuf,
    },
    Run(RunArgs),
    Compare {
        old_report: PathBuf,
        new_report: PathBuf,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunArgs {
    pub suite: String,
    pub data: PathBuf,
    pub task: Option<String>,
    pub min_points: Option<u16>,
}

pub fn parse(args: &[String]) -> Result<BenchmarkCommand, Vec<String>> {
    match args {
        [one] if one == "list" => Ok(BenchmarkCommand::List),
        [one] if one == "check-corpus" => Ok(BenchmarkCommand::CheckCorpus),
        [one, rest @ ..] if one == "judge" => parse_judge(rest),
        [one, rest @ ..] if one == "run" => parse_run(rest),
        [one, old_report, new_report] if one == "compare" => Ok(BenchmarkCommand::Compare {
            old_report: PathBuf::from(old_report),
            new_report: PathBuf::from(new_report),
        }),
        _ => Err(usage()),
    }
}

fn parse_judge(args: &[String]) -> Result<BenchmarkCommand, Vec<String>> {
    let mut task = None;
    let mut workspace = None;
    let mut index = 0;
    while index < args.len() {
        match args.get(index).map(String::as_str) {
            Some("--task") => {
                task = args.get(index + 1).cloned();
                index += 2;
            }
            Some("--workspace") => {
                workspace = args.get(index + 1).map(PathBuf::from);
                index += 2;
            }
            _ => return Err(usage()),
        }
    }
    match (task, workspace) {
        (Some(task), Some(workspace)) => Ok(BenchmarkCommand::Judge { task, workspace }),
        _ => Err(usage()),
    }
}

fn parse_run(args: &[String]) -> Result<BenchmarkCommand, Vec<String>> {
    let mut suite = None;
    let mut data = None;
    let mut task = None;
    let mut min_points = None;
    let mut index = 0;
    while index < args.len() {
        match args.get(index).map(String::as_str) {
            Some("--suite") => {
                suite = args.get(index + 1).cloned();
                index += 2;
            }
            Some("--data") => {
                data = args.get(index + 1).map(PathBuf::from);
                index += 2;
            }
            Some("--task") => {
                task = args.get(index + 1).cloned();
                index += 2;
            }
            Some("--min-points") => {
                min_points = args.get(index + 1).and_then(|value| value.parse().ok());
                index += 2;
            }
            _ => return Err(usage()),
        }
    }
    match (suite, data) {
        (Some(suite), Some(data)) => Ok(BenchmarkCommand::Run(RunArgs {
            suite,
            data,
            task,
            min_points,
        })),
        _ => Err(usage()),
    }
}

fn usage() -> Vec<String> {
    vec![
        "benchmark failed".to_string(),
        "exit status: 2".to_string(),
        "use: benchmark list | benchmark check-corpus | benchmark judge --task <id> --workspace <dir> | benchmark run --suite <name> --data <dir> | benchmark compare <old> <new>"
            .to_string(),
    ]
}
