mod support;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use lkjagent_protocol::parse_completion;
use lkjagent_tools::dispatch::{dispatch, validate_action};
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_next_missing_root_suggests_apply() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-missing-root")?;
    let output = run(
        &workspace,
        action("artifact.next", &[("root", "cookbooks/bread")]),
    )?;

    assert!(output.contains("missing=root"));
    assert!(output.contains("next_decision_required=true"));
    assert!(output.contains("candidate_action=artifact.apply"));
    assert!(!output.contains("next_action=artifact.apply"));
    assert!(output.contains("<tool>artifact.apply</tool>"));
    Ok(())
}

#[test]
fn artifact_next_for_scaffolded_cookbook_returns_batch_write() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-cookbook")?;
    let root = "cookbooks/bread-cookbook";
    run(
        &workspace,
        action(
            "artifact.apply",
            &[
                ("root", root),
                ("title", "Bread Cookbook"),
                ("kind", "cookbook"),
            ],
        ),
    )?;

    let output = run(&workspace, action("artifact.next", &[("root", root)]))?;
    let example = valid_example_from(&output)?;
    let parsed = parse_completion(example).map_err(|err| format!("parse failed: {err:?}"))?;

    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    assert!(output.contains("kind=cookbook"));
    assert!(output.contains("runtime_event=ArtifactWeakPathFound"));
    assert!(output.contains("candidate_action=fs.batch_write"));
    assert!(!output.contains("next_action=fs.batch_write"));
    assert!(output.contains("- foundations/"));
    assert_no_scaffold_phrases(&output);
    assert_eq!(parsed.tool, "fs.batch_write");
    Ok(())
}

#[test]
fn artifact_next_advances_cursor_then_requests_audit() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-cursor")?;
    let root = "cookbooks/bread-cookbook";
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    dispatch(
        &action(
            "artifact.apply",
            &[
                ("root", root),
                ("title", "Bread Cookbook"),
                ("kind", "cookbook"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    let mut seen = BTreeSet::new();
    let mut requested_audit = false;
    for _ in 0..10 {
        dispatch_state.reset_repeat_tracking();
        let output = dispatch(
            &action("artifact.next", &[("root", root)]),
            &runtime,
            &mut conn,
            &mut dispatch_state,
        )
        .content;
        if output.contains("candidate_action=artifact.audit") {
            requested_audit = true;
            break;
        }
        for path in next_paths(&output) {
            assert!(seen.insert(path), "artifact.next repeated weak path");
        }
    }

    assert!(requested_audit, "artifact.next did not exhaust to audit");
    assert!(lkjagent_store::state::get(&conn, &format!("artifact.next cursor {root}"))?.is_some());
    Ok(())
}

#[test]
fn artifact_audit_passes_meaningful_cookbook() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-cookbook-pass")?;
    let root = "cookbooks/bread-cookbook";
    run(
        &workspace,
        action(
            "artifact.apply",
            &[
                ("root", root),
                ("title", "Bread Cookbook"),
                ("kind", "cookbook"),
            ],
        ),
    )?;
    replace_leaves(&workspace.join(root))?;

    let audit = run(
        &workspace,
        action("artifact.audit", &[("root", root), ("kind", "cookbook")]),
    )?;
    let next = run(&workspace, action("artifact.next", &[("root", root)]))?;

    assert!(audit.contains("artifact audit passed"));
    assert!(next.contains("missing=0"));
    assert!(next.contains("candidate_action=artifact.audit"));
    Ok(())
}

fn run(workspace: &Path, action: lkjagent_protocol::Action) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(&action, &runtime, &mut conn, &mut dispatch_state).content)
}

fn valid_example_from(output: &str) -> TestResult<&str> {
    output
        .split_once("candidate_example:\n")
        .map(|(_, example)| example)
        .ok_or_else(|| "missing valid example".into())
}

fn assert_no_scaffold_phrases(text: &str) {
    for phrase in [
        "Replace this skeleton",
        "Add the requested substance",
        "real cookbook content before dispatch",
    ] {
        assert!(!text.contains(phrase), "scaffold phrase found: {phrase}");
    }
}

fn next_paths(output: &str) -> Vec<String> {
    let Some((_, rest)) = output.split_once("next_paths:\n") else {
        return Vec::new();
    };
    let block = rest
        .split_once("\nrequired_sections:")
        .map_or(rest, |(paths, _)| paths);
    block
        .lines()
        .filter_map(|line| line.strip_prefix("- "))
        .map(str::to_string)
        .collect()
}

fn replace_leaves(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            replace_leaves(&path)?;
        } else if is_leaf_markdown(&path) {
            fs::write(&path, cookbook_leaf_text(&path))?;
        }
    }
    Ok(())
}

fn is_leaf_markdown(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "md")
        && path.file_name().and_then(|name| name.to_str()) != Some("README.md")
}

fn cookbook_leaf_text(path: &Path) -> String {
    let title = path
        .file_stem()
        .and_then(|name| name.to_str())
        .map_or("bread section", |name| name);
    format!(
        "# {title}\n\n## Purpose\n\nThis bread section gives concrete kitchen guidance for the requested cookbook.\n\n## Ingredients Or Concept\n\nIngredients include flour, water, salt, yeast, and optional oil or milk. The concept explains how hydration, gluten development, fermentation, and heat shape the final loaf.\n\n## Method Or Procedure\n\nMethod steps: mix, rest, knead or fold, proof, shape, score, and bake. The procedure names signals to look for, including a domed dough surface, visible bubbles, and a hollow crust sound.\n\n## Timing, Fixes, And Verification\n\nTiming ranges and yield notes guide batch size. Common mistakes include underproofing, weak shaping, and excess flour; corrective action explains how to fix texture, temperature, and bake range. A lookup table records temperatures and troubleshooting notes.\n"
    )
}
