mod support;

use lkjagent_graph::initial_state;
use lkjagent_protocol::render_action;
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{RuntimeState, StopReason, TaskState};
use support::{action, ok_output, runtime_state, TestResult};

#[test]
fn content_artifact_done_refuses_structure_without_readiness() -> TestResult<()> {
    let planned = apply_tool(
        open_content_task()?,
        "graph.plan",
        &[
            ("objective", "Finish long SF story artifact"),
            ("steps", "audit story tree; confirm content readiness"),
            ("checks", "artifact audit passed with readiness"),
            ("paths", "stories/long-sf-story"),
            ("reason", "content artifacts need readiness evidence"),
        ],
        "graph plan recorded",
    );
    let state = apply_tool(
        planned,
        "artifact.audit",
        &[("root", "stories/long-sf-story"), ("kind", "story")],
        "artifact audit passed\nroot=stories/long-sf-story\nfailed=0",
    );

    let result = attempt_done(state);

    assert_eq!(result.stop_reason, Some(StopReason::ToolError));
    assert!(matches!(result.state.task, TaskState::Open { .. }));
    assert!(result
        .effects
        .iter()
        .any(|effect| { format!("{effect:?}").contains("artifact-readiness") }));
    Ok(())
}

#[test]
fn content_readiness_passed_artifact_audit_records_readiness() -> TestResult<()> {
    let planned = apply_tool(
        open_content_task()?,
        "graph.plan",
        &[
            ("objective", "Finish long SF story artifact"),
            ("steps", "audit story tree; confirm content readiness"),
            ("checks", "artifact audit passed with readiness"),
            ("paths", "stories/long-sf-story"),
            ("reason", "content artifacts need readiness evidence"),
        ],
        "graph plan recorded",
    );
    let state = apply_tool(
        planned,
        "artifact.audit",
        &[("root", "stories/long-sf-story"), ("kind", "story")],
        "artifact audit passed\nroot=stories/long-sf-story\ncontent_readiness=passed\nfailed=0",
    );

    assert!(state
        .graph
        .as_ref()
        .is_some_and(|graph| graph.evidence.has("artifact-readiness")));
    Ok(())
}

#[test]
fn explicit_artifact_readiness_evidence_does_not_bypass_audit() -> TestResult<()> {
    let planned = apply_tool(
        open_content_task()?,
        "graph.plan",
        &[
            ("objective", "Finish long SF story artifact"),
            ("steps", "audit story tree; confirm content readiness"),
            ("checks", "artifact audit passed with readiness"),
            ("paths", "stories/long-sf-story"),
            ("reason", "content artifacts need readiness evidence"),
        ],
        "graph plan recorded",
    );
    let structured = apply_tool(
        planned,
        "artifact.audit",
        &[("root", "stories/long-sf-story"), ("kind", "story")],
        "artifact audit passed\nroot=stories/long-sf-story\nfailed=0",
    );
    let claimed = apply_tool(
        structured,
        "graph.evidence",
        &[
            ("kind", "artifact-readiness"),
            ("summary", "claimed content readiness"),
            ("path", "stories/long-sf-story"),
        ],
        "graph evidence recorded\nkind=artifact-readiness\npath=stories/long-sf-story",
    );

    assert!(claimed.graph.as_ref().is_some_and(|graph| {
        graph.evidence.has("document-structure") && !graph.evidence.has("artifact-readiness")
    }));
    let result = attempt_done(claimed);

    assert_eq!(result.stop_reason, Some(StopReason::ToolError));
    assert!(matches!(result.state.task, TaskState::Open { .. }));
    assert!(result
        .effects
        .iter()
        .any(|effect| format!("{effect:?}").contains("artifact-readiness")));
    Ok(())
}

fn open_content_task() -> TestResult<RuntimeState> {
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
