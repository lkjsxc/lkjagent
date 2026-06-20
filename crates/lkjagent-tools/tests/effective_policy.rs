mod support;

use lkjagent_tools::dispatch::{dispatch, EffectivePolicy, GraphDispatchPolicy};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn maintenance_memory_save_not_blocked_by_graph_policy() -> TestResult<()> {
    let workspace = temp_workspace("effective-maintenance-save")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(owner_graph_policy(vec!["graph.state"]));
    state.effective_policy = Some(maintenance_policy());

    let saved = dispatch(
        &action(
            "memory.save",
            &[
                ("kind", "lesson"),
                ("title", "Maintenance Dedupe"),
                ("content", "Deduplicate before saving maintenance lessons."),
                ("tags", "maintenance,memory"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(saved.kind, OutputKind::Observation { .. }));
    assert!(saved.content.contains("memory_id="));
    assert!(!saved.content.contains("graph policy refused"));
    Ok(())
}

#[test]
fn maintenance_graph_state_blocked_by_maintenance_policy_only() -> TestResult<()> {
    let workspace = temp_workspace("effective-maintenance-block")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(owner_graph_policy(vec!["graph.state"]));
    state.effective_policy = Some(maintenance_policy());

    let refused = dispatch(
        &action("graph.state", &[]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("effective policy refused graph.state"));
    assert!(refused.content.contains("active_mode=Maintenance"));
    assert!(!refused.content.contains("graph policy refused"));
    Ok(())
}

#[test]
fn owner_task_memory_save_blocked_when_graph_disallows() -> TestResult<()> {
    let workspace = temp_workspace("effective-owner-block")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_policy = Some(owner_graph_policy(vec!["graph.state"]));
    state.effective_policy = Some(owner_policy(vec!["graph.state"]));

    let refused = dispatch(
        &action(
            "memory.save",
            &[
                ("kind", "lesson"),
                ("title", "Blocked"),
                ("content", "Should not save from this owner node."),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(matches!(refused.kind, OutputKind::Notice { .. }));
    assert!(refused.content.contains("active_mode=OwnerTask"));
    assert!(refused.content.contains("effective policy refused memory.save"));
    Ok(())
}

fn maintenance_policy() -> EffectivePolicy {
    EffectivePolicy {
        mode: "Maintenance".to_string(),
        allowed_tools: vec![
            "memory.find".to_string(),
            "memory.prune".to_string(),
            "memory.save".to_string(),
            "queue.list".to_string(),
            "agent.done".to_string(),
        ],
        blocked_tools: vec!["graph.state".to_string()],
        shell_allowed: false,
        completion_allowed: true,
        reason: "tool is not admitted by Maintenance active mode".to_string(),
        preferred_next_action: "memory.find".to_string(),
    }
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
        active_node: "plan".to_string(),
        phase: "planning".to_string(),
        allowed_tools: allowed.into_iter().map(str::to_string).collect(),
        blocked_tools: vec!["memory.save".to_string()],
        allowed_packages: Vec::new(),
        legal_transitions: Vec::new(),
        evidence_requirements: vec!["plan".to_string()],
        blocked_reason: None,
        plan_ready: false,
        completion_ready: false,
        shell_allowed: false,
    }
}
