mod support;

use std::fs;

use lkjagent_tools::dispatch::{dispatch, dispatch_with_text};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn fs_tools_read_write_edit_and_report_errors() -> TestResult<()> {
    let workspace = temp_workspace("fs")?;
    let mut runtime = runtime(workspace.clone())?;
    runtime.observation_tokens = 128;
    let mut conn = store()?;
    let mut state = state();
    fs::write(workspace.join("notes.md"), "one\ntwo\n")?;

    let read = dispatch(
        &action("fs.read", &[("path", "notes.md"), ("count", "1")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(read.content.contains("path=notes.md"));
    assert!(read.content.contains("one\n"));

    let write = dispatch(
        &action("fs.write", &[("path", "out/new.md"), ("content", "secret")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(write.content.contains("bytes=6"));
    assert!(!write.content.contains("secret"));
    assert_eq!(fs::read_to_string(workspace.join("out/new.md"))?, "secret");

    let edit = dispatch(
        &action(
            "fs.edit",
            &[("path", "notes.md"), ("find", "two"), ("replace", "second")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(edit.content.contains("line=2"));
    assert_eq!(
        fs::read_to_string(workspace.join("notes.md"))?,
        "one\nsecond\n"
    );

    fs::write(workspace.join("dups.md"), "same\nsame\n")?;
    let many = dispatch(
        &action(
            "fs.edit",
            &[("path", "dups.md"), ("find", "same"), ("replace", "one")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&many));
    assert!(many.content.contains("find matched 2 times"));
    Ok(())
}

#[test]
fn duplicate_read_refuses_unchanged_region() -> TestResult<()> {
    let workspace = temp_workspace("duplicate-read")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    fs::write(workspace.join("same.md"), "alpha\n")?;
    let read = action("fs.read", &[("path", "same.md"), ("count", "1")]);

    let first = dispatch_with_text(&read, "first-act", &runtime, &mut conn, &mut state);
    let second = dispatch_with_text(&read, "second-act", &runtime, &mut conn, &mut state);

    assert!(first.content.contains("alpha"));
    assert!(matches!(second.kind, OutputKind::Notice { .. }));
    assert!(second.content.contains("duplicate read refused"));
    Ok(())
}

#[test]
fn shell_reports_exit_truncates_and_times_out() -> TestResult<()> {
    let workspace = temp_workspace("shell")?;
    let mut runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();

    let exited = dispatch(
        &action("shell.run", &[("command", "printf 'alpha\\n'; exit 7")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(exited.content.contains("exit_code=7"));
    assert!(exited.content.contains("alpha"));

    runtime.observation_tokens = 60;
    let capped = dispatch(
        &action(
            "shell.run",
            &[(
                "command",
                "i=0; while [ $i -lt 200 ]; do echo line-$i; i=$((i+1)); done",
            )],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(capped.content.contains("truncated middle"));
    assert!(capped.content.contains("line-0"));
    assert!(capped.content.contains("line-199"));

    let slow = dispatch(
        &action(
            "shell.run",
            &[
                ("command", "printf start; sleep 2; printf end"),
                ("timeout", "1"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&slow));
    assert!(slow.content.contains("timeout_seconds=1"));
    assert!(!slow.content.contains("end"));
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
