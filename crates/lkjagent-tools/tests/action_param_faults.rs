mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn fs_batch_write_accepts_json_payload_inside_files() -> TestResult<()> {
    let workspace = temp_workspace("batch-json-fault")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    let json_payload = r##"[{"path":"out/one.md","content":"# One"}]"##;

    let output = dispatch(
        &action("fs.batch_write", &[("files", json_payload)]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(output.kind, OutputKind::Observation { status } if status == "ok"));
    assert!(output.content.contains("input_format=json-array"));
    assert!(workspace.join("out/one.md").is_file());
    Ok(())
}

#[test]
fn shell_run_timeout_without_command_gets_schema_repair() -> TestResult<()> {
    let workspace = temp_workspace("shell-missing-command")?;
    fs::write(workspace.join("marker.txt"), "unchanged\n")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();

    let output = dispatch(
        &action("shell.run", &[("timeout", "30")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&output));
    assert!(output.content.contains("missing=command"));
    assert!(output.content.contains("valid_example:"));
    assert!(output.content.contains("<tool>shell.run</tool>"));
    assert_eq!(
        fs::read_to_string(workspace.join("marker.txt"))?,
        "unchanged\n"
    );
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(&output.kind, OutputKind::Observation { status } if status == "error")
        || matches!(&output.kind, OutputKind::Notice { kind } if kind == "error")
}
