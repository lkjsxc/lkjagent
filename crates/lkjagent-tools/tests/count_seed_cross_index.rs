mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_main_ledger_links_parts_to_design_owners() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-cross-index")?;

    let report = scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about 100 files total for a large story with docs and main content.",
    )?;

    let main_index = fs::read_to_string(workspace.join("structured-output/main/README.md"))?;
    let main_part =
        fs::read_to_string(workspace.join(format!("structured-output/{}", support::main_path(8))))?;
    assert!(main_index.contains(&format!(
        "{}: opening / inciting pressure; design: {}",
        support::main_path(1),
        support::design_path(1)
    )));
    assert!(main_index.contains(&format!(
        "- opening: {} through {}",
        support::main_path(1),
        support::main_path(15)
    )));
    assert!(main_index.contains(&format!(
        "- exploration: {} through {}",
        support::main_path(16),
        support::main_path(29)
    )));
    assert!(main_index.contains(&format!(
        "{}: opening / reversal seed; design: {}",
        support::main_path(15),
        support::design_path(3)
    )));
    assert!(main_index.contains(&format!(
        "{}: exploration / choice under pressure; design: {}",
        support::main_path(16),
        support::design_path(3)
    )));
    assert!(main_index.contains(&format!(
        "{}: opening / choice under pressure; design: {}",
        support::main_path(8),
        support::design_path(2)
    )));
    assert!(main_index.contains(&format!(
        "{}: resolution / public consequence; design: {}",
        support::main_path(85),
        support::design_path(12)
    )));
    assert!(main_part.contains(&format!("- Design owner: {}", support::design_path(2))));
    assert!(main_part.contains(&format!("- Previous: {}", support::main_path(7))));
    assert!(main_part.contains(&format!("- Current: {}", support::main_path(8))));
    assert!(main_part.contains(&format!("- Next: {}", support::main_path(9))));
    assert!(main_part.contains("## Local Verification"));
    assert!(report.contains("part_ledger=ok"));
    assert!(report.contains("design_owner_links=ok"));
    assert!(report.contains("local_verification=ok"));
    assert!(report.contains("sequence_paths=ok"));
    assert!(report.contains("verification=ok"));
    Ok(())
}
