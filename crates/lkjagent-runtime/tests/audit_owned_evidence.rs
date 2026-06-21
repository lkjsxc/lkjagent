mod support;

use lkjagent_graph::initial_state;
use lkjagent_protocol::render_action;
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{RuntimeState, StopReason, TaskState};
use support::{action, ok_output, runtime_state, TestResult};

#[test]
fn explicit_document_structure_evidence_does_not_bypass_audit() -> TestResult<()> {
    let planned = apply_tool(
        open_doc_task()?,
        "graph.plan",
        &[
            ("objective", "Finish long SF story artifact"),
            ("steps", "audit story tree"),
            ("checks", "document audit passed"),
            ("paths", "stories/long-sf-story"),
            ("reason", "document structure must be audited"),
        ],
        "graph plan recorded",
    );
    let claimed = apply_tool(
        planned,
        "graph.evidence",
        &[
            ("kind", "document-structure"),
            ("summary", "claimed document structure"),
            ("path", "stories/long-sf-story"),
        ],
        "graph evidence recorded\nkind=document-structure\npath=stories/long-sf-story",
    );

    assert!(claimed
        .graph
        .as_ref()
        .is_some_and(|graph| !graph.evidence.has("document-structure")));
    let result = attempt_done(claimed);

    assert_eq!(result.stop_reason, Some(StopReason::ToolError));
    assert!(matches!(result.state.task, TaskState::Open { .. }));
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
