mod support;

use lkjagent_context::model::FrameKind;
use lkjagent_runtime::step::{step, Effect, StepInput};
use lkjagent_runtime::task::{StopReason, TaskState};
use support::{ok_output, oversized_state, prefix, runtime_state, summary_frame, TestResult};

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
                max_turns: 2
            } if summary == "finished"
        )
    }));
    Ok(())
}

#[test]
fn parse_faults_pause_after_three_consecutive_failures() -> TestResult<()> {
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
    assert!(matches!(state.task, TaskState::Paused { .. }));
    assert!(last_effects
        .iter()
        .any(|effect| matches!(effect, Effect::Pause { .. })));
    Ok(())
}

#[test]
fn compaction_rebuilds_window_and_records_event() -> TestResult<()> {
    let compacted = step(
        oversized_state(),
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
    Ok(())
}
