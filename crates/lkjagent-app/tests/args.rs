use lkjagent_app::args::{parse, Command};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn parser_accepts_only_public_commands() -> TestResult<()> {
    assert_eq!(parse([] as [&str; 0])?.command, Command::Help);
    assert_eq!(parse(["help"])?.command, Command::Help);
    assert_eq!(parse(["run"])?.command, Command::Run { once: false });
    assert_eq!(
        parse(["run", "--once"])?.command,
        Command::Run { once: true }
    );
    assert_eq!(parse(["status"])?.command, Command::Status);
    assert_eq!(parse(["doctor"])?.command, Command::Doctor { json: false });
    assert_eq!(
        parse(["doctor", "--json"])?.command,
        Command::Doctor { json: true }
    );
    assert_eq!(
        parse(["send", "--new", "hello", "owner"])?.command,
        Command::Send {
            text: "hello owner".to_string(),
            force_new: true,
        }
    );
    Ok(())
}

#[test]
fn parser_rejects_removed_commands_and_aliases() -> TestResult<()> {
    for command in [
        "console",
        "workbench",
        "workspace",
        "log",
        "matter",
        "queue",
        "context",
        "record",
        "memory",
        "watch",
        "today",
        "journal",
        "todo",
        "calendar",
        "finance",
        "note",
        "project",
        "artifact",
        "dev",
    ] {
        let error = match parse([command]) {
            Err(error) => error,
            Ok(_) => return Err(format!("removed command parsed: {command}").into()),
        };
        assert_eq!(error, format!("unknown command: {command}"));
    }
    Ok(())
}

#[test]
fn parser_rejects_arguments_outside_the_public_shapes() {
    for args in [
        &["help", "extra"][..],
        &["status", "--json"][..],
        &["doctor", "--once"][..],
        &["run", "--json"][..],
        &["send", "--new"][..],
    ] {
        assert!(parse(args.iter().copied()).is_err(), "accepted {args:?}");
    }
}
