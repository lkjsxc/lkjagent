use std::path::{Path, PathBuf};

pub fn run(args: &[String], root: &Path) -> i32 {
    match parse(args, root).and_then(crate::experiment_protocol::run) {
        Ok(path) => {
            println!("ok experiment protocol out={}", path.display());
            0
        }
        Err(error) => {
            eprintln!("experiment protocol failed");
            eprintln!("exit status: 1");
            eprintln!("{error}");
            1
        }
    }
}

fn parse(args: &[String], root: &Path) -> Result<crate::experiment_protocol::Options, String> {
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
