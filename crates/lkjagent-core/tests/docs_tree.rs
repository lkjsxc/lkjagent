use lkjagent_core::classify::instantiate;
use lkjagent_core::docs_tree::{extract, validate_plan};
use lkjagent_core::model::{Step, StepKind, StepState, TemplateId};

#[test]
fn docs_tree_extraction_handles_exact_approximate_and_japanese_counts() {
    let exact = extract("Create 3 pages of documentation in docs/daemon.");
    assert_eq!(exact.root, "docs/daemon");
    assert_eq!(exact.page_count, Some(3));
    assert!(exact.exact);

    let approx = extract("Create about 4 docs pages for the daemon.");
    assert_eq!(approx.page_count, Some(4));
    assert!(!approx.exact);

    let japanese = extract("docs/guide に 約 三 pages の documentation を作る");
    assert_eq!(japanese.page_count, Some(3));
    assert!(!japanese.exact);
}

#[test]
fn docs_tree_snapshot_and_plan_validation_report_defects() {
    let snapshot = instantiate(9, "Create 2 pages of documentation in docs/daemon.");
    assert_eq!(snapshot.task.template, TemplateId::DocsTree);
    assert_eq!(snapshot.steps[0].kind, StepKind::Plan);
    assert_eq!(snapshot.steps[1].kind, StepKind::Verify);

    let good = steps(
        &snapshot.steps[0],
        &[
            "docs/daemon/README.md",
            "docs/daemon/setup.md",
            "docs/daemon/run.md",
        ],
    );
    assert!(validate_plan(&snapshot.steps[0], &good).is_ok());

    let missing = steps(
        &snapshot.steps[0],
        &["docs/daemon/setup.md", "docs/daemon/run.md"],
    );
    assert!(validate_plan(&snapshot.steps[0], &missing).is_err_and(|e| e.contains("README")));

    let escaped = steps(
        &snapshot.steps[0],
        &["other/README.md", "docs/daemon/run.md"],
    );
    assert!(validate_plan(&snapshot.steps[0], &escaped).is_err_and(|e| e.contains("escapes")));

    let count = steps(
        &snapshot.steps[0],
        &["docs/daemon/README.md", "docs/daemon/run.md"],
    );
    assert!(validate_plan(&snapshot.steps[0], &count).is_err_and(|e| e.contains("count")));
}

fn steps(parent: &Step, paths: &[&str]) -> Vec<Step> {
    paths
        .iter()
        .enumerate()
        .map(|(index, path)| Step {
            id: 100 + index as u64,
            task_id: parent.task_id,
            ordinal: index as u32 + 2,
            kind: StepKind::Write,
            title: path.to_string(),
            instruction: "write docs".to_string(),
            inputs: String::new(),
            output_path: Some(path.to_string()),
            checks: Vec::new(),
            state: StepState::Pending,
            attempts_used: 0,
            actions_used: 0,
            action_budget: 0,
            split_used: false,
        })
        .collect()
}
