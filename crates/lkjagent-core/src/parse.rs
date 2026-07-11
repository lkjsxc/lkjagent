use serde_json::Value;

use crate::model::{CheckResult, StepKind};
use crate::runtime_decision::{OutputEnvelope, RuntimeDecision};
use crate::runtime_tool_call::{parse_tool_call, ToolCallError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedOutput {
    Content(String),
    Plan(Vec<PlanLine>),
    Action(Action),
    Message(String),
    Verdict(CheckResult),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanLine {
    Write {
        path: String,
        title: String,
        words: usize,
    },
    Explore {
        goal: String,
        budget: u32,
    },
    Respond {
        summary: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub tool: String,
    pub params: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseFault {
    WrongBlock,
    Unclosed,
    Empty,
    UnknownTool,
    BadParams,
    DecisionRequired,
    BadPlanLine(String),
    Action(ToolCallError),
}

pub fn parse_fault_diagnosis(fault: &ParseFault) -> String {
    match fault {
        ParseFault::WrongBlock => "Use only the expected envelope tag for this decision.".into(),
        ParseFault::Unclosed => "Close the expected envelope tag before sending.".into(),
        ParseFault::Empty => "Put non-empty content inside the expected envelope.".into(),
        ParseFault::UnknownTool => "Choose one tool from the selected tool view.".into(),
        ParseFault::BadParams => "Use selected tool argument names and primitive values.".into(),
        ParseFault::DecisionRequired => "Wait for the persisted runtime decision.".into(),
        ParseFault::BadPlanLine(line) => format!("Revise plan line `{line}` to the grammar."),
        ParseFault::Action(error) => format!(
            "Action parse fault: {error:?}. Repair: {}",
            action_hint(error)
        ),
    }
}

fn action_hint(error: &ToolCallError) -> &'static str {
    match error {
        ToolCallError::NoActionFound => "Return one lkjagent_action envelope.",
        ToolCallError::MultipleActionsFound => "Return exactly one lkjagent_action envelope.",
        ToolCallError::Attribute(_) => "Remove attributes; use child tags only.",
        ToolCallError::DuplicateTag(_) => "Keep one value for each scalar or argument name.",
        ToolCallError::DecisionMismatch => "Echo the current decision_id exactly.",
        ToolCallError::ContextMismatch => "Echo the current context_fingerprint exactly.",
        ToolCallError::ToolUnknown => "Choose one tool from the selected tool view.",
        ToolCallError::ArgsSchemaViolation(_) => "Match the selected tool argument schema.",
        _ => "Use the documented action envelope with balanced child tags only.",
    }
}

pub fn parse_expected(kind: StepKind, raw: &str) -> Result<ParsedOutput, ParseFault> {
    match kind {
        StepKind::Write | StepKind::Revise => block(raw, "content").map(ParsedOutput::Content),
        StepKind::Plan => {
            crate::parse_plan::parse_plan(&block(raw, "plan")?).map(ParsedOutput::Plan)
        }
        StepKind::Explore => Err(ParseFault::DecisionRequired),
        StepKind::Respond | StepKind::Ask => block(raw, "message").map(ParsedOutput::Message),
        StepKind::Verify => parse_verdict(&block(raw, "verdict")?).map(ParsedOutput::Verdict),
    }
}

pub fn parse_expected_for_decision(
    decision: &RuntimeDecision,
    raw: &str,
) -> Result<ParsedOutput, ParseFault> {
    match decision.expected_envelope {
        OutputEnvelope::Content => block(raw, "content").map(ParsedOutput::Content),
        OutputEnvelope::Plan => {
            crate::parse_plan::parse_plan(&block(raw, "plan")?).map(ParsedOutput::Plan)
        }
        OutputEnvelope::Action => parse_action(raw, decision),
        OutputEnvelope::Message => block(raw, "message").map(ParsedOutput::Message),
        OutputEnvelope::Verdict => {
            parse_verdict(&block(raw, "verdict")?).map(ParsedOutput::Verdict)
        }
        OutputEnvelope::None => Err(ParseFault::WrongBlock),
    }
}

pub fn block(raw: &str, tag: &str) -> Result<String, ParseFault> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let trimmed = raw.trim();
    if !trimmed.starts_with(&open) {
        return Err(ParseFault::WrongBlock);
    }
    let Some(end) = trimmed.find(&close) else {
        return Err(ParseFault::Unclosed);
    };
    if !trimmed[end + close.len()..].trim().is_empty() {
        return Err(ParseFault::WrongBlock);
    }
    let body = trimmed[open.len()..end].trim().to_string();
    if body.is_empty() {
        Err(ParseFault::Empty)
    } else {
        Ok(body)
    }
}

fn parse_action(raw: &str, decision: &RuntimeDecision) -> Result<ParsedOutput, ParseFault> {
    let parsed = parse_tool_call(raw, decision).map_err(map_action_fault)?;
    Ok(ParsedOutput::Action(Action {
        tool: parsed.tool_name,
        params: parsed
            .args
            .into_iter()
            .map(|(name, value)| (name, param_value(&value)))
            .collect(),
    }))
}

fn map_action_fault(error: ToolCallError) -> ParseFault {
    match error {
        ToolCallError::ToolUnknown => ParseFault::UnknownTool,
        other => ParseFault::Action(other),
    }
}

fn param_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

fn parse_verdict(body: &str) -> Result<CheckResult, ParseFault> {
    let mut lines = body.lines();
    let Some(first) = lines.next().map(str::trim) else {
        return Err(ParseFault::Empty);
    };
    let passed = match first {
        "pass" => true,
        "fail" => false,
        _ => return Err(ParseFault::BadParams),
    };
    Ok(CheckResult {
        name: "judged".to_string(),
        params: None,
        decision_id: None,
        evidence_fingerprint: None,
        artifact_refs: Vec::new(),
        passed,
        measured: lines.collect::<Vec<_>>().join(" "),
    })
}
