use std::path::Path;

pub fn run(_args: &[String], _root: &Path) -> i32 {
    print_failure(&[
        "benchmark failed".to_string(),
        "exit status: 1".to_string(),
        "benchmark corpus is not yet rebuilt for the plan engine".to_string(),
    ]);
    1
}

pub fn print_failure(lines: &[String]) {
    for line in lines {
        eprintln!("{line}");
    }
}
