use std::path::{Path, PathBuf};

pub fn run(args: &[String], root: &Path) -> i32 {
    match dispatch(args, root) {
        Ok(path) => {
            println!("ok experiment out={}", path.display());
            0
        }
        Err(error) => {
            eprintln!("experiment failed");
            eprintln!("exit status: 1");
            eprintln!("{error}");
            1
        }
    }
}

fn dispatch(args: &[String], root: &Path) -> Result<PathBuf, String> {
    match args.first().map(String::as_str) {
        Some("live-profiles") => parse_live(&args[1..], root).and_then(crate::experiment_live::run),
        _ => parse_protocol(args, root).and_then(crate::experiment_protocol::run),
    }
}

fn parse_protocol(
    args: &[String],
    root: &Path,
) -> Result<crate::experiment_protocol::Options, String> {
    let mut options = crate::experiment_protocol::Options {
        out: root.join("tmp/protocol-experiment-current.md"),
        out_dir: root.join("tmp/protocol-experiments/current"),
        profile: "baseline".to_string(),
        all: false,
    };
    let mut index = if args.first().is_some_and(|arg| arg == "protocol") {
        1
    } else {
        0
    };
    while index < args.len() {
        match args[index].as_str() {
            "--all" => {
                options.all = true;
                index += 1;
            }
            "--out" => {
                options.out = path_arg(args, index + 1, root, "--out")?;
                index += 2;
            }
            "--out-dir" => {
                options.out_dir = path_arg(args, index + 1, root, "--out-dir")?;
                index += 2;
            }
            "--profile" => {
                options.profile = args.get(index + 1).ok_or("--profile needs a name")?.clone();
                index += 2;
            }
            other => return Err(format!("unknown experiment argument: {other}")),
        }
    }
    Ok(options)
}

fn parse_live(args: &[String], root: &Path) -> Result<crate::experiment_live::LiveOptions, String> {
    let mut out_dir = root.join("tmp/live-runs/current-profiles");
    let mut data_dir = root.join("tmp/live-profile-data/current");
    let mut duration_seconds = 900;
    let mut force_skip = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--out-dir" => {
                out_dir = path_arg(args, index + 1, root, "--out-dir")?;
                index += 2;
            }
            "--data" => {
                data_dir = path_arg(args, index + 1, root, "--data")?;
                index += 2;
            }
            "--duration-seconds" => {
                duration_seconds = args
                    .get(index + 1)
                    .ok_or("--duration-seconds needs a value")?
                    .parse::<u64>()
                    .map_err(|error| error.to_string())?;
                index += 2;
            }
            "--skip-endpoint" => {
                force_skip = true;
                index += 1;
            }
            other => return Err(format!("unknown live-profiles argument: {other}")),
        }
    }
    Ok(crate::experiment_live::LiveOptions {
        out_dir,
        data_dir,
        duration_seconds,
        force_skip,
    })
}

fn path_arg(args: &[String], index: usize, root: &Path, flag: &str) -> Result<PathBuf, String> {
    let value = args
        .get(index)
        .ok_or_else(|| format!("{flag} needs a path"))?;
    let path = PathBuf::from(value);
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}
