mod support;

use lkjagent_tools::dispatch::dispatch;
use std::fs;
use std::path::Path;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn approx_count_accepts_extra_markdown_files() -> TestResult<()> {
    let workspace = temp_workspace("doc-count-approx-extra")?;
    let root = workspace.join("docs");
    fs::create_dir_all(&root)?;
    fs::write(root.join("README.md"), readme())?;
    for name in ["a.md", "b.md", "c.md", "d.md"] {
        fs::write(
            root.join(name),
            format!("# {name}\n\n## Purpose\n\nBody.\n"),
        )?;
    }

    let audit = audit(&workspace, &[("root", "docs"), ("count", "3")])?;

    assert!(audit.contains("document audit passed"), "{audit}");
    assert!(!audit.contains("count_mismatch"));
    Ok(())
}

fn readme() -> &'static str {
    "# Docs\n\n## Purpose\n\nDocs.\n\n## Table of Contents\n\n- [A](a.md)\n- [B](b.md)\n- [C](c.md)\n- [D](d.md)\n"
}

fn audit(workspace: &Path, params: &[(&str, &str)]) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut state = state();
    Ok(dispatch(
        &action("doc.audit", params),
        &runtime,
        &mut conn,
        &mut state,
    )
    .content)
}
