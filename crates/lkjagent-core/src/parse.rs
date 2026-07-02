use crate::model::{CheckResult, StepKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedOutput {
    Content(String),
    Plan(Vec<PlanLine>),
    Action(Action),
    Finish(String),
    Message(String),
    Question(String),
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
    match kind {
        StepKind::Write | StepKind::Revise => block(raw, "content").map(ParsedOutput::Content),
        StepKind::Plan => {
            crate::parse_plan::parse_plan(&block(raw, "plan")?).map(ParsedOutput::Plan)
        }
        StepKind::Explore => parse_explore(raw),
        StepKind::Respond | StepKind::Ask => block(raw, "message").map(ParsedOutput::Message),
        StepKind::Verify => parse_verdict(&block(raw, "verdict")?).map(ParsedOutput::Verdict),
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
    let body = trimmed[open.len()..end].trim().to_string();
    if body.is_empty() {
        Err(ParseFault::Empty)
    } else {
        Ok(body)
    }
}

fn parse_explore(raw: &str) -> Result<ParsedOutput, ParseFault> {
    if raw.trim().starts_with("<finish>") {
        return block(raw, "finish").map(ParsedOutput::Finish);
    }
    if raw.trim().starts_with("<ask>") {
        return block(raw, "ask").map(ParsedOutput::Question);
    }
    let body = block(raw, "action")?;
    let tool = tag_value(&body, "tool").ok_or(ParseFault::BadParams)?;
    if !legal_tool(&tool) {
        return Err(ParseFault::UnknownTool);
    }
    let params = parse_params(&body);
    if params.iter().filter(|(name, _)| name == "tool").count() != 1 {
        return Err(ParseFault::BadParams);
    }
    Ok(ParsedOutput::Action(Action { tool, params }))
}

fn parse_params(body: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find('<') {
        let after = &rest[start + 1..];
        let Some(end_name) = after.find('>') else {
            break;
        };
        let name = &after[..end_name];
        if name.starts_with('/') || name.is_empty() {
            rest = &after[end_name + 1..];
            continue;
        }
        let close = format!("</{name}>");
        let value_start = start + 1 + end_name + 1;
        let Some(close_at) = rest[value_start..].find(&close) else {
            break;
        };
        let value = rest[value_start..value_start + close_at].trim().to_string();
        params.push((name.to_string(), value));
        rest = &rest[value_start + close_at + close.len()..];
    }
    params
}

fn tag_value(body: &str, tag: &str) -> Option<String> {
    parse_params(body)
        .into_iter()
        .find(|(name, _)| name == tag)
        .map(|(_, value)| value)
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
    let measured = lines.collect::<Vec<_>>().join(" ");
    Ok(CheckResult {
        name: "judged".to_string(),
        passed,
        measured,
    })
}

fn legal_tool(tool: &str) -> bool {
    const TOOLS: &[&str] = &[
        "fs.read",
        "fs.list",
        "fs.tree",
        "fs.search",
        "fs.write",
        "shell.run",
        "memory.find",
        "memory.save",
        "plan.note",
        "finish",
    ];
    TOOLS.contains(&tool)
}
