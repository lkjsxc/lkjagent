mod support;

use lkjagent_store::artifact_ledger::{latest_for_case, weak_paths};
use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_plan_and_apply_write_ledger_identity() -> TestResult<()> {
    let workspace = temp_workspace("artifact-ledger-plan-apply")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    dispatch(
        &action(
            "artifact.plan",
            &[
                ("root", "cookbooks/japanese-home"),
                ("title", "Japanese Home"),
                ("kind", "cookbook"),
                ("scale", "small"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    let planned = latest_for_case(&conn, 0)?.ok_or("missing planned artifact")?;
    assert_eq!(planned.kind, "cookbook");
    assert_eq!(planned.lifecycle_state, "identity-ready");

    dispatch_state.reset_repeat_tracking();
    dispatch(
        &action(
            "artifact.apply",
            &[
                ("root", "cookbooks/japanese-home"),
                ("title", "Japanese Home"),
                ("kind", "cookbook"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    let applied = latest_for_case(&conn, 0)?.ok_or("missing applied artifact")?;
    assert_eq!(applied.id, planned.id);
    assert_eq!(applied.lifecycle_state, "adopted-or-scaffolded");
    assert_eq!(applied.readiness_status, "needs-audit");
    Ok(())
}

#[test]
fn artifact_audit_records_failed_readiness_and_weak_paths() -> TestResult<()> {
    let workspace = temp_workspace("artifact-ledger-audit")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    dispatch(
        &action(
            "artifact.apply",
            &[
                ("root", "cookbooks/bread"),
                ("title", "Bread"),
                ("kind", "cookbook"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    dispatch_state.reset_repeat_tracking();
    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "cookbooks/bread"), ("kind", "cookbook")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    let latest = latest_for_case(&conn, 0)?.ok_or("missing audited artifact")?;
    assert!(output
        .content
        .contains(&format!("artifact_ledger_id={}", latest.id)));
    assert_eq!(latest.readiness_status, "failed");
    assert!(latest.weak_path_count > 0);
    assert!(!weak_paths(&conn, latest.id)?.is_empty());
    Ok(())
}
