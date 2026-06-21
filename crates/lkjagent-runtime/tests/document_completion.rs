mod support;

use lkjagent_graph::initial_state;
use lkjagent_protocol::render_action;
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{RuntimeState, StopReason, TaskState};
use support::{action, ok_output, runtime_state, TestResult};

#[test]
fn content_artifact_scaffold_only_does_not_satisfy_structure() -> TestResult<()> {
    let state = apply_tool(
        open_doc_task()?,
        "doc.scaffold",
        &[
            ("root", "stories/long-sf-story"),
            ("title", "Long SF Story"),
            ("kind", "content-artifact"),
        ],
        "document scaffold created\nroot=stories/long-sf-story",
    );

    assert!(state
        .graph
        .as_ref()
        .is_some_and(|graph| graph.evidence.has("observation")));
    assert!(state.graph.as_ref().is_some_and(|graph| {
        !graph.evidence.has("document-structure") && !graph.completion.ready
    }));
    Ok(())
}

#[test]
fn failed_document_audit_does_not_satisfy_structure() -> TestResult<()> {
    let state = apply_tool(
        open_doc_task()?,
        "doc.audit",
        &[("root", "stories/long-sf-story")],
        "document audit failed\nroot=stories/long-sf-story\nfailed=2",
    );

    assert!(state
        .graph
        .as_ref()
        .is_some_and(|graph| graph.evidence.has("observation")));
    assert!(state
        .graph
        .as_ref()
        .is_some_and(|graph| !graph.evidence.has("document-structure")));
    Ok(())
}

#[test]
fn passed_document_audit_satisfies_structure() -> TestResult<()> {
    let state = apply_tool(
        open_doc_task()?,
        "artifact.audit",
        &[("root", "stories/long-sf-story"), ("kind", "story")],
        "artifact audit passed\nroot=stories/long-sf-story\nfailed=0\nreadiness=content-bearing",
    );

    assert!(state
        .graph
        .as_ref()
        .is_some_and(|graph| graph.evidence.has("document-structure")));
    Ok(())
}

#[test]
fn passed_artifact_audit_satisfies_structure() -> TestResult<()> {
    let state = apply_tool(
        open_doc_task()?,
        "artifact.audit",
        &[("root", "stories/long-sf-story"), ("kind", "story")],
        "artifact audit passed\nroot=stories/long-sf-story\nfailed=0",
    );

    assert!(state
        .graph
        .as_ref()
        .is_some_and(|graph| graph.evidence.has("document-structure")));
    Ok(())
}

#[test]
fn agent_done_refuses_scaffold_only_artifact() -> TestResult<()> {
    let state = apply_tool(
        open_doc_task()?,
        "doc.scaffold",
        &[
            ("root", "stories/long-sf-story"),
            ("title", "Long SF Story"),
            ("kind", "content-artifact"),
        ],
        "document scaffold created\nroot=stories/long-sf-story",
    );

    let result = attempt_done(state);

    assert_eq!(result.stop_reason, Some(StopReason::ToolError));
    assert!(matches!(result.state.task, TaskState::Open { .. }));
    assert!(result
        .state
        .graph
        .as_ref()
        .is_some_and(|graph| graph.status != lkjagent_graph::CaseStatus::Closed));
    Ok(())
}

#[test]
fn agent_done_refuses_missing_document_audit() -> TestResult<()> {
    let state = apply_tool(
        open_doc_task()?,
        "doc.audit",
        &[("root", "stories/long-sf-story")],
        "document audit failed\nroot=stories/long-sf-story\nfailed=2",
    );

    let result = attempt_done(state);

    assert_eq!(result.stop_reason, Some(StopReason::ToolError));
    assert!(matches!(result.state.task, TaskState::Open { .. }));
    assert!(result
        .effects
        .iter()
        .any(|effect| format!("{effect:?}")
            .contains("agent.done refused by runtime completion gate")));
    Ok(())
}

#[test]
fn agent_done_allows_complete_after_artifact_audit() -> TestResult<()> {
    let planned = apply_tool(
        open_doc_task()?,
        "graph.plan",
        &[
            ("objective", "Finish long SF story artifact"),
            ("steps", "scaffold story tree; audit story tree"),
            ("checks", "artifact audit passed with readiness"),
            ("paths", "stories/long-sf-story"),
            ("reason", "completion requires plan and readiness evidence"),
        ],
        "graph plan recorded",
    );
    let state = apply_tool(
        planned,
        "artifact.audit",
        &[("root", "stories/long-sf-story"), ("kind", "story")],
        "artifact audit passed\nroot=stories/long-sf-story\nfailed=0\nreadiness=content-bearing",
    );

    let result = attempt_done(state);

    assert_eq!(result.stop_reason, Some(StopReason::Done));
    assert!(matches!(result.state.task, TaskState::Closed { .. }));
    assert!(result
        .state
        .graph
        .as_ref()
        .is_some_and(|graph| graph.status == lkjagent_graph::CaseStatus::Closed));
    Ok(())
}

fn open_doc_task() -> TestResult<RuntimeState> {
    let mut state = runtime_state()?;
    state.task = TaskState::Open { turns_remaining: 8 };
    state.graph = Some(initial_state("Create long SF story.", Some(1)));
    Ok(state)
}

fn attempt_done(state: RuntimeState) -> lkjagent_runtime::step::StepResult {
    let action = action("agent.done", &[("summary", "finished")]);
    let acted = step(
        state,
        StepInput::Completion {
            content: render_action(&action),
            tokens: 12,
        },
    );
    step(acted.state, StepInput::ToolOutput(ok_output("done")))
}

fn apply_tool(
    state: RuntimeState,
    tool: &str,
    params: &[(&str, &str)],
    output: &str,
) -> RuntimeState {
    let action = action(tool, params);
    let acted = step(
        state,
        StepInput::Completion {
            content: render_action(&action),
            tokens: 12,
        },
    );
    step(acted.state, StepInput::ToolOutput(ok_output(output))).state
}
