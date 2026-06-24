mod support;

use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{StopReason, TaskState};
use support::{error_output, repeat_notice, runtime_state, TestResult};

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
    assert!(joined.contains("<tool>graph.state</tool>\n</action>"));
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
            content: "no action envelope".to_string(),
            tokens: 3,
        },
    )
}

fn param_fault(state: lkjagent_runtime::task::RuntimeState) -> lkjagent_runtime::step::StepResult {
    step(
        state,
        StepInput::Completion {
            content: "<action>\n<tool>graph.state</tool>\n<path>.</path>\n</action>".to_string(),
            tokens: 5,
        },
    )
}

fn pending_read(state: lkjagent_runtime::task::RuntimeState) -> lkjagent_runtime::step::StepResult {
    step(
        state,
        StepInput::Completion {
            content: "<action>\n<tool>fs.read</tool>\n<path>missing.md</path>\n</action>"
                .to_string(),
            tokens: 10,
        },
    )
}
