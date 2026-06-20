mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{StopReason, TaskState};
use lkjagent_store::queue;
use support::http::{completion, serve_responses};
use support::{error_output, repeat_notice, runtime_state, store, temp_workspace, TestResult};

#[test]
fn third_parse_fault_routes_to_graph_recovery() -> TestResult<()> {
    let mut state = open("parse loop")?;
    for _ in 0..2 {
        state = parse_fault(state).state;
    }

    let waited = parse_fault(state);

    assert_eq!(waited.stop_reason, Some(StopReason::InvalidAction));
    assert!(matches!(waited.state.task, TaskState::Open { .. }));
    assert_eq!(
        waited.state.graph.as_ref().map(|graph| graph.active_node.0),
        Some("recover-parse")
    );
    assert!(waited.effects.iter().any(|effect| {
        matches!(
            effect,
            lkjagent_runtime::step::Effect::RecordGraphFault { kind, count, .. }
                if kind == "parse" && *count == 3
        )
    }));
    Ok(())
}

#[test]
fn param_fault_routes_to_recover_params() -> TestResult<()> {
    let mut state = open("parameter loop")?;
    for _ in 0..2 {
        state = param_fault(state).state;
    }

    let routed = param_fault(state);

    assert_eq!(routed.stop_reason, Some(StopReason::BadParams));
    assert_eq!(
        routed.state.graph.as_ref().map(|graph| graph.active_node.0),
        Some("recover-params")
    );
    assert!(routed.effects.iter().any(|effect| {
        matches!(
            effect,
            lkjagent_runtime::step::Effect::RecordGraphFault { kind, count, .. }
                if kind == "params" && *count == 3
        )
    }));
    Ok(())
}

#[test]
fn recover_params_renders_valid_example() -> TestResult<()> {
    let state = open("parameter fault")?;
    let result = param_fault(state);
    let joined = result
        .effects
        .iter()
        .filter_map(|effect| match effect {
            lkjagent_runtime::step::Effect::RecordEvent { content, .. } => Some(content.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert!(joined.contains("action params refused"));
    assert!(joined.contains("tool=graph.state"));
    assert!(joined.contains("expected=no parameters"));
    assert!(joined.contains("<tool>graph.state</tool>\n</act>"));
    assert!(joined.contains("fs.list/workspace.summary"));
    Ok(())
}

#[test]
fn third_repeat_action_routes_to_graph_recovery() -> TestResult<()> {
    let mut state = open("repeat loop")?;
    for _ in 0..3 {
        state = pending_read(state).state;
        state = step(state, StepInput::ToolOutput(repeat_notice())).state;
    }

    assert!(matches!(state.task, TaskState::Open { .. }));
    assert_eq!(
        state.graph.as_ref().map(|graph| graph.active_node.0),
        Some("recover-repeat")
    );
    Ok(())
}

#[test]
fn third_tool_error_routes_without_owner_wait() -> TestResult<()> {
    let mut state = open("tool loop")?;
    for _ in 0..3 {
        state = pending_read(state).state;
        state = step(state, StepInput::ToolOutput(error_output("missing file"))).state;
    }
    assert!(matches!(state.task, TaskState::Open { .. }));
    assert_eq!(
        state.graph.as_ref().map(|graph| graph.active_node.0),
        Some("recover-tool")
    );
    assert!(matches!(
        state
            .graph
            .as_ref()
            .map(|graph| graph.recovery.ladder_position),
        Some(3)
    ));
    Ok(())
}

#[test]
fn daemon_routes_after_three_endpoint_parse_faults() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "recover parse loop", "owner-send", "101")?;
    let workspace = temp_workspace("fault-wait-daemon")?;
    let server = serve_responses(vec![
        completion("not an act"),
        completion("still not an act"),
        completion("again not an act"),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    server.join()?;

    let active = lkjagent_store::graph::active_case(&conn)?.ok_or("missing graph case")?;
    assert_eq!(active.phase, "recovery");
    assert_eq!(active.active_node, "recover-parse");
    Ok(())
}

fn open(content: &str) -> TestResult<lkjagent_runtime::task::RuntimeState> {
    Ok(step(
        runtime_state()?,
        StepInput::Owner {
            content: content.to_string(),
            tokens: 3,
            graph: Some(Box::new(lkjagent_graph::initial_state(content, Some(1)))),
            turn_budget: 64,
        },
    )
    .state)
}

fn parse_fault(state: lkjagent_runtime::task::RuntimeState) -> lkjagent_runtime::step::StepResult {
    step(
        state,
        StepInput::Completion {
            content: "no act block".to_string(),
            tokens: 3,
        },
    )
}

fn param_fault(state: lkjagent_runtime::task::RuntimeState) -> lkjagent_runtime::step::StepResult {
    step(
        state,
        StepInput::Completion {
            content: "<act>\n<tool>graph.state</tool>\n<path>.</path>\n</act>".to_string(),
            tokens: 5,
        },
    )
}

fn pending_read(state: lkjagent_runtime::task::RuntimeState) -> lkjagent_runtime::step::StepResult {
    step(
        state,
        StepInput::Completion {
            content: "<act>\n<tool>fs.read</tool>\n<path>missing.md</path>\n</act>".to_string(),
            tokens: 10,
        },
    )
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
