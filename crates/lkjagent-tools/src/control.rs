use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::structure::verify_recursive_tree;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlState {
    pub work_open: bool,
    pub question_outstanding: bool,
    pub guard: CompletionGuard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionGuard {
    None,
    RecursiveStructure,
}

impl CompletionGuard {
    pub fn as_state_value(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RecursiveStructure => "recursive-structure",
        }
    }

    pub fn from_state_value(value: &str) -> Self {
        match value {
            "recursive-structure" => Self::RecursiveStructure,
            _ => Self::None,
        }
    }
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            work_open: true,
            question_outstanding: false,
            guard: CompletionGuard::None,
        }
    }
}

impl ControlState {
    pub fn start_task(&mut self, content: &str) {
        self.work_open = true;
        self.question_outstanding = false;
        self.guard = classify(content);
    }

    pub fn resume_task(&mut self) {
        self.work_open = true;
        self.question_outstanding = false;
    }

    pub fn set_guard(&mut self, guard: CompletionGuard) {
        self.guard = guard;
    }
}

pub fn done(state: &mut ControlState, workspace: &Path, summary: &str) -> ToolResult<String> {
    if !state.work_open {
        return Err(ToolError::invalid("no open task or cycle"));
    }
    if summary.trim().is_empty() {
        return Err(ToolError::invalid("summary must not be empty"));
    }
    if matches!(state.guard, CompletionGuard::RecursiveStructure) {
        verify_recursive_tree(workspace)?;
    }
    state.work_open = false;
    state.question_outstanding = false;
    state.guard = CompletionGuard::None;
    Ok(format!("done\nsummary={summary}"))
}

pub fn ask(state: &mut ControlState, question: &str) -> ToolResult<String> {
    if state.question_outstanding {
        return Err(ToolError::invalid("a question is already outstanding"));
    }
    if question.trim().is_empty() {
        return Err(ToolError::invalid("question must not be empty"));
    }
    state.question_outstanding = true;
    Ok(format!("waiting\nquestion={question}"))
}

fn classify(content: &str) -> CompletionGuard {
    let lower = content.to_ascii_lowercase();
    let recursive = lower.contains("recursive") || content.contains("再帰");
    let structure = lower.contains("structure")
        || lower.contains("structured")
        || lower.contains("organize")
        || content.contains("構造");
    if recursive && structure {
        CompletionGuard::RecursiveStructure
    } else {
        CompletionGuard::None
    }
}
