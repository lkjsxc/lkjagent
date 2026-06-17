use std::path::PathBuf;

use lkjagent_context::budget::LOG_OBSERVATION;

use crate::control::ControlState;
use crate::observe::OutputKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRuntime {
    pub workspace: PathBuf,
    pub skill_library: PathBuf,
    pub now: String,
    pub observation_tokens: usize,
    pub shell_timeout_max: u64,
}

impl ToolRuntime {
    pub fn new(workspace: PathBuf, skill_library: PathBuf, now: impl Into<String>) -> Self {
        Self {
            workspace,
            skill_library,
            now: now.into(),
            observation_tokens: LOG_OBSERVATION,
            shell_timeout_max: 600,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchState {
    pub last_action_text: Option<String>,
    pub last_frame_ref: Option<usize>,
    pub next_frame_ref: usize,
    pub repeat_count: usize,
    pub reads: Vec<ReadRecord>,
    pub loaded_skills: Vec<LoadedSkillRecord>,
    pub loaded_skill_tokens: usize,
    pub control: ControlState,
}

impl Default for DispatchState {
    fn default() -> Self {
        Self {
            last_action_text: None,
            last_frame_ref: None,
            next_frame_ref: 1,
            repeat_count: 0,
            reads: Vec::new(),
            loaded_skills: Vec::new(),
            loaded_skill_tokens: 0,
            control: ControlState::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadRecord {
    pub path: String,
    pub start: usize,
    pub count: usize,
    pub total_lines: usize,
    pub body: String,
    pub frame_ref: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedSkillRecord {
    pub name: String,
    pub frame_ref: usize,
    pub tokens: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchOutput {
    pub frame_ref: usize,
    pub kind: OutputKind,
    pub content: String,
    pub rendered: String,
}
