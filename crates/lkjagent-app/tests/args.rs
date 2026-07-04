use lkjagent_app::args::{parse, Command};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn parser_accepts_log_follow_forms() -> TestResult<()> {
    let first = parse(["log", "--follow"])?;
    assert_eq!(
        first.command,
        Command::Log {
            limit: 20,
            follow: true
        }
    );

    let second = parse(["log", "--limit", "7", "--follow"])?;
    assert_eq!(
        second.command,
        Command::Log {
            limit: 7,
            follow: true
        }
    );
    Ok(())
}

#[test]
fn parser_rejects_empty_memory_query() -> TestResult<()> {
    let error = match parse(["memory"]) {
        Ok(_) => return Err("memory without query parsed".into()),
        Err(error) => error,
    };
    assert!(error.contains("memory requires QUERY"));
    Ok(())
}
