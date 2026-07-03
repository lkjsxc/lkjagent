use std::path::Path;

const CURRENT: &[&str] = &[
    "lkjagent-core",
    "lkjagent-store",
    "lkjagent-llm",
    "lkjagent-effects",
    "lkjagent-app",
    "lkjagent-xtask",
];
const REMOVED: &[&str] = &[
    "lkjagent-runtime",
    "lkjagent-graph",
    "lkjagent-tools",
    "lkjagent-cli",
    "lkjagent-context",
    "lkjagent-protocol",
    "lkjagent-benchmark",
];

pub fn run(args: &[String], root: &Path) -> i32 {
    match args {
        [] => run_audit(root),
        [one] if one == "audit" => run_audit(root),
        _ => fail("use: structure audit"),
    }
}

fn run_audit(root: &Path) -> i32 {
    match audit(root) {
        Ok(()) => {
            println!("ok structure audit");
            0
        }
        Err(error) => fail(&error),
    }
}

pub fn audit(root: &Path) -> Result<(), String> {
    for name in CURRENT {
        if !root.join("crates").join(name).is_dir() {
            return Err(format!("missing crate {name}"));
        }
    }
    for name in REMOVED {
        if root.join("crates").join(name).exists() {
            return Err(format!("removed crate still present {name}"));
        }
    }
    Ok(())
}

fn fail(message: &str) -> i32 {
    eprintln!("structure audit failed");
    eprintln!("exit status: 1");
    eprintln!("{message}");
    1
}
