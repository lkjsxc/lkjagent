use std::path::Path;

pub fn run(args: &[String], root: &Path) -> i32 {
    match args {
        [command] if command == "check-corpus" => {
            match crate::evaluation_harness::check_source(root) {
                Ok(_) => {
                    println!("ok bench check-corpus");
                    0
                }
                Err(failures) => fail("bench check-corpus", &failures.join("\n")),
            }
        }
        [command, ..] if command == "run" => fail(
            "benchmark run",
            "live benchmark summary lacks raw source-bound evaluation authority",
        ),
        _ => fail("benchmark", "use: bench check-corpus"),
    }
}

pub fn validate_corpus(root: &Path) -> Result<usize, String> {
    crate::evaluation_harness::check_source(root).map_err(|failures| failures.join("\n"))
}

fn fail(name: &str, message: &str) -> i32 {
    eprintln!("{name} failed");
    eprintln!("exit status: 1");
    eprintln!("{message}");
    1
}
