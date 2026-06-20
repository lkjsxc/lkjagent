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
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    let second = fs::read_to_string(root.join(support::main_path(2)))?;
    assert!(readme.contains("audit this deliverable as a guide"));
    assert!(first.contains("### Procedure Role"));
    assert!(first.contains("### Execution Commitments"));
    assert!(first.contains("workspace boundary"));
    assert!(second.contains("handoff packet"));
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
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    let second = fs::read_to_string(root.join(support::main_path(2)))?;
    assert!(readme.contains("audit this deliverable as a narrative"));
    assert!(first.contains("### Scene Role"));
    assert!(first.contains("### Concrete Commitments"));
    assert!(first.contains("### Specific Detail"));
    assert!(first.contains("archive stair"));
    assert!(second.contains("cold signal tower"));
    Ok(())
}

#[test]
fn large_novel_varies_first_and_final_segments() -> TestResult<()> {
    let workspace = temp_workspace("count-kind-large-novel-variation")?;
    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a large fantasy novel. Count docs and main \
         chapters together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    let last = fs::read_to_string(root.join(support::main_path(85)))?;
    assert!(first.contains("archive stair"));
    assert!(!last.contains("archive stair"));
    assert!(!last.contains("justifies the next part"));
    assert!(!last.contains("pressure for the next segment"));
    assert!(last.contains("completed state rather than a next-step hook"));
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
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    let second = fs::read_to_string(root.join(support::main_path(2)))?;
    assert!(readme.contains("audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    assert!(first.contains("### Analysis Commitments"));
    assert!(first.contains("usage evidence"));
    assert!(second.contains("risk evidence"));
    Ok(())
}

#[test]
fn training_course_profiles_as_guide() -> TestResult<()> {
    let workspace = temp_workspace("count-kind-training-course")?;
    scaffold(
        &workspace,
        "Create about 20 files total for a training course pack with lesson plan files.",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("audit this deliverable as a guide"));
    assert!(first.contains("### Procedure Role"));
    assert!(first.contains("### Execution Commitments"));
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
