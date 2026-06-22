mod support;

use lkjagent_tools::dispatch::{dispatch, DispatchOutput};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn raw_write_rejects_large_payload() -> TestResult<()> {
    let workspace = temp_workspace("fs-payload-write")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let content = "x".repeat(1_801);

    let output = dispatch(
        &action("fs.write", &[("path", "big.md"), ("content", &content)]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&output));
    assert!(output.content.contains("payload too large for fs.write"));
    assert!(output.content.contains("use fs.batch_write"));
    Ok(())
}

#[test]
fn batch_write_rejects_total_payload_limit() -> TestResult<()> {
    let workspace = temp_workspace("fs-payload-batch")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let body = "x".repeat(1_501);
    let files = format!(
        "path: a.md\ncontent:\n{0}\n-- lkjagent-next-file --\npath: b.md\ncontent:\n{0}\n-- lkjagent-next-file --\npath: c.md\ncontent:\n{0}\n-- lkjagent-next-file --\npath: d.md\ncontent:\n{0}\n",
        body
    );

    let output = dispatch(
        &action("fs.batch_write", &[("files", &files)]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&output));
    assert!(output
        .content
        .contains("payload too large for fs.batch_write batch"));
    Ok(())
}

fn is_error(output: &DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
