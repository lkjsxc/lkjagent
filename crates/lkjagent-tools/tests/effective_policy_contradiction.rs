mod support;

use lkjagent_tools::dispatch::{dispatch, EffectivePolicy, GraphDispatchPolicy};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn plan_missing_policy_contradiction_gets_admitted_alternate() -> TestResult<()> {
    let workspace = temp_workspace("effective-plan-contradiction")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_missing = vec!["plan".to_string()];
    state.graph_policy = Some(owner_graph_policy(vec!["agent.done"]));
    state.effective_policy = Some(owner_policy(
        vec!["agent.done", "fs.read"],
        vec!["graph.plan"],
        "graph.plan",
    ));

    let refused = dispatch(
        &action(
            "memory.save",
            &[("kind", "lesson"), ("title", "x"), ("content", "y")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("policy contradiction"));
    assert!(refused.content.contains("preferred_next_action=fs.read"));
    assert!(refused.content.contains("<tool>fs.read</tool>"));
    assert!(!refused.content.contains("preferred_next_action=graph.plan"));
    Ok(())
}

#[test]
fn completion_refusal_uses_effective_admitted_graph_plan() -> TestResult<()> {
    let workspace = temp_workspace("effective-plan-admitted")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_missing = vec!["plan".to_string()];
    state.graph_policy = Some(owner_graph_policy(vec!["agent.done"]));
    state.effective_policy = Some(owner_policy(
        vec!["agent.done", "graph.plan", "fs.read"],
        vec![],
        "graph.plan",
    ));

    let refused = dispatch(
        &action("agent.done", &[("summary", "finished")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("missing=plan"));
    assert!(refused
        .content
        .contains("next_executable_action=graph.plan"));
    assert!(refused.content.contains("<tool>graph.plan</tool>"));
    Ok(())
}

fn owner_policy(allowed: Vec<&str>, blocked: Vec<&str>, preferred: &str) -> EffectivePolicy {
    EffectivePolicy {
        mode: "OwnerTask".to_string(),
        allowed_tools: allowed.into_iter().map(str::to_string).collect(),
        blocked_tools: blocked.into_iter().map(str::to_string).collect(),
        shell_allowed: false,
        completion_allowed: false,
        reason: "tool is not admitted by the active graph node".to_string(),
        preferred_next_action: preferred.to_string(),
    }
}

fn owner_graph_policy(allowed: Vec<&str>) -> GraphDispatchPolicy {
    GraphDispatchPolicy {
        active_node: "execute".to_string(),
        phase: "execution".to_string(),
        allowed_tools: allowed.into_iter().map(str::to_string).collect(),
        blocked_tools: vec!["graph.plan".to_string(), "memory.save".to_string()],
        allowed_packages: Vec::new(),
        legal_transitions: Vec::new(),
        evidence_requirements: vec!["plan".to_string()],
        blocked_reason: Some("plan evidence missing".to_string()),
        plan_ready: false,
        completion_ready: false,
        shell_allowed: false,
    }
}
