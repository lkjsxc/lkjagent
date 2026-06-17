use crate::error::{ToolError, ToolResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlState {
    pub work_open: bool,
    pub question_outstanding: bool,
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            work_open: true,
            question_outstanding: false,
        }
    }
}

pub fn done(state: &mut ControlState, summary: &str) -> ToolResult<String> {
    if !state.work_open {
        return Err(ToolError::invalid("no open task or cycle"));
    }
    if summary.trim().is_empty() {
        return Err(ToolError::invalid("summary must not be empty"));
    }
    state.work_open = false;
    state.question_outstanding = false;
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
