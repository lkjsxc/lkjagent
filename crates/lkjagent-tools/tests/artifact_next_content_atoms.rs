mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn report_next_uses_content_atom_contract() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-report-atoms")?;
    let root = "reports/market-map";
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    seed_report_root(&runtime.workspace, root)?;

    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "report")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("artifact_atom_profile=report"), "{output}");
    assert!(output.contains("atom_missing_count=5"), "{output}");
    assert!(
        output.contains("next_atom=executive-summary.md"),
        "{output}"
    );
    assert!(
        output.contains("reports/market-map/executive-summary.md"),
        "{output}"
    );
    assert!(
        output.contains("candidate_action=fs.batch_write"),
        "{output}"
    );
    Ok(())
}

fn seed_report_root(workspace: &std::path::Path, root: &str) -> TestResult<()> {
    let root = workspace.join(root);
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"report\"\n")?;
    fs::write(
        root.join("README.md"),
        "# Market Map\n\n## Purpose\n\nNavigate the market report.\n",
    )?;
    fs::write(
        root.join("objective.md"),
        "# Objective\n\n## Purpose\n\nDefine the requested market report root identity.\n",
    )?;
    Ok(())
}
