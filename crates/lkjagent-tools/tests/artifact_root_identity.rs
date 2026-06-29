mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn missing_story_root_returns_flat_identity_contract() -> TestResult<()> {
    let workspace = temp_workspace("artifact-root-identity")?;
    let output = run(&workspace, "artifact.next", &[("root", "stories/novel")])?;
    let paths = contract_paths(&output);

    assert_eq!(
        paths,
        vec![
            "stories/novel/catalog.toml",
            "stories/novel/README.md",
            "stories/novel/objective.md",
            "stories/novel/setting-overview.md",
            "stories/novel/cast.md",
        ]
    );
    assert!(!output.contains("request/objective.md"));
    assert!(paths
        .iter()
        .all(|path| flat_under_root(path, "stories/novel")));
    Ok(())
}

#[test]
fn root_identity_content_can_pass_doc_audit() -> TestResult<()> {
    let workspace = temp_workspace("artifact-root-identity-pass")?;
    seed_story_identity(&workspace, "stories/novel")?;

    let audit = run(&workspace, "doc.audit", &[("root", "stories/novel")])?;

    assert!(audit.contains("document audit passed"), "{audit}");
    assert!(audit.contains("content_readiness=passed"), "{audit}");
    Ok(())
}

fn run(workspace: &Path, tool: &str, params: &[(&str, &str)]) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(
        &action(tool, params),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content)
}

fn contract_paths(output: &str) -> Vec<String> {
    let Some((_, rest)) = output.split_once("paths:\n") else {
        return Vec::new();
    };
    rest.lines()
        .map(str::trim)
        .take_while(|line| line.starts_with("- "))
        .map(|line| line.trim_start_matches("- ").to_string())
        .collect()
}

fn flat_under_root(path: &str, root: &str) -> bool {
    let Some(relative) = path.strip_prefix(&format!("{root}/")) else {
        return false;
    };
    !relative.contains('/')
}

fn seed_story_identity(workspace: &Path, root: &str) -> TestResult<()> {
    let root_path = workspace.join(root);
    fs::create_dir_all(&root_path)?;
    fs::write(root_path.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(root_path.join("README.md"), readme())?;
    fs::write(root_path.join("objective.md"), story_leaf("Objective"))?;
    fs::write(
        root_path.join("setting-overview.md"),
        story_leaf("Setting Overview"),
    )?;
    fs::write(root_path.join("cast.md"), story_leaf("Cast"))?;
    Ok(())
}

fn readme() -> &'static str {
    "# Novel\n\n## Purpose\n\nNavigate the story bible identity.\n\n- [Catalog](catalog.toml)\n- [Objective](objective.md)\n- [Setting Overview](setting-overview.md)\n- [Cast](cast.md)\n"
}

fn story_leaf(title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis story bible reference detail records concrete setting facts, continuity note anchors, cast motives, verification note checks, narrative constraints, and owner objective context for the long novel root identity.\n"
    )
}
