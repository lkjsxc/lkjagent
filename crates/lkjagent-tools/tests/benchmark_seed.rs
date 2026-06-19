mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::benchmark_seed::scaffold_markdown_corpus;
use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn benchmark_seed_creates_exact_markdown_only_tree() -> TestResult<()> {
    let workspace = temp_workspace("benchmark-seed")?;

    let report = scaffold_markdown_corpus(&workspace, 12)?;

    assert!(report.contains("markdown_files=12"));
    let root = workspace.join("docs/benchmark-corpus");
    assert_eq!(counts(&root)?.markdown, 12);
    assert_eq!(counts(&root)?.other, 0);
    assert_readmes(&root)?;
    let sample = fs::read_to_string(root.join("api/topic-001.md"))?;
    assert!(sample.contains("# API Topic 1"));
    assert!(sample.contains("## Purpose"));
    Ok(())
}

#[test]
fn count_seed_creates_exact_markdown_tree() -> TestResult<()> {
    let workspace = temp_workspace("count-seed")?;

    let report = scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 20,
            mode: CountMode::Exact,
        },
        "Create a structured archive for the northern expedition.",
    )?;

    assert!(report.contains("files=20"));
    let root = workspace.join("structured-output");
    assert_eq!(counts(&root)?.markdown, 20);
    assert_eq!(counts(&root)?.other, 0);
    assert!(root.join("README.md").exists());
    assert!(root.join("docs/design-001.md").exists());
    assert!(root.join("main/part-001.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("northern expedition"));
    Ok(())
}

fn assert_readmes(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let child = entry?.path();
        if child.is_dir() {
            assert!(child.join("README.md").exists(), "missing {:?}", child);
            assert_readmes(&child)?;
        }
    }
    Ok(())
}

#[derive(Default)]
struct Counts {
    markdown: usize,
    other: usize,
}

fn counts(path: &Path) -> TestResult<Counts> {
    let mut totals = Counts::default();
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            let nested = counts(&child)?;
            totals.markdown = totals.markdown.saturating_add(nested.markdown);
            totals.other = totals.other.saturating_add(nested.other);
        } else if child.extension().is_some_and(|extension| extension == "md") {
            totals.markdown = totals.markdown.saturating_add(1);
        } else {
            totals.other = totals.other.saturating_add(1);
        }
    }
    Ok(totals)
}
