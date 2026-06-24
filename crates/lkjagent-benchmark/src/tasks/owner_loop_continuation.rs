use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const CONTINUE_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "turn budget checkpoint reached; continuing autonomously\ncheckpoint_event=TurnBudgetCheckpoint\ncontinuation_decision=continue-owner-execution\nmissing_evidence=artifact-readiness\nexact_next_action=<action><tool>artifact.next</tool></action>\ndaemon_state=working\n",
}];

const CONTINUE_BAD_WAIT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "turn budget exhausted\nTurn budget exhausted. Send guidance to continue.\ndaemon_state=waiting\n",
}];

const CONTINUE_BAD_QUESTION: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "checkpoint reached\nShould I continue?\nDo you want me to proceed?\n",
}];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-turn-budget-continuation-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "turn-budget", "continuation"],
    prompt: "Handle turn-budget checkpoints without asking for generic continuation.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "turn-budget-continuation-checkpoint",
        files: CONTINUE_GOOD,
    }],
    bad: &[
        Fixture {
            name: "turn-budget-send-guidance-wait",
            files: CONTINUE_BAD_WAIT,
        },
        Fixture {
            name: "turn-budget-asks-should-continue",
            files: CONTINUE_BAD_QUESTION,
        },
    ],
    judge: JudgeKind::TurnBudgetContinuation,
    seed: 8121,
    points: 1,
    timeout_seconds: 120,
};
