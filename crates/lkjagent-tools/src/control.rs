use std::path::Path;

mod classify;
mod guard;

use crate::count_guard::verify_count;
use crate::error::{ToolError, ToolResult};
use crate::structure::verify_recursive_tree;
use crate::structure_network::verify_knowledge_network;
pub use guard::CompletionGuard;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlState {
    pub work_open: bool,
    pub question_outstanding: bool,
    pub guard: CompletionGuard,
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
        self.guard = classify::classify(content);
    }

    pub fn resume_task(&mut self) {
        self.work_open = true;
        self.question_outstanding = false;
    }

    pub fn resume_task_with(&mut self, content: &str) {
        self.resume_task();
        self.guard = classify::merge_guard(self.guard, classify::classify(content));
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
    match state.guard {
        CompletionGuard::None => {}
        CompletionGuard::FileCount { .. } | CompletionGuard::MarkdownCount { .. } => {}
        CompletionGuard::RecursiveStructure | CompletionGuard::RecursiveStructureCount { .. } => {
            verify_recursive_tree(workspace)?
        }
        CompletionGuard::RecursiveKnowledge | CompletionGuard::RecursiveKnowledgeCount { .. } => {
            verify_knowledge_network(workspace)?
        }
    }
    if let Some(count) = state.guard.count_guard() {
        verify_count(workspace, count)?;
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
