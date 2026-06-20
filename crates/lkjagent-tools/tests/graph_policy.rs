mod support;

use lkjagent_tools::dispatch::{dispatch, GraphDispatchPolicy};
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
    Ok(())
}
