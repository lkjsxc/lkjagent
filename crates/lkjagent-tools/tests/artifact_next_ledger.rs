mod support;

use lkjagent_store::artifact_cursor::latest_batch_cursor;
use lkjagent_store::artifact_ledger::latest_for_case;
use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_next_records_normalized_batch_cursor() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-ledger-cursor")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let root = "cookbooks/bread";
    dispatch(
        &action(
            "artifact.apply",
            &[("root", root), ("title", "Bread"), ("kind", "cookbook")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    dispatch_state.reset_repeat_tracking();
    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "cookbook")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("next_action=fs.batch_write"));
    let artifact = latest_for_case(&conn, 0)?.ok_or("missing artifact")?;
    assert_eq!(artifact.lifecycle_state, "repair-planned");
    let cursor = latest_batch_cursor(&conn, artifact.id)?.ok_or("missing cursor")?;
    assert_eq!(cursor.root, root);
    assert!(cursor.planned_paths.contains("foundations/"));
    assert!(cursor
        .last_valid_example
        .contains("<tool>fs.batch_write</tool>"));
    assert_eq!(cursor.fallback_mode, "batch-write");
    Ok(())
}
