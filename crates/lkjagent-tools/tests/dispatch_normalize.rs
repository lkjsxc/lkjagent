mod support;

use lkjagent_protocol::{Action, Param};
use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_state_with_safe_path_is_normalized() -> TestResult<()> {
    let workspace = temp_workspace("normalize-graph")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_state = Some("case=1\nphase=planning".to_string());

    let output = dispatch(
        &action("graph.state", &[("path", ".")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(output.content.contains("action params normalized"));
    assert!(output.content.contains("dropped=path"));
    assert!(output.content.contains("phase=planning"));
    assert_eq!(state.graph_evidence[0].kind, "action-normalization");
    Ok(())
}

#[test]
fn doc_scaffold_path_is_renamed_to_root() -> TestResult<()> {
    let workspace = temp_workspace("normalize-scaffold")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();

    let output = dispatch(
        &action("doc.scaffold", &[("path", "docs"), ("title", "Docs")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(output.content.contains("renamed=path->root"));
    assert!(output.content.contains("root=docs"));
    assert!(workspace.join("docs/catalog.toml").is_file());
    Ok(())
}

#[test]
fn doc_audit_path_is_renamed_to_root() -> TestResult<()> {
    let workspace = temp_workspace("normalize-audit")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();

    let output = dispatch(
        &action("doc.audit", &[("path", "docs")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(output.content.contains("renamed=path->root"));
    assert!(output.content.contains("missing_root: docs"));
    Ok(())
}

#[test]
fn unknown_param_error_prints_valid_example() -> TestResult<()> {
    let workspace = temp_workspace("normalize-refusal")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let bad = Action::new("graph.state", vec![Param::new("path", "docs")]);

    let output = dispatch(&bad, &runtime, &mut conn, &mut state);

    assert!(matches!(output.kind, OutputKind::Notice { .. }));
    assert!(output.content.contains("action params refused"));
    assert!(output.content.contains("expected=no parameters"));
    assert!(output.content.contains("received=path"));
    assert!(output.content.contains("<tool>graph.state</tool>"));
    Ok(())
}

#[test]
fn artifact_apply_unknown_scale_prints_one_canonical_example() -> TestResult<()> {
    let workspace = temp_workspace("normalize-artifact-scale")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let bad = Action::new(
        "artifact.apply",
        vec![
            Param::new("root", "dictionary"),
            Param::new("kind", "dictionary"),
            Param::new("scale", "large"),
        ],
    );

    let output = dispatch(&bad, &runtime, &mut conn, &mut state);

    assert!(matches!(output.kind, OutputKind::Notice { .. }));
    assert!(output.content.contains("unknown=scale"));
    assert_eq!(output.content.matches("valid_example:").count(), 1);
    let Some((_, example)) = output.content.split_once("valid_example:\n") else {
        return Ok(());
    };
    assert!(example.contains("<tool>artifact.apply</tool>"));
    assert!(!example.contains("<scale>"));
    Ok(())
}

#[test]
fn repeated_invalid_schema_does_not_reprint_same_example_loop() -> TestResult<()> {
    let workspace = temp_workspace("normalize-repeat-invalid")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let bad = Action::new("fs.stat", vec![Param::new("bogus", "README.md")]);

    let first = dispatch(&bad, &runtime, &mut conn, &mut state);
    let second = dispatch(&bad, &runtime, &mut conn, &mut state);

    assert!(first.content.contains("action params refused"));
    assert!(first.content.contains("valid_example:"));
    assert!(second.content.contains("repeat action refused"));
    assert!(!second.content.contains("valid_example:"));
    Ok(())
}
