mod support;

use lkjagent_tools::structure::verify_recursive_tree;
use lkjagent_tools::structure_seed::scaffold_recursive_docs;
use support::{temp_workspace, TestResult};

#[test]
fn scaffold_recursive_docs_satisfies_completion_guard() -> TestResult<()> {
    let workspace = temp_workspace("structure-seed")?;

    let report = scaffold_recursive_docs(&workspace)?;

    assert!(report.contains("verification=ok"));
    verify_recursive_tree(&workspace)?;
    Ok(())
}
