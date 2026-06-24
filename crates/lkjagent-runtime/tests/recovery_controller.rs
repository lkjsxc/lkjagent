mod support;

use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::StopReason;
use support::{ok_output, repeat_notice, runtime_state, TestResult};

#[test]
fn runtime_updates_active_node_after_graph_plan() -> TestResult<()> {
    let state = open("fix parser bug")?;
    let pending = step(
        state,
        StepInput::Completion {
            content: plan_action("fix parser bug"),
            tokens: 20,
        },
    )
    .state;

    let planned = step(
        pending,
        StepInput::ToolOutput(ok_output("graph plan recorded")),
    );

    assert_eq!(
        planned
            .state
            .graph
            .as_ref()
            .map(|graph| graph.active_node.0),
        Some("execute")
    );
    Ok(())
}

#[test]
fn runtime_updates_active_node_after_observation() -> TestResult<()> {
    let state = open("fix parser bug")?;
    let planned = step(
        step(
            state,
            StepInput::Completion {
                content: plan_action("fix parser bug"),
                tokens: 20,
            },
        )
        .state,
        StepInput::ToolOutput(ok_output("graph plan recorded")),
    )
    .state;
    let pending = step(
        planned,
        StepInput::Completion {
            content: "<action>\n<tool>fs.read</tool>\n<path>README.md</path>\n</action>"
                .to_string(),
            tokens: 10,
        },
    )
    .state;

    let observed = step(pending, StepInput::ToolOutput(ok_output("read README.md")));

    assert_eq!(
        observed
            .state
            .graph
            .as_ref()
            .map(|graph| graph.active_node.0),
        Some("verify")
    );
    Ok(())
}

#[test]
fn runtime_routes_param_fault_to_schema_action_class() -> TestResult<()> {
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
    assert_eq!(
        routed
            .state
            .graph
            .as_ref()
            .map(|graph| graph.next_action_class.as_str()),
        Some("exact-schema-example")
    );
    Ok(())
}

#[test]
fn runtime_routes_repeated_action_to_different_action_class() -> TestResult<()> {
    let mut state = open("repeat loop")?;
    for _ in 0..3 {
        state = pending_read(state).state;
        state = step(state, StepInput::ToolOutput(repeat_notice())).state;
    }

    assert_eq!(
        state.graph.as_ref().map(|graph| graph.active_node.0),
        Some("recover-repeat")
    );
    assert_eq!(
        state
            .graph
            .as_ref()
            .map(|graph| graph.next_action_class.as_str()),
        Some("different-action-class")
    );
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

fn plan_action(objective: &str) -> String {
    format!(
        "<action>\n<tool>graph.plan</tool>\n<objective>{objective}</objective>\n<steps>read README.md</steps>\n<checks>README.md is readable</checks>\n<paths>README.md</paths>\n<reason>establish controlled action</reason>\n</action>"
    )
}
