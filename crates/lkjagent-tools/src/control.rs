use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::structure::verify_recursive_tree;
use crate::structure_network::verify_knowledge_network;

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
    RecursiveKnowledge,
}

impl CompletionGuard {
    pub fn as_state_value(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RecursiveStructure => "recursive-structure",
            Self::RecursiveKnowledge => "recursive-knowledge",
        }
    }

    pub fn from_state_value(value: &str) -> Self {
        match value {
            "recursive-structure" => Self::RecursiveStructure,
            "recursive-knowledge" => Self::RecursiveKnowledge,
            _ => Self::None,
        }
    }

    pub fn is_recursive(self) -> bool {
        matches!(self, Self::RecursiveStructure | Self::RecursiveKnowledge)
    }

    pub fn is_knowledge(self) -> bool {
        matches!(self, Self::RecursiveKnowledge)
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
    match state.guard {
        CompletionGuard::None => {}
        CompletionGuard::RecursiveStructure => verify_recursive_tree(workspace)?,
        CompletionGuard::RecursiveKnowledge => verify_knowledge_network(workspace)?,
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
    let recursive =
        lower.contains("recursive") || content.contains("再帰") || content.contains("再起");
    let structure = lower.contains("structure")
        || lower.contains("structured")
        || lower.contains("organize")
        || content.contains("構造");
    if knowledge_request(&lower, content)
        && (recursive || structure || creation_request(&lower, content))
    {
        CompletionGuard::RecursiveKnowledge
    } else if recursive && structure {
        CompletionGuard::RecursiveStructure
    } else {
        CompletionGuard::None
    }
}

fn knowledge_request(lower: &str, content: &str) -> bool {
    lower.contains("encyclopedia")
        || lower.contains("knowledge base")
        || lower.contains("knowledge")
        || lower.contains("wiki")
        || content.contains("百科事典")
        || content.contains("知識")
}

fn creation_request(lower: &str, content: &str) -> bool {
    lower.contains("build")
        || lower.contains("create")
        || lower.contains("make")
        || lower.contains("write")
        || lower.contains("generate")
        || lower.contains("docs")
        || lower.contains("documentation")
        || content.contains("作")
        || content.contains("生成")
        || content.contains("構築")
}
