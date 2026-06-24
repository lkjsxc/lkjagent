mod support;

use lkjagent_tools::dispatch::{dispatch, EffectivePolicy, GraphDispatchPolicy};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn runtime_effective_policy_admits_repair_tool_over_graph_completion_node() -> TestResult<()> {
    let workspace = temp_workspace("effective-owner-repair")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(owner_graph_policy(vec!["agent.done", "graph.audit"]));
    state.effective_policy = Some(owner_policy(vec![
        "agent.done",
        "graph.audit",
        "artifact.next",
        "fs.batch_write",
    ]));

    let output = dispatch(
        &action(
            "artifact.next",
            &[("root", "dictionary"), ("kind", "dictionary")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(output.kind, OutputKind::Observation { .. }));
    assert!(output.content.contains("artifact_next_result=root_missing"));
    assert!(!output.content.contains("graph policy refused"));
    Ok(())
}

fn owner_policy(allowed: Vec<&str>) -> EffectivePolicy {
    EffectivePolicy {
        mode: "OwnerTask".to_string(),
        allowed_tools: allowed.into_iter().map(str::to_string).collect(),
        blocked_tools: vec!["memory.save".to_string()],
        shell_allowed: false,
        completion_allowed: false,
        reason: "tool is not admitted by the active graph node".to_string(),
        preferred_next_action: "follow active graph policy".to_string(),
    }
}

fn owner_graph_policy(allowed: Vec<&str>) -> GraphDispatchPolicy {
    GraphDispatchPolicy {
        active_node: "complete".to_string(),
        phase: "completion".to_string(),
        allowed_tools: allowed.into_iter().map(str::to_string).collect(),
        blocked_tools: vec!["artifact.next".to_string(), "fs.batch_write".to_string()],
        allowed_packages: Vec::new(),
        legal_transitions: Vec::new(),
        evidence_requirements: vec!["artifact-readiness".to_string()],
        blocked_reason: Some("completion evidence missing".to_string()),
        plan_ready: true,
        completion_ready: false,
        shell_allowed: false,
    }
}
