mod support;

use lkjagent_tools::dispatch::{dispatch, dispatch_with_text, GraphDispatchPolicy};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_policy_refuses_shell_without_shell_admission() -> TestResult<()> {
    let workspace = temp_workspace("graph-shell-refusal")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(GraphDispatchPolicy {
        active_node: "verify-focused".to_string(),
        phase: "verification".to_string(),
        allowed_tools: vec!["shell.run".to_string(), "graph.next".to_string()],
        blocked_tools: Vec::new(),
        allowed_packages: Vec::new(),
        legal_transitions: vec!["recover-by-shell-escape".to_string()],
        evidence_requirements: vec!["verification".to_string()],
        blocked_reason: None,
        plan_ready: true,
        completion_ready: false,
        shell_allowed: false,
    });

    let refused = dispatch(
        &action("shell.run", &[("command", "echo no")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("graph policy refused shell.run"));
    assert!(refused.content.contains("verify-focused"));
    assert!(refused.content.contains("preferred_next_action=graph.next"));
    assert!(refused.content.contains("valid_example:"));
    Ok(())
}

#[test]
fn graph_policy_refusal_includes_copyable_allowed_action() -> TestResult<()> {
    let workspace = temp_workspace("graph-refusal-copyable")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(recovery_policy());

    let refused = dispatch(
        &action("fs.write", &[("path", "x.md"), ("content", "x")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("graph policy refused fs.write"));
    assert!(refused
        .content
        .contains("allowed_tools=graph.state, graph.recover, fs.list"));
    assert!(refused
        .content
        .contains("preferred_next_action=graph.recover"));
    assert!(refused.content.contains("<tool>graph.recover</tool>"));
    Ok(())
}

#[test]
fn second_graph_next_in_recovery_forces_alternate_action_class() -> TestResult<()> {
    let workspace = temp_workspace("graph-next-repeat")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(recovery_policy());
    let next = action("graph.next", &[]);
    let text = "<act>\n<tool>graph.next</tool>\n</act>";

    let first = dispatch_with_text(&next, text, &runtime, &mut conn, &mut state);
    let second = dispatch_with_text(&next, text, &runtime, &mut conn, &mut state);

    assert!(first.content.contains("preferred_next_action"));
    assert!(matches!(second.kind, OutputKind::Notice { .. }));
    assert!(second
        .content
        .contains("graph.next already inspected for this fault"));
    assert!(second.content.contains("next_action_must_be=graph.recover"));
    assert!(second.content.contains("<tool>graph.recover</tool>"));
    Ok(())
}

#[test]
fn graph_recover_guidance_does_not_recommend_graph_next_loop() -> TestResult<()> {
    let workspace = temp_workspace("graph-recover-guidance")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(recovery_policy());

    let recovered = dispatch(
        &action("graph.recover", &[]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(recovered.content.contains("next=use graph.transition"));
    assert!(!recovered.content.contains("next=use graph.next"));
    Ok(())
}

#[test]
fn policy_refusal_example_target_is_legal() -> TestResult<()> {
    let workspace = temp_workspace("graph-refusal-legal-transition")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(transition_policy(vec!["document-audit"]));

    let refused = dispatch(
        &action("fs.write", &[("path", "x.md"), ("content", "x")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused
        .content
        .contains("preferred_next_action=graph.transition"));
    assert!(refused.content.contains("<target>document-audit</target>"));
    assert!(!refused.content.contains("<target>plan</target>"));
    Ok(())
}

#[test]
fn policy_refusal_skips_transition_without_legal_targets() -> TestResult<()> {
    let workspace = temp_workspace("graph-refusal-no-legal-transition")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(transition_policy(Vec::new()));

    let refused = dispatch(
        &action("fs.write", &[("path", "x.md"), ("content", "x")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused
        .content
        .contains("preferred_next_action=graph.state"));
    assert!(!refused.content.contains("<tool>graph.transition</tool>"));
    Ok(())
}

fn recovery_policy() -> GraphDispatchPolicy {
    GraphDispatchPolicy {
        active_node: "recover-repeat".to_string(),
        phase: "recovery".to_string(),
        allowed_tools: vec![
            "graph.state".to_string(),
            "graph.recover".to_string(),
            "fs.list".to_string(),
        ],
        blocked_tools: vec!["fs.write".to_string()],
        allowed_packages: Vec::new(),
        legal_transitions: vec!["recover-by-alternate-tool".to_string()],
        evidence_requirements: Vec::new(),
        blocked_reason: Some("mutation blocked until recovery route changes".to_string()),
        plan_ready: true,
        completion_ready: false,
        shell_allowed: false,
    }
}

fn transition_policy(legal_transitions: Vec<&str>) -> GraphDispatchPolicy {
    GraphDispatchPolicy {
        active_node: "document".to_string(),
        phase: "execution".to_string(),
        allowed_tools: vec!["graph.transition".to_string(), "graph.state".to_string()],
        blocked_tools: vec!["fs.write".to_string()],
        allowed_packages: Vec::new(),
        legal_transitions: legal_transitions.into_iter().map(str::to_string).collect(),
        evidence_requirements: Vec::new(),
        blocked_reason: Some("move through admitted document transition".to_string()),
        plan_ready: true,
        completion_ready: false,
        shell_allowed: false,
    }
}
