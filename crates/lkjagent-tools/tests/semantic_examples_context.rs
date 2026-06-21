mod support;

use std::fs;

use lkjagent_protocol::{parse_completion, Action};
use lkjagent_tools::dispatch::{
    dispatch, valid_example_for, DispatchOutput, ExampleContext, GraphDispatchPolicy,
};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_transition_valid_example_uses_legal_target() -> TestResult<()> {
    let example = valid_example_for(
        "graph.transition",
        ExampleContext {
            legal_transitions: vec!["document-audit".to_string()],
            ..ExampleContext::default()
        },
    )?
    .render();

    assert!(example.contains("<target>document-audit</target>"));
    let mut state = state();
    state.graph_policy = Some(policy_with_transition("document-audit"));
    let output = dispatch_example_with_state(parse_example(&example)?, state)?;

    assert!(output.content.contains("graph transition admitted"));
    assert!(output.content.contains("target=document-audit"));
    Ok(())
}

#[test]
fn graph_note_normalizes_attempt_to_decision() -> TestResult<()> {
    let output = dispatch_example(action(
        "graph.note",
        &[
            ("kind", "attempt"),
            ("summary", "Tried a smaller valid action"),
        ],
    ))?;

    assert!(output.content.contains("kind=decision"));
    assert!(output.content.contains("normalized_from=attempt"));
    Ok(())
}

#[test]
fn graph_note_normalizes_document_structure_with_path() -> TestResult<()> {
    let output = dispatch_example(action(
        "graph.note",
        &[
            ("kind", "document-structure"),
            ("summary", "Story manifest path selected"),
            ("path", "stories/example-story/manifest.md"),
        ],
    ))?;

    assert!(output.content.contains("kind=path"));
    assert!(output
        .content
        .contains("normalized_from=document-structure"));
    Ok(())
}

#[test]
fn graph_evidence_rejects_risk_without_policy() -> TestResult<()> {
    let output = dispatch_example(action(
        "graph.evidence",
        &[("kind", "risk"), ("summary", "wrong evidence kind")],
    ))?;

    assert!(is_error(&output));
    assert!(output.content.contains("allowed_values=plan, observation"));
    assert_eq!(output.content.matches("valid_example:").count(), 1);
    assert!(output.content.contains("<kind>plan</kind>"));
    Ok(())
}

fn policy_with_transition(target: &str) -> GraphDispatchPolicy {
    GraphDispatchPolicy {
        active_node: "document".to_string(),
        phase: "execution".to_string(),
        allowed_tools: vec!["graph.transition".to_string()],
        blocked_tools: Vec::new(),
        allowed_packages: Vec::new(),
        legal_transitions: vec![target.to_string()],
        evidence_requirements: Vec::new(),
        blocked_reason: None,
        plan_ready: true,
        completion_ready: false,
        shell_allowed: false,
    }
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
    let workspace = temp_workspace("semantic-example-context")?;
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
