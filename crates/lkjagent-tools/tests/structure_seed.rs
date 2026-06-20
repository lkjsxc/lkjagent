mod support;

use std::fs;

use lkjagent_tools::structure::verify_recursive_tree;
use lkjagent_tools::structure_network::verify_knowledge_network;
use lkjagent_tools::structure_seed::{scaffold_profile, scaffold_recursive_docs, ScaffoldProfile};
use support::{temp_workspace, TestResult};

#[test]
fn scaffold_recursive_docs_satisfies_completion_guard() -> TestResult<()> {
    let workspace = temp_workspace("structure-seed")?;

    let report = scaffold_recursive_docs(&workspace)?;

    assert!(report.contains("verification=ok"));
    assert!(report.contains("graph=docs/.lkj-doc-graph.md"));
    verify_recursive_tree(&workspace)?;
    assert!(workspace.join("docs/.lkj-doc-graph.md").exists());
    Ok(())
}

#[test]
fn knowledge_scaffold_satisfies_network_guard() -> TestResult<()> {
    let workspace = temp_workspace("knowledge-seed")?;

    let report = scaffold_profile(&workspace, ScaffoldProfile::Knowledge)?;

    assert!(report.contains("profile=knowledge"));
    assert!(report.contains("graph=docs/.lkj-doc-graph.md"));
    assert!(report.contains("growth=incremental"));
    verify_knowledge_network(&workspace)?;
    assert!(workspace.join("docs/.lkj-doc-graph.md").exists());
    assert!(workspace.join("docs/maps/concept-network.md").exists());
    assert!(workspace.join("docs/current-state.md").exists());
    assert!(workspace.join("docs/execution/expansion-queue.md").exists());
    assert!(workspace.join("docs/execution/rebalance-plan.md").exists());
    assert!(workspace.join("docs/reference/ontology.md").exists());
    assert!(!workspace.join("docs/timelines").exists());
    assert!(!workspace.join("docs/questions").exists());
    assert!(markdown_count(&workspace.join("docs"))? <= 25);
    Ok(())
}

#[test]
fn knowledge_guard_rejects_non_contract_state_anchor() -> TestResult<()> {
    let workspace = temp_workspace("knowledge-contract")?;
    scaffold_profile(&workspace, ScaffoldProfile::Knowledge)?;
    fs::write(
        workspace.join("docs/current-state.md"),
        "# Broken - State\n\nNo purpose section.\n",
    )?;

    let error = verify_knowledge_network(&workspace)
        .unwrap_err()
        .to_string();

    assert!(error.contains("docs_contract="));
    assert!(error.contains("docs/current-state.md is missing ## Purpose"));
    Ok(())
}

fn markdown_count(path: &std::path::Path) -> TestResult<usize> {
    let mut count: usize = 0;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count = count.saturating_add(markdown_count(&child)?);
        } else if child.extension().is_some_and(|extension| extension == "md") {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
