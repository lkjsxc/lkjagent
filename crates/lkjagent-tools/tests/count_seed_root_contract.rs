mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_root_audit_matches_generated_counts() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-root-audit")?;

    let report = scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about 100 files total for a large story with docs and main content.",
    )?;

    let readme = fs::read_to_string(workspace.join("structured-output/README.md"))?;
    assert!(readme.contains("Design coverage: 12 design memos"));
    assert!(readme.contains("Main coverage: 85 main files"));
    assert!(readme.contains("Content contract: every main file carries"));
    assert!(readme.contains("Kind contract: audit this deliverable as a narrative"));
    assert!(report.contains("acceptance_audit=ok"));
    assert!(report.contains("file_budget=ok"));
    Ok(())
}
