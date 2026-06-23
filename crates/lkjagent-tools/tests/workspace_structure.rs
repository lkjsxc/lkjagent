mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::structure::verify_recursive_tree;
use support::{temp_workspace, TestResult};

#[test]
fn recursive_guard_rejects_scaffold_only_leaves() -> TestResult<()> {
    let workspace = temp_workspace("workspace-structure-weak")?;
    write_weak_tree(&workspace)?;

    let error = match verify_recursive_tree(&workspace) {
        Ok(()) => return Err("recursive guard accepted scaffold-only leaves".into()),
        Err(error) => error.to_string(),
    };

    assert!(error.contains("weak_markdown_files=6"));
    assert!(error.contains("weak_markdown_paths="));
    Ok(())
}

fn write_weak_tree(workspace: &Path) -> TestResult<()> {
    for dir in [
        "docs",
        "docs/a",
        "docs/a/deep",
        "docs/a/deep/more",
        "docs/b",
        "docs/c",
    ] {
        let path = workspace.join(dir);
        fs::create_dir_all(&path)?;
        fs::write(
            path.join("README.md"),
            format!(
                "# Index\n\n## Purpose\n\nIndexed directory for {dir}.\n\n## Table of Contents\n\n- [leaf.md](leaf.md): leaf.\n"
            ),
        )?;
    }
    for index in 1..=6 {
        let dir = if index % 2 == 0 { "docs/b" } else { "docs/a" };
        fs::write(
            workspace.join(format!("{dir}/leaf-{index}.md")),
            format!(
                "# Leaf {index}\n\n## Purpose\n\nThis file records the generated role for topic {index}.\n"
            ),
        )?;
    }
    Ok(())
}
