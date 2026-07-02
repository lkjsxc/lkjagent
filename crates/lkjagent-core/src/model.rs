use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub objective: String,
    pub template: TemplateId,
    pub state: TaskState,
    pub brief: String,
    pub budget_used: u32,
    pub budget: u32,
    pub summary: String,
    pub checks: Vec<CheckSpec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateId {
    Generic,
    Question,
    Manuscript,
    DocsTree,
    FileWork,
    Journal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    Open,
    Waiting,
    Blocked,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub id: u64,
    pub task_id: u64,
    pub ordinal: u32,
    pub kind: StepKind,
    pub title: String,
    pub instruction: String,
    pub inputs: String,
    pub output_path: Option<String>,
    pub checks: Vec<CheckSpec>,
    pub state: StepState,
    pub attempts_used: u32,
    pub actions_used: u32,
    pub action_budget: u32,
    pub split_used: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepKind {
    Plan,
    Write,
    Revise,
    Explore,
    Verify,
    Respond,
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepState {
    Pending,
    Active,
    Done,
    Blocked,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attempt {
    pub step_id: u64,
    pub ordinal: u32,
    pub prompt_fingerprint: String,
    pub outcome: AttemptOutcome,
    pub diagnosis: String,
    pub tokens_in: u32,
    pub tokens_out: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttemptOutcome {
    Ok,
    ParseFault,
    CheckFail,
    EffectError,
    EndpointError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskSnapshot {
    pub task: Task,
    pub steps: Vec<Step>,
    pub attempts: Vec<Attempt>,
    pub check_results: Vec<CheckResult>,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckSpec {
    FileExists {
        path: String,
    },
    MinWords {
        path: String,
        n: usize,
    },
    MinWordsTotal {
        glob: String,
        n: usize,
    },
    MaxLines {
        path: String,
        n: usize,
    },
    FileCount {
        glob: String,
        min: usize,
        max: Option<usize>,
    },
    Contains {
        path: String,
        needle: String,
    },
    Absent {
        path: String,
        needle: String,
    },
    ReadmeCoverage {
        root: String,
    },
    LinksResolve {
        root: String,
    },
    Command {
        cmd: String,
    },
    Judged {
        criterion: String,
        path: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub measured: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    Owner,
    StepDone,
    StepBlocked,
    TaskClosed,
    TaskBlocked,
    Question,
    Answer,
    Notice,
}
