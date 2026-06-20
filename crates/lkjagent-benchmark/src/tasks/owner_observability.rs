use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const STATUS_PROMPT: &str = "\
After a run, status output must show compact context fraction and input,
output, cached input, and total token usage when available.
";

const STATUS_GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "status.txt",
    content: "daemon=running\nqueue_depth=0\nctx=12.43K/24.58K 50.58% pressure=yellow\nin=8.12K out=1.04K cache=6.88K total=9.16K\nprefix=5.38K log=7.05K reserve=2.05K headroom=10.10K\n",
}];

const STATUS_BAD_NO_TOKENS: &[FileSpec] = &[FileSpec {
    path: "status.txt",
    content: "daemon=running\nqueue_depth=0\n",
}];

const STATUS_BAD_NO_PERCENT: &[FileSpec] = &[FileSpec {
    path: "status.txt",
    content: "ctx=12430/24580 pressure=yellow\nin=8120 out=1040 cache=6880 total=9160\n",
}];

const STATUS_GOOD: &[Fixture] = &[Fixture {
    name: "compact-accounting",
    files: STATUS_GOOD_FILES,
}];

const STATUS_BAD: &[Fixture] = &[
    Fixture {
        name: "no-token-ledger",
        files: STATUS_BAD_NO_TOKENS,
    },
    Fixture {
        name: "no-percentage-or-short-count",
        files: STATUS_BAD_NO_PERCENT,
    },
];

pub const STATUS_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-status-accounting-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "status", "tokens"],
    prompt: STATUS_PROMPT,
    follow_up: None,
    starter_files: &[],
    good: STATUS_GOOD,
    bad: STATUS_BAD,
    judge: JudgeKind::StatusAccounting,
    seed: 8201,
    points: 1,
    timeout_seconds: 120,
};

const LOG_PROMPT: &str = "\
Produce the single current GPT-5.5-Pro handoff log with owner objective,
state tracks, token usage, fault ledger, and verification sections.
";

const LOG_GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "data/logs/current-gpt-5.5-pro.md",
    content: "# lkjagent GPT-5.5-Pro Run Log\n\n## Snapshot\n\n- token_usage: in=8.12K out=1.04K cache=6.88K total=9.16K\n\n## Owner Objective\n\nNormalized objective.\n\n## Active State Tracks\n\n| rank | posture | label |\n| --- | --- | --- |\n| 1 | Implementing | docs |\n\n## Fault Ledger\n\n| turn | kind | message |\n| --- | --- | --- |\n\n## Verification\n\n| command | result |\n| --- | --- |\n",
}];

const LOG_BAD_FRAGMENTED: &[FileSpec] = &[
    FileSpec {
        path: "data/logs/snapshot.md",
        content: "# Snapshot\n",
    },
    FileSpec {
        path: "data/logs/faults.md",
        content: "# Faults\n",
    },
];

const LOG_BAD_SHALLOW: &[FileSpec] = &[FileSpec {
    path: "data/logs/current-gpt-5.5-pro.md",
    content: "# lkjagent GPT-5.5-Pro Run Log\n\n## Snapshot\n\nNo state tracks.\n",
}];

const LOG_GOOD: &[Fixture] = &[Fixture {
    name: "single-current-log",
    files: LOG_GOOD_FILES,
}];

const LOG_BAD: &[Fixture] = &[
    Fixture {
        name: "fragmented",
        files: LOG_BAD_FRAGMENTED,
    },
    Fixture {
        name: "shallow",
        files: LOG_BAD_SHALLOW,
    },
];

pub const LOG_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-gpt-log-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "gpt-log", "handoff"],
    prompt: LOG_PROMPT,
    follow_up: None,
    starter_files: &[],
    good: LOG_GOOD,
    bad: LOG_BAD,
    judge: JudgeKind::GptHandoffLog,
    seed: 8202,
    points: 1,
    timeout_seconds: 120,
};
