mod support;

use std::fs;

use lkjagent_protocol::{parse_completion, Action};
use lkjagent_tools::dispatch::{
    dispatch, registry_valid_example, valid_example_for, validate_action, DispatchOutput,
    ExampleContext, GraphDispatchPolicy,
};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_note_normalizes_planning_to_decision() -> TestResult<()> {
    let output = dispatch_example(action(
        "graph.note",
        &[
            ("kind", "planning"),
            ("summary", "Use smaller recovery action"),
        ],
    ))?;

    assert!(output.content.contains("graph note recorded"));
    assert!(output.content.contains("kind=decision"));
    assert!(output.content.contains("normalized_from=planning"));
    Ok(())
}

#[test]
fn graph_evidence_valid_example_uses_missing_requirement() -> TestResult<()> {
    let example = valid_example_for(
        "graph.evidence",
        ExampleContext {
            evidence_requirement: Some("verification".to_string()),
            ..ExampleContext::default()
        },
    )?
    .render();

    assert!(example.contains("<kind>verification</kind>"));
    let parsed = parse_example(&example)?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    Ok(())
}

#[test]
fn graph_evidence_rejects_audit_owned_requirement() -> TestResult<()> {
    let mut state = state();
    state.graph_policy = Some(policy_with_evidence(&[
        "plan",
        "observation",
        "document-structure",
        "artifact-readiness",
    ]));
    let output = dispatch_example_with_state(
        action(
            "graph.evidence",
            &[
                ("kind", "artifact-readiness"),
                ("summary", "claimed content readiness"),
                ("path", "stories/example-story"),
            ],
        ),
        state,
    )?;

    assert!(is_error(&output));
    assert!(output.content.contains("audit-owned graph evidence"));
    assert!(output.content.contains("<tool>artifact.audit</tool>"));
    assert!(!output.content.contains("<tool>graph.evidence</tool>"));
    Ok(())
}

#[test]
fn graph_evidence_rejects_document_structure_requirement() -> TestResult<()> {
    let mut state = state();
    state.graph_policy = Some(policy_with_evidence(&[
        "plan",
        "observation",
        "document-structure",
    ]));
    let output = dispatch_example_with_state(
        action(
            "graph.evidence",
            &[
                ("kind", "document-structure"),
                ("summary", "claimed document structure"),
                ("path", "docs"),
            ],
        ),
        state,
    )?;

    assert!(is_error(&output));
    assert!(output
        .content
        .contains("document structure comes from doc.audit"));
    assert!(output.content.contains("<tool>doc.audit</tool>"));
    Ok(())
}

#[test]
fn graph_evidence_rejects_decision_with_known_requirements() -> TestResult<()> {
    let mut state = state();
    state.graph_policy = Some(policy_with_evidence(&[
        "plan",
        "observation",
        "verification",
        "document-structure",
    ]));
    let output = dispatch_example_with_state(
        action(
            "graph.evidence",
            &[("kind", "decision"), ("summary", "wrong evidence kind")],
        ),
        state,
    )?;

    assert!(is_error(&output));
    assert!(output
        .content
        .contains("unknown graph evidence requirement"));
    assert!(output.content.contains("allowed_values=plan, observation"));
    assert_eq!(output.content.matches("valid_example:").count(), 1);
    assert!(output.content.contains("<kind>plan</kind>"));
    Ok(())
}

#[test]
fn graph_plan_valid_example_dispatches() -> TestResult<()> {
    let example = example_for("graph.plan")?;
    assert!(example.contains("<checks>dispatch accepts semantic plan</checks>"));
    assert!(example.contains("<paths>.</paths>"));
    let output = dispatch_example(parse_example(&example)?)?;
    assert!(output.content.contains("graph plan recorded"));
    Ok(())
}

fn example_for(tool: &str) -> TestResult<String> {
    registry_valid_example(tool).ok_or_else(|| format!("missing example for {tool}").into())
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
    let workspace = temp_workspace("semantic-example")?;
    fs::write(workspace.join("README.md"), "# Example\n")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    Ok(dispatch(&action, &runtime, &mut conn, &mut state))
}

fn policy_with_evidence(evidence: &[&str]) -> GraphDispatchPolicy {
    GraphDispatchPolicy {
        active_node: "execute".to_string(),
        phase: "execution".to_string(),
        allowed_tools: vec!["graph.evidence".to_string()],
        blocked_tools: Vec::new(),
        allowed_packages: Vec::new(),
        legal_transitions: Vec::new(),
        evidence_requirements: evidence.iter().map(|item| item.to_string()).collect(),
        blocked_reason: None,
        plan_ready: true,
        completion_ready: false,
        shell_allowed: false,
    }
}

fn is_error(output: &DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
