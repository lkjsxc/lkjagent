mod support;

use std::fs;

use lkjagent_store::artifact_graph::{active_contract_for_plan, plan_for_root, readiness_for_plan};
use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_plan_next_write_and_audit_use_atom_graph() -> TestResult<()> {
    let workspace = temp_workspace("artifact-graph-loop")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let root = "reports/market-map";

    let plan = dispatch(
        &action(
            "artifact.plan",
            &[
                ("root", root),
                ("title", "Market Map"),
                ("kind", "report"),
                ("scale", "1200"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(plan.contains("artifact_plan=ready"), "{plan}");
    let row = plan_for_root(&conn, root)?.ok_or("missing plan")?;
    fs::create_dir_all(runtime.workspace.join(root))?;
    fs::write(
        runtime.workspace.join(root).join("catalog.toml"),
        "kind = \"report\"\n",
    )?;
    fs::write(
        runtime.workspace.join(root).join("README.md"),
        "# Market Map\n\n## Purpose\n\nNavigate the report.\n",
    )?;
    fs::write(
        runtime.workspace.join(root).join("objective.md"),
        "# Objective\n\n## Purpose\n\nDefine the report objective.\n",
    )?;

    dispatch_state.reset_repeat_tracking();
    let next = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "report")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(next.contains("active_contract=contract-"), "{next}");
    let contract = active_contract_for_plan(&conn, row.id)?.ok_or("missing contract")?;
    let path = contract.exact_paths.lines().next().ok_or("missing path")?;
    let files = format!("path: {path}\ncontent:\n# Executive Summary\n\n## Purpose\n\nThis analysis purpose paragraph gives concrete market evidence, risks, recommendations, and quantified tradeoffs for the requested report atom. It names actors, constraints, timing, verification notes, and decision pressure in finished prose.\n");

    dispatch_state.reset_repeat_tracking();
    let write = dispatch(
        &action("fs.batch_write", &[("files", &files)]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(write.contains("files_written=1"), "{write}");

    dispatch_state.reset_repeat_tracking();
    let audit = dispatch(
        &action("artifact.audit", &[("root", root), ("kind", "report")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(audit.contains("artifact_profile=report"), "{audit}");
    assert!(readiness_for_plan(&conn, row.id)?.is_some());
    Ok(())
}

#[test]
fn active_contract_refuses_other_batch_path() -> TestResult<()> {
    let workspace = temp_workspace("artifact-graph-refusal")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    dispatch(
        &action(
            "artifact.plan",
            &[
                ("root", "reports/refusal"),
                ("title", "Refusal"),
                ("kind", "report"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    dispatch_state.reset_repeat_tracking();
    dispatch(
        &action(
            "artifact.next",
            &[("root", "reports/refusal"), ("kind", "report")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    dispatch_state.reset_repeat_tracking();
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[("files", "path: reports/refusal/wrong.md\ncontent:\nNo.")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(output.contains("outside"), "{output}");
    Ok(())
}
