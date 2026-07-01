mod collect;
mod db;
mod db_support;
mod files;
mod model;
mod render;
mod render_rows;

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use collect::collect;
use model::CollectOptions;

pub fn run(args: &[String], root: &Path) -> i32 {
    match args {
        [cmd, rest @ ..] if cmd == "collect" => match parse_collect(rest, root) {
            Ok(options) => run_collect(&options),
            Err(error) => fail("proof collect", 2, error),
        },
        _ => fail(
            "proof",
            2,
            "use: proof collect [--data data] [--out tmp/proof-current]".to_string(),
        ),
    }
}

fn run_collect(options: &CollectOptions) -> i32 {
    match collect(options) {
        Ok(path) => {
            println!("ok proof-collect artifact={}", path.display());
            0
        }
        Err(error) => fail("proof collect", 1, error),
    }
}

fn parse_collect(args: &[String], root: &Path) -> Result<CollectOptions, String> {
    let mut data_dir = root.join("data");
    let mut out_dir = root.join("tmp/proof-current");
    let mut index = 0usize;
    while index < args.len() {
        match args.get(index).map(String::as_str) {
            Some("--data") => {
                data_dir = path_arg(args, index + 1, root, "--data")?;
                index += 2;
            }
            Some("--out") => {
                out_dir = path_arg(args, index + 1, root, "--out")?;
                index += 2;
            }
            Some(flag) => return Err(format!("unknown proof collect argument: {flag}")),
            None => break,
        }
    }
    Ok(CollectOptions { data_dir, out_dir })
}

fn path_arg(args: &[String], index: usize, root: &Path, flag: &str) -> Result<PathBuf, String> {
    let Some(value) = args.get(index) else {
        return Err(format!("{flag} needs a path"));
    };
    let path = PathBuf::from(value);
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(root.join(path))
    }
}

fn fail(name: &str, code: i32, message: String) -> i32 {
    eprintln!("{name} failed");
    eprintln!("exit status: {code}");
    eprintln!("{message}");
    code
}
