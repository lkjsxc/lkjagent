mod support;

use std::fs;

use lkjagent_protocol::registry::TOOLS;
use lkjagent_protocol::{parse_completion, Action};
use lkjagent_tools::dispatch::{dispatch, registry_valid_example, validate_action, DispatchOutput};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_note_valid_example_uses_allowed_kind() -> TestResult<()> {
    let example = example_for("graph.note")?;
    assert!(example.contains("<kind>decision</kind>"));
    assert!(example.contains("<summary>Chose smaller recovery action</summary>"));

    let parsed = parse_example(&example)?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    let output = dispatch_example(parsed)?;

    assert!(output.content.contains("graph note recorded"));
    assert!(output.content.contains("kind=decision"));
    Ok(())
}

#[test]
fn graph_note_invalid_kind_lists_allowed_values() -> TestResult<()> {
    let output = dispatch_example(action(
        "graph.note",
        &[("kind", "observation"), ("summary", "wrong note kind")],
    ))?;

    assert!(is_error(&output));
    assert!(output.content.contains("unknown graph.note kind"));
    assert!(output.content.contains("constraint, assumption, risk"));
    assert!(output.content.contains("<kind>decision</kind>"));
    Ok(())
}

#[test]
fn graph_evidence_valid_example_uses_observation_kind() -> TestResult<()> {
    let example = example_for("graph.evidence")?;
    assert!(example.contains("<kind>observation</kind>"));
    assert!(example.contains("<summary>Read README.md</summary>"));

    let parsed = parse_example(&example)?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    let output = dispatch_example(parsed)?;

    assert!(output.content.contains("graph evidence recorded"));
    assert!(output.content.contains("kind=observation"));
    Ok(())
}

#[test]
fn memory_save_valid_example_uses_allowed_kind() -> TestResult<()> {
    let example = example_for("memory.save")?;
    assert!(example.contains("<kind>lesson</kind>"));

    let output = dispatch_example(parse_example(&example)?)?;
    assert!(output.content.contains("memory_id=1"));
    Ok(())
}

#[test]
fn fs_stat_example_dispatches() -> TestResult<()> {
    let output = dispatch_example(parse_example(&example_for("fs.stat")?)?)?;
    assert!(output.content.contains("path=README.md"));
    assert!(output.content.contains("kind=file"));
    Ok(())
}

#[test]
fn doc_scaffold_example_dispatches() -> TestResult<()> {
    let output = dispatch_example(parse_example(&example_for("doc.scaffold")?)?)?;
    assert!(output.content.contains("document scaffold created"));
    assert!(output.content.contains("root=docs"));
    Ok(())
}

#[test]
fn all_rendered_examples_parse() -> TestResult<()> {
    for spec in TOOLS {
        parse_example(&example_for(spec.name)?)?;
    }
    Ok(())
}

#[test]
fn all_registry_examples_validate() -> TestResult<()> {
    for spec in TOOLS {
        let example = example_for(spec.name)?;
        let parsed = parse_example(&example)?;
        validate_action(&parsed).map_err(|err| {
            format!(
                "example for {} failed validation: {err}\n{example}",
                spec.name
            )
        })?;
    }
    Ok(())
}

fn example_for(tool: &str) -> TestResult<String> {
    registry_valid_example(tool)
        .ok_or_else(|| format!("missing registry valid example for {tool}").into())
}

fn parse_example(example: &str) -> TestResult<Action> {
    parse_completion(example).map_err(|err| format!("parse failed: {err:?}").into())
}

fn dispatch_example(action: Action) -> TestResult<DispatchOutput> {
    dispatch_example_with_state(action, state())
}

fn dispatch_example_with_state(
    action: Action,
    mut state: lkjagent_tools::dispatch::DispatchState,
) -> TestResult<DispatchOutput> {
    let workspace = temp_workspace("registry-example")?;
    fs::write(workspace.join("README.md"), "# Example\n")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    Ok(dispatch(&action, &runtime, &mut conn, &mut state))
}

fn is_error(output: &DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
