mod support;

use lkjagent_context::model::FrameKind;
use lkjagent_runtime::step::{step, Effect, StepInput};
use lkjagent_runtime::task::{StopReason, TaskState};
use support::{
    error_output, ok_output, oversized_state, prefix, repeat_notice, runtime_state, summary_frame,
    TestResult,
};

#[test]
fn scripted_task_reaches_done_and_distillation_prompt() -> TestResult<()> {
    let owner = step(
        runtime_state()?,
        StepInput::Owner {
            content: "ship the task".to_string(),
            tokens: 4,
        },
    );
    assert!(matches!(owner.state.task, TaskState::Open { .. }));
    assert!(owner
        .effects
        .iter()
        .any(|effect| matches!(effect, Effect::RecordEvent { .. })));

    let completion = step(
        owner.state,
        StepInput::Completion {
            content: "<act>\n<tool>agent.done</tool>\n<summary>finished</summary>\n</act>"
                .to_string(),
            tokens: 12,
        },
    );
    assert_eq!(completion.stop_reason, Some(StopReason::Acted));
    assert!(completion
        .effects
        .iter()
        .any(|effect| matches!(effect, Effect::ExecuteTool { .. })));

    let done = step(completion.state, StepInput::ToolOutput(ok_output("done")));
    assert_eq!(done.stop_reason, Some(StopReason::Done));
    assert!(matches!(
        done.state.task,
        TaskState::Closed { ref summary } if summary == "finished"
    ));
    assert!(done.effects.iter().any(|effect| {
        matches!(
            effect,
            Effect::DistillTask {
                summary,
                prompt,
                max_turns: 2
            } if summary == "finished" && prompt.contains("distill closed task")
        )
    }));
    assert!(done
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.content.contains("max_turns=2")));
    Ok(())
}

#[test]
fn parse_faults_keep_task_open_and_recover_after_three_failures() -> TestResult<()> {
    let mut state = step(
        runtime_state()?,
        StepInput::Owner {
            content: "break parser".to_string(),
            tokens: 3,
        },
    )
    .state;
    let mut last_effects = Vec::new();
    for _ in 0..3 {
        let result = step(
            state,
            StepInput::Completion {
                content: "no act block".to_string(),
                tokens: 3,
            },
        );
        state = result.state;
        last_effects = result.effects;
    }
    assert!(matches!(state.task, TaskState::Open { .. }));
    assert!(last_effects
        .iter()
        .any(|effect| matches!(effect, Effect::RecordEvent { content, .. } if content.contains("shell.run heredoc/script"))));
    let recovered = step(
        state,
        StepInput::Completion {
            content: "<act>\n<tool>agent.done</tool>\n<summary>recovered</summary>\n</act>"
                .to_string(),
            tokens: 12,
        },
    );
    assert_eq!(recovered.stop_reason, Some(StopReason::Acted));
    assert_eq!(recovered.state.parse_faults, 0);
    Ok(())
}

#[test]
fn tool_and_repeat_errors_add_recovery_notices_without_pausing() -> TestResult<()> {
    let mut state = step(
        runtime_state()?,
        StepInput::Owner {
            content: "recover tool use".to_string(),
            tokens: 3,
        },
    )
    .state;
    state = step(
        state,
        StepInput::Completion {
            content: "<act>\n<tool>fs.read</tool>\n<path>missing.md</path>\n</act>".to_string(),
            tokens: 10,
        },
    )
    .state;

    let tool_error = step(state, StepInput::ToolOutput(error_output("missing file")));
    assert_eq!(tool_error.stop_reason, Some(StopReason::ToolError));
    assert!(matches!(tool_error.state.task, TaskState::Open { .. }));
    assert!(tool_error
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.content.contains("tool error recorded")));

    let pending_repeat = step(
        tool_error.state,
        StepInput::Completion {
            content: "<act>\n<tool>fs.read</tool>\n<path>missing.md</path>\n</act>".to_string(),
            tokens: 10,
        },
    );
    let repeat = step(pending_repeat.state, StepInput::ToolOutput(repeat_notice()));
    assert_eq!(repeat.stop_reason, Some(StopReason::RepeatAction));
    assert!(matches!(repeat.state.task, TaskState::Open { .. }));
    assert!(repeat
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.content.contains("repeated action was refused")));
    Ok(())
}

#[test]
fn compaction_rebuilds_window_and_records_event() -> TestResult<()> {
    let mut state = oversized_state();
    state.task = TaskState::Open { turns_remaining: 7 };
    let compacted = step(
        state,
        StepInput::Compact {
            prefix: prefix()?,
            summary: summary_frame(),
            memory_ids: vec![1, 2],
        },
    );
    assert_eq!(compacted.stop_reason, Some(StopReason::Compaction));
    assert!(matches!(
        compacted.state.context.log.first().map(|frame| &frame.kind),
        Some(FrameKind::Notice(_))
    ));
    assert!(compacted.effects.iter().any(|effect| {
        matches!(
            effect,
            Effect::CompactionRecorded {
                memory_ids,
                ..
            } if memory_ids == &vec![1, 2]
        )
    }));
    assert!(compacted.effects.iter().any(|effect| {
        matches!(
            effect,
            Effect::DistillCompaction {
                max_turns: 4,
                task_summary_required: true,
                ..
            }
        )
    }));
    assert!(compacted
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.content.contains("task_summary_required=true")));
    Ok(())
}
