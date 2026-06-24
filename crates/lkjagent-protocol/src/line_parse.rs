use crate::error::ParseResult;
use crate::model::{Action, MalformedTagReason, Param, ParseFault, ACTION_CLOSE};
use crate::registry::{find_tool, ToolSpec};
use crate::tag_line::{classify_tag_line, TagLineClass};

pub fn starts_line_action(lines: &[&str], index: usize) -> bool {
    lines
        .iter()
        .skip(index)
        .find(|line| !line.trim().is_empty())
        .is_some_and(|line| line.trim_start().starts_with("tool:"))
}

pub fn parse_line_action(lines: &[&str], mut index: usize) -> ParseResult<(Action, usize)> {
    let mut tool: Option<String> = None;
    let mut params = Vec::new();
    let mut seen = Vec::new();
    while index < lines.len() {
        let line = lines[index].trim_end();
        if line == ACTION_CLOSE {
            let Some(tool_name) = tool else {
                return Err(ParseFault::MissingTool);
            };
            validate_params(&tool_name, &params)?;
            return Ok((Action::new(tool_name, params), index + 1));
        }
        if line.trim().is_empty() {
            index += 1;
            continue;
        }
        let Some((name, value)) = line_pair(line) else {
            return Err(non_pair_fault(tool.as_deref(), &params, line));
        };
        if name == "case" {
            index += 1;
            continue;
        }
        if seen.contains(&name) {
            return Err(ParseFault::DuplicateParam { name });
        }
        seen.push(name.clone());
        if name == "tool" {
            if find_tool(&value).is_none() {
                return Err(ParseFault::UnknownTool { tool: value });
            }
            tool = Some(value);
            index += 1;
            continue;
        }
        if tool.is_none() {
            return Err(ParseFault::MissingTool);
        }
        if value.is_empty() && matches!(name.as_str(), "content" | "files" | "patch") {
            let (block, next) = collect_block(lines, index + 1, &name)?;
            params.push(Param::new(name, block));
            index = next;
        } else {
            params.push(Param::new(name, value));
            index += 1;
        }
    }
    Err(ParseFault::UnclosedActionEnvelope)
}

fn collect_block(lines: &[&str], start: usize, name: &str) -> ParseResult<(String, usize)> {
    let mut value = Vec::new();
    let mut index = start;
    while let Some(line) = lines.get(index) {
        if line.trim_end() == ACTION_CLOSE {
            let text = if name == "files" {
                normalize_files(&value)
            } else {
                value.join("\n")
            };
            return Ok((text, index));
        }
        value.push((*line).to_string());
        index += 1;
    }
    Err(ParseFault::UnclosedTag {
        tag: name.to_string(),
    })
}

fn normalize_files(lines: &[String]) -> String {
    if !lines.iter().any(|line| line.trim() == "-- file --") {
        return lines.join("\n");
    }
    let mut blocks = Vec::new();
    let mut current = Vec::new();
    for line in lines {
        match line.trim() {
            "-- file --" => current.clear(),
            "-- end-file --" => blocks.push(current.join("\n")),
            _ => current.push(line.clone()),
        }
    }
    blocks.join("\n-- lkjagent-next-file --\n")
}

fn line_pair(line: &str) -> Option<(String, String)> {
    let (name, value) = line.split_once(':')?;
    valid_name(name.trim()).then(|| (name.trim().to_string(), value.trim_start().to_string()))
}

fn non_pair_fault(tool: Option<&str>, params: &[Param], line: &str) -> ParseFault {
    match classify_tag_line(line) {
        TagLineClass::AttributeLikeTag {
            tag_name,
            value_hint,
        } => ParseFault::AttributeLikeTag {
            tag_name,
            value_hint,
        },
        TagLineClass::MalformedAngleText => ParseFault::MalformedTag {
            line: line.to_string(),
            reason: MalformedTagReason::BadAngleSyntax,
        },
        _ => bad_params_fault(tool, params, line),
    }
}

fn bad_params_fault(tool: Option<&str>, params: &[Param], line: &str) -> ParseFault {
    let Some(tool) = tool else {
        return ParseFault::MissingTool;
    };
    ParseFault::BadParams {
        tool: tool.to_string(),
        missing: find_tool(tool).map_or_else(Vec::new, |spec| missing_required(spec, params)),
        unknown: vec![line.to_string()],
    }
}

fn validate_params(tool_name: &str, params: &[Param]) -> ParseResult<()> {
    let Some(spec) = find_tool(tool_name) else {
        return Err(ParseFault::UnknownTool {
            tool: tool_name.to_string(),
        });
    };
    let missing = missing_required(spec, params);
    let unknown = unknown_params(spec, params);
    if missing.is_empty() && unknown.is_empty() {
        Ok(())
    } else {
        Err(ParseFault::BadParams {
            tool: tool_name.to_string(),
            missing,
            unknown,
        })
    }
}

fn missing_required(spec: &ToolSpec, params: &[Param]) -> Vec<String> {
    spec.params
        .iter()
        .filter(|param| param.required)
        .filter(|param| !params.iter().any(|given| given.name == param.name))
        .map(|param| param.name.to_string())
        .collect()
}

fn unknown_params(spec: &ToolSpec, params: &[Param]) -> Vec<String> {
    params
        .iter()
        .filter(|given| !spec.params.iter().any(|param| param.name == given.name))
        .map(|given| given.name.clone())
        .collect()
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}
