mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_main_anchor_uses_content_topic_not_count_command() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-content-anchor")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Use GPT-5.3-Codex-Spark thrift. Create about 100 files total for a large structured story deliverable with docs and main content.",
    )?;

    let readme = fs::read_to_string(workspace.join("structured-output/README.md"))?;
    assert!(readme.contains("- Use GPT-5.3-Codex-Spark thrift"));
    let first_part = fs::read_to_string(workspace.join("structured-output/main/part-001.md"))?;
    assert!(first_part.contains(
        "Local objective: Turn \"a large structured story deliverable\" into this file's distinct contribution."
    ));
    assert!(!first_part.contains("Local objective: Turn \"Create about 100 files total"));
    Ok(())
}
