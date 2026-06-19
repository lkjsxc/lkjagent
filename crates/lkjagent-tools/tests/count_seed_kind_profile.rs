mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn playbook_profiles_as_guide() -> TestResult<()> {
    let workspace = temp_workspace("count-kind-playbook")?;
    scaffold(
        &workspace,
        "Create about 20 files total for an operations playbook.",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(readme.contains("audit this deliverable as a guide"));
    assert!(first.contains("### Procedure Role"));
    assert!(first.contains("### Execution Commitments"));
    Ok(())
}

#[test]
fn screenplay_profiles_as_narrative() -> TestResult<()> {
    let workspace = temp_workspace("count-kind-screenplay")?;
    scaffold(
        &workspace,
        "Create about 20 files total for a screenplay manuscript.",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(readme.contains("audit this deliverable as a narrative"));
    assert!(first.contains("### Scene Role"));
    assert!(first.contains("### Concrete Commitments"));
    Ok(())
}

#[test]
fn whitepaper_profiles_as_report() -> TestResult<()> {
    let workspace = temp_workspace("count-kind-whitepaper")?;
    scaffold(
        &workspace,
        "Create about 20 files total for a technical whitepaper.",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(readme.contains("audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    assert!(first.contains("### Analysis Commitments"));
    Ok(())
}

fn scaffold(workspace: &Path, objective: &str) -> TestResult<()> {
    scaffold_counted_documents(
        workspace,
        CountGuard {
            kind: CountKind::File,
            target: 20,
            mode: CountMode::Approximate,
        },
        objective,
    )?;
    Ok(())
}
