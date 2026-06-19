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
    assert!(main_index
        .contains("main/part-001.md: opening / inciting pressure; design: docs/design-001.md"));
    assert!(main_index
        .contains("main/part-008.md: opening / choice under pressure; design: docs/design-002.md"));
    assert!(main_index
        .contains("main/part-085.md: resolution / public consequence; design: docs/design-012.md"));
    assert!(report.contains("part_ledger=ok"));
    assert!(report.contains("verification=ok"));
    Ok(())
}
