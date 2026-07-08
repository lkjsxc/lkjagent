use serde_json::Value;

use crate::model::{CheckResult, StepKind};
use crate::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use crate::runtime_tool_call::{parse_tool_call, ToolCallError};
use crate::runtime_tool_catalog::explore_tool_view;

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
    BadPlanLine(String),
    Action(ToolCallError),
}

pub fn parse_expected(kind: StepKind, raw: &str) -> Result<ParsedOutput, ParseFault> {
    parse_expected_with_view(kind, raw, &explore_tool_view())
}

pub fn parse_expected_with_view(
    kind: StepKind,
    raw: &str,
    view: &ToolSetView,
) -> Result<ParsedOutput, ParseFault> {
    match kind {
        StepKind::Write | StepKind::Revise => block(raw, "content").map(ParsedOutput::Content),
        StepKind::Plan => {
            crate::parse_plan::parse_plan(&block(raw, "plan")?).map(ParsedOutput::Plan)
        }
        StepKind::Explore => parse_action(raw, &synthetic_action_decision(view)),
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

fn synthetic_action_decision(view: &ToolSetView) -> RuntimeDecision {
    let mut decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        view.clone(),
        OutputEnvelope::Action,
    );
    decision.context_frame_fingerprint = "ctx-1".to_string();
    decision
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
