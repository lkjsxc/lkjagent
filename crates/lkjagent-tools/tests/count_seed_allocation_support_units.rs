mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_honors_supporting_exhibits_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-supporting-exhibits")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a market intelligence dossier. Use \
         twenty-four supporting exhibits. The rest as ordered report sections. Count docs and \
         main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "report")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

#[test]
fn count_seed_honors_assessment_rubrics_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-assessment-rubrics")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for an onboarding curriculum. Use twenty-four \
         assessment rubrics. The rest as ordered training modules. Count docs and main content \
         together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "guide")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

#[test]
fn count_seed_honors_review_criteria_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-review-criteria")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a policy manual. Use twenty-four review \
         criteria. The rest as ordered policy sections. Count docs and main content together. \
         Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "guide")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

#[test]
fn count_seed_honors_case_studies_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-case-studies")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a business operations playbook. Use \
         twenty-four case studies. The rest as ordered playbook sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "guide")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

#[test]
fn count_seed_honors_decision_records_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-decision-records")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a product architecture playbook. Use \
         twenty-four decision records. The rest as ordered implementation chapters. Count docs \
         and main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "guide")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

#[test]
fn count_seed_honors_lesson_plans_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-lesson-plans")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a training course. Use twenty-four \
         lesson plans. The rest as ordered training modules. Count docs and main content \
         together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "guide")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

#[test]
fn count_seed_honors_character_sketches_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-character-sketches")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a large fantasy novel. Use twenty-four \
         character sketches. The rest as ordered chapter drafts. Count docs and main content \
         together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "narrative")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

#[test]
fn count_seed_honors_character_bios_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-character-bios")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a large fantasy novel. Use twenty-four \
         character bios. The rest as ordered chapter drafts. Count docs and main content \
         together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "narrative")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

fn file_guard() -> CountGuard {
    CountGuard {
        kind: CountKind::File,
        target: 100,
        mode: CountMode::Approximate,
    }
}

fn assert_counts(root: &Path, design: usize, main: usize, kind: &str) -> TestResult<()> {
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains(&format!("- Design memos: {design}")));
    assert!(readme.contains(&format!("- Main files: {main}")));
    assert!(readme.contains(&format!(
        "Kind contract: audit this deliverable as a {kind}"
    )));
    Ok(())
}
