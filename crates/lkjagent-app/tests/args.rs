use lkjagent_app::args::{parse, Command};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn parser_accepts_console_command() -> TestResult<()> {
    assert_eq!(parse(["console"])?.command, Command::Console);
    Ok(())
}

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
fn parser_accepts_record_forms() -> TestResult<()> {
    let add = parse(["record", "add", "custom", "Odd", "Task", "--body", "body"])?;
    assert_eq!(
        add.command,
        Command::RecordAdd {
            kind: "custom".to_string(),
            title: "Odd Task".to_string(),
            body: "body".to_string(),
        }
    );
    assert_eq!(
        parse(["record", "list", "custom"])?.command,
        Command::RecordList {
            kind: Some("custom".to_string())
        }
    );
    assert_eq!(
        parse(["record", "link", "rec_1", "record:rec_2"])?.command,
        Command::RecordLink {
            id: "rec_1".to_string(),
            target: "record:rec_2".to_string(),
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
