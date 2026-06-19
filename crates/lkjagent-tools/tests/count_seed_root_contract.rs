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
    assert!(readme.contains("## Audit Manifest"));
    assert!(readme.contains("- files: 100"));
    assert!(readme.contains("- index_files: 2"));
    assert!(readme.contains("- design_memos: 12"));
    assert!(readme.contains("- main_files: 85"));
    assert!(readme.contains("- restart_guide: required"));
    assert!(readme.contains("- sequence_paths: required"));
    assert!(readme.contains("- completion: ready"));
    assert!(readme.contains("Design coverage: 12 design memos"));
    assert!(readme.contains("Main coverage: 85 main files"));
    assert!(readme.contains("Content contract: every main file carries"));
    assert!(readme.contains("Kind contract: audit this deliverable as a narrative"));
    assert!(readme.contains("## Restart Guide"));
    assert!(readme.contains("docs/README.md"));
    assert!(readme.contains("main/README.md"));
    assert!(readme.contains("Design owner"));
    assert!(readme.contains("Sequence Ledger"));
    assert!(report.contains("audit_manifest=ok"));
    assert!(report.contains("acceptance_audit=ok"));
    assert!(report.contains("file_budget=ok"));
    assert!(report.contains("restart_guide=ok"));
    Ok(())
}

#[test]
fn count_seed_root_manifest_marks_empty_scopes_as_not_applicable() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-root-manifest-small")?;

    let report = scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 1,
            mode: CountMode::Exact,
        },
        "Create one file.",
    )?;

    let readme = fs::read_to_string(workspace.join("structured-output/README.md"))?;
    assert!(readme.contains("- files: 1"));
    assert!(readme.contains("- index_files: 0"));
    assert!(readme.contains("- design_memos: 0"));
    assert!(readme.contains("- main_files: 0"));
    assert!(readme.contains("- index_scope: n/a"));
    assert!(readme.contains("- content_blocks: n/a"));
    assert!(readme.contains("- restart_guide: required"));
    assert!(readme.contains("- sequence_paths: n/a"));
    assert!(readme.contains("No main files exist"));
    assert!(report.contains("audit_manifest=ok"));
    assert!(report.contains("restart_guide=ok"));
    Ok(())
}
