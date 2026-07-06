use std::collections::BTreeSet;

use crate::model::{CheckResult, StepKind};
use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolSetView};
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
        StepKind::Explore => parse_action(raw, view),
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
        OutputEnvelope::Action => parse_action(raw, &decision.tool_view),
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

fn parse_action(raw: &str, view: &ToolSetView) -> Result<ParsedOutput, ParseFault> {
    let body = block(raw, "tool_call")?;
    let params = parse_params(&body)?;
    if params.first().map(|(name, _)| name.as_str()) != Some("tool_name") {
        return Err(ParseFault::BadParams);
    }
    let tool = one(&params, "tool_name")?.to_string();
    validate_params(view, &tool, &params)?;
    Ok(ParsedOutput::Action(Action { tool, params }))
}

fn parse_params(body: &str) -> Result<Vec<(String, String)>, ParseFault> {
    let mut params = Vec::new();
    let mut rest = body.trim();
    while !rest.is_empty() {
        if !rest.starts_with('<') {
            return Err(ParseFault::BadParams);
        }
        let Some(end_name) = rest.find('>') else {
            return Err(ParseFault::BadParams);
        };
        let name = &rest[1..end_name];
        if name.is_empty() || name.starts_with('/') || name.chars().any(char::is_whitespace) {
            return Err(ParseFault::BadParams);
        }
        let close = format!("</{name}>");
        let value_start = end_name + 1;
        let Some(close_at) = rest[value_start..].find(&close) else {
            return Err(ParseFault::BadParams);
        };
        let value = rest[value_start..value_start + close_at].trim().to_string();
        if value.is_empty() {
            return Err(ParseFault::BadParams);
        }
        params.push((name.to_string(), value));
        rest = rest[value_start + close_at + close.len()..].trim();
    }
    unique(&params)?;
    Ok(params)
}

fn unique(params: &[(String, String)]) -> Result<(), ParseFault> {
    let mut seen = BTreeSet::new();
    for (name, _) in params {
        if !seen.insert(name) {
            return Err(ParseFault::BadParams);
        }
    }
    Ok(())
}

fn one<'a>(params: &'a [(String, String)], name: &str) -> Result<&'a str, ParseFault> {
    params
        .iter()
        .find(|(param, _)| param == name)
        .map(|(_, value)| value.as_str())
        .ok_or(ParseFault::BadParams)
}

fn validate_params(
    view: &ToolSetView,
    tool: &str,
    params: &[(String, String)],
) -> Result<(), ParseFault> {
    let entry = view.entry(tool).ok_or(ParseFault::UnknownTool)?;
    for (name, _) in params {
        if name != "tool_name" && !entry.accepts_param(name) {
            return Err(ParseFault::BadParams);
        }
    }
    for required in &entry.required_params {
        one(params, required)?;
    }
    Ok(())
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
        passed,
        measured: lines.collect::<Vec<_>>().join(" "),
    })
}
