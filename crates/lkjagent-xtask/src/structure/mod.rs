pub mod brief;
pub mod catalog;
pub mod findings;
pub mod plan;
pub mod readme;
pub mod render;

use std::path::Path;

use crate::facts::collect_files;

use plan::{build_plan, StructurePlan};
use render::{render_audit, render_plan};

pub fn audit(files: &[crate::model::RepoFile], root: &str) -> StructurePlan {
    build_plan(files, root, 8)
}

pub fn run(args: &[String], repo_root: &Path) -> i32 {
    match parse(args) {
        Ok(Command::Audit { root }) => run_command(repo_root, &root, render_audit),
        Ok(Command::Plan { root }) => run_command(repo_root, &root, render_plan),
        Err(lines) => {
            for line in lines {
                eprintln!("{line}");
            }
            2
        }
    }
}

fn run_command(repo_root: &Path, root: &str, render: fn(&StructurePlan) -> Vec<String>) -> i32 {
    let files = match collect_files(repo_root) {
        Ok(files) => files,
        Err(message) => {
            eprintln!("structure audit failed");
            eprintln!("exit status: 1");
            eprintln!("{message}");
            return 1;
        }
    };
    let plan = audit(&files, root);
    for line in render(&plan) {
        println!("{line}");
    }
    0
}

enum Command {
    Audit { root: String },
    Plan { root: String },
}

fn parse(args: &[String]) -> Result<Command, Vec<String>> {
    match args {
        [command, flag, root] if flag == "--root" && command == "audit" => Ok(Command::Audit {
            root: root.to_string(),
        }),
        [command, flag, root] if flag == "--root" && command == "plan" => Ok(Command::Plan {
            root: root.to_string(),
        }),
        _ => Err(vec![
            "structure failed".to_string(),
            "exit status: 2".to_string(),
            "use: structure audit --root <path> | structure plan --root <path>".to_string(),
        ]),
    }
}
