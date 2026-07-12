use crate::runtime_decision::{OutputEnvelope, RuntimeDecision};
use crate::runtime_tool_call::{parse_model_value, ModelValue, ToolCallError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedOutput {
    Action(Action),
    Message(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub tool: String,
    pub params: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseFault {
    WrongBlock,
    UnknownTool,
    Action(ToolCallError),
}

pub fn parse_fault_diagnosis(fault: &ParseFault) -> String {
    match fault {
        ParseFault::WrongBlock => "Use only the expected envelope tag for this decision.".into(),
        ParseFault::UnknownTool => "Choose one tool from the selected tool view.".into(),
        ParseFault::Action(error) => format!(
            "Action parse fault: {error:?}. Repair: {}",
            action_hint(error)
        ),
    }
}

fn action_hint(error: &ToolCallError) -> &'static str {
    match error {
        ToolCallError::MissingRoot | ToolCallError::MultipleRoots => {
            "Return exactly one complete tool_call or final envelope."
        }
        ToolCallError::Attribute => "Remove attributes; use child tags only.",
        ToolCallError::DuplicateTag => "Keep one value for each field name.",
        ToolCallError::WrongGrammarPhase => "Use the envelope selected by the harness.",
        ToolCallError::HiddenTool => "Choose one visible tool from this decision.",
        ToolCallError::FieldOrder | ToolCallError::MissingField | ToolCallError::UnknownTag => {
            "Match the ordered descriptor fields exactly."
        }
        ToolCallError::ValueClass | ToolCallError::UnsafePath | ToolCallError::Bounds => {
            "Match the descriptor value class and bounds."
        }
        _ => "Use balanced, attribute-free child tags only.",
    }
}

pub fn parse_expected_for_decision(
    decision: &RuntimeDecision,
    raw: &str,
) -> Result<ParsedOutput, ParseFault> {
    if !matches!(
        decision.expected_envelope,
        OutputEnvelope::Action | OutputEnvelope::Message
    ) {
        return Err(ParseFault::WrongBlock);
    }
    match parse_model_value(raw, decision).map_err(map_action_fault)? {
        ModelValue::ToolCall(parsed) => Ok(ParsedOutput::Action(Action {
            tool: parsed.tool_name,
            params: parsed.args,
        })),
        ModelValue::Final(message) => Ok(ParsedOutput::Message(message)),
    }
}

fn map_action_fault(error: ToolCallError) -> ParseFault {
    match error {
        ToolCallError::HiddenTool => ParseFault::UnknownTool,
        other => ParseFault::Action(other),
    }
}
