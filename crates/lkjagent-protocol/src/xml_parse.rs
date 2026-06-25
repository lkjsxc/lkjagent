mod batch_files;

use crate::error::ParseResult;
use crate::model::{Action, MalformedTagReason, Param, ParseFault, ACTION_CLOSE, ACTION_OPEN};
use crate::registry::{find_tool, missing_required, missing_required_any, unknown_params};
use crate::tag_line::{classify_tag_line, valid_tag_name, TagLineClass};

pub fn parse_tag_action(lines: &[&str], mut index: usize) -> ParseResult<(Action, usize)> {
    let mut tool: Option<String> = None;
    let mut params = Vec::new();
    let mut seen = Vec::new();

    loop {
        if index >= lines.len() {
            return Err(ParseFault::UnclosedActionEnvelope);
        }
        let line = lines[index].trim_end();
        if line == ACTION_CLOSE {
            let Some(tool_name) = tool else {
                return Err(ParseFault::MissingTool);
            };
            validate_params(&tool_name, &params)?;
            return Ok((Action::new(tool_name, params), index + 1));
        }
        if line == ACTION_OPEN {
            return Err(ParseFault::MultipleActionEnvelopes);
        }
        if line.is_empty() {
            index += 1;
            continue;
        }
        if !starts_pair(line) {
            return Err(non_pair_fault(tool.as_deref(), &params, line));
        }

        let (name, value, next) = parse_tool_pair(lines, index, tool.as_deref())?;
        if seen.contains(&name) {
            return Err(ParseFault::DuplicateParam { name });
        }
        seen.push(name.clone());
        if tool.is_none() {
            if name != "tool" {
                return Err(ParseFault::MissingTool);
            }
            if find_tool(&value).is_none() {
                return Err(ParseFault::UnknownTool { tool: value });
            }
            tool = Some(value);
        } else {
            params.push(Param::new(name, value));
        }
        index = next;
    }
}

fn parse_tool_pair(
    lines: &[&str],
    index: usize,
    tool: Option<&str>,
) -> ParseResult<(String, String, usize)> {
    if tool == Some("fs.batch_write") && batch_files::starts_files_pair(lines[index]) {
        return batch_files::parse_batch_files_pair(lines, index);
    }
    parse_pair(lines, index)
}

pub(super) fn parse_pair(lines: &[&str], index: usize) -> ParseResult<(String, String, usize)> {
    let line = lines[index].trim_end();
    if let TagLineClass::InlineTag { name, value } = classify_tag_line(line) {
        return Ok((name, value, index + 1));
    }
    let Some((name, first_value)) = open_name_and_tail(line) else {
        return Err(ParseFault::MissingTool);
    };
    let close = format!("</{name}>");
    let mut value_lines = Vec::new();
    if !first_value.is_empty() {
        value_lines.push(first_value);
    }
    let mut cursor = index + 1;
    while let Some(raw) = lines.get(cursor) {
        if raw.trim_end() == close {
            return Ok((name, value_lines.join("\n"), cursor + 1));
        }
        value_lines.push((*raw).to_string());
        cursor += 1;
    }
    Err(ParseFault::UnclosedTag { tag: name })
}

fn starts_pair(line: &str) -> bool {
    matches!(
        classify_tag_line(line),
        TagLineClass::InlineTag { .. } | TagLineClass::OpenTag { .. }
    ) || open_name_and_tail(line).is_some()
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
        missing: find_tool(tool).map_or_else(Vec::new, |spec| {
            let names = param_names(params);
            let mut missing = missing_required(spec, &names);
            missing.extend(missing_required_any(spec, &names));
            missing
        }),
        unknown: vec![line.to_string()],
    }
}

fn validate_params(tool_name: &str, params: &[Param]) -> ParseResult<()> {
    let Some(spec) = find_tool(tool_name) else {
        return Err(ParseFault::UnknownTool {
            tool: tool_name.to_string(),
        });
    };
    let names = param_names(params);
    let mut missing = missing_required(spec, &names);
    missing.extend(missing_required_any(spec, &names));
    let unknown = unknown_params(spec, &names);
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

fn param_names(params: &[Param]) -> Vec<&str> {
    params.iter().map(|param| param.name.as_str()).collect()
}

pub fn is_action_open(line: &str) -> bool {
    line.trim_end() == ACTION_OPEN
}

pub(super) fn open_name_and_tail(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_end();
    if !trimmed.starts_with('<') || trimmed.starts_with("</") {
        return None;
    }
    let open_end = trimmed.find('>')?;
    let name = &trimmed[1..open_end];
    valid_tag_name(name).then(|| {
        (
            name.to_string(),
            trimmed[open_end.saturating_add(1)..].to_string(),
        )
    })
}
