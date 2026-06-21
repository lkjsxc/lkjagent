mod support;

use lkjagent_tools::dispatch::{dispatch, GraphDispatchPolicy};
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_recover_excludes_graph_plan_when_not_admitted() -> TestResult<()> {
    let output = recover(policy(vec!["graph.recover", "fs.list"], false))?;

    assert!(output.contains("next=use fs.list"));
    assert!(!output.contains("graph.plan"));
    Ok(())
}

#[test]
fn graph_recover_excludes_graph_plan_after_plan_ready() -> TestResult<()> {
    let output = recover(policy(vec!["graph.recover", "graph.plan", "fs.list"], true))?;

    assert!(output.contains("next=use fs.list"));
    assert!(!output.contains("next=use graph.plan"));
    Ok(())
}

#[test]
fn graph_recover_includes_graph_plan_only_when_needed() -> TestResult<()> {
    let output = recover(policy(
        vec!["graph.recover", "graph.plan", "fs.list"],
        false,
    ))?;

    assert!(output.contains("next=use graph.plan"));
    Ok(())
}

fn recover(policy: GraphDispatchPolicy) -> TestResult<String> {
    let workspace = temp_workspace("graph-recover")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    dispatch_state.graph_policy = Some(policy);
    Ok(dispatch(
        &action("graph.recover", &[]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content)
}

fn policy(allowed_tools: Vec<&str>, plan_ready: bool) -> GraphDispatchPolicy {
    GraphDispatchPolicy {
        active_node: "recover-repeat".to_string(),
        phase: "recovery".to_string(),
        allowed_tools: allowed_tools.into_iter().map(str::to_string).collect(),
        blocked_tools: Vec::new(),
        allowed_packages: Vec::new(),
        legal_transitions: Vec::new(),
        evidence_requirements: Vec::new(),
        blocked_reason: None,
        plan_ready,
        completion_ready: false,
        shell_allowed: false,
    }
}
