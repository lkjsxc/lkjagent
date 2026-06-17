use crate::error::ParseResult;
use crate::model::{Action, Param, ParseFault};
use crate::registry::{find_tool, ToolSpec};

pub fn parse_completion(text: &str) -> ParseResult<Action> {
    let lines: Vec<&str> = text.lines().collect();
    let start = find_act_start(&lines)?;
    let (action, next) = parse_act(&lines, start + 1)?;
    if lines.iter().skip(next).any(|line| is_open(line, "act")) {
        Err(ParseFault::MultipleAct)
    } else {
        Ok(action)
    }
}

fn find_act_start(lines: &[&str]) -> ParseResult<usize> {
    lines
        .iter()
        .position(|line| is_open(line, "act"))
        .ok_or(ParseFault::MissingAct)
}

fn parse_act(lines: &[&str], mut index: usize) -> ParseResult<(Action, usize)> {
    let mut tool: Option<String> = None;
    let mut params = Vec::new();
    let mut seen = Vec::new();

    loop {
        if index >= lines.len() {
            return Err(ParseFault::UnclosedTag {
                tag: "act".to_string(),
            });
        }
        let line = lines[index].trim_end();
        if line == "</act>" {
            let Some(tool_name) = tool else {
                return Err(ParseFault::MissingTool);
            };
            validate_params(&tool_name, &params)?;
            return Ok((Action::new(tool_name, params), index + 1));
        }
        if line == "<act>" {
            return Err(ParseFault::MultipleAct);
        }
        if line.is_empty() {
            index += 1;
            continue;
        }
        if !starts_pair(line) {
            return Err(non_pair_fault(tool.is_none(), line));
        }

        let (name, value, next) = parse_pair(lines, index)?;
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

fn parse_pair(lines: &[&str], index: usize) -> ParseResult<(String, String, usize)> {
    let line = lines[index].trim_end();
    if let Some((name, value)) = inline_pair(line) {
        return Ok((name, value, index + 1));
    }
    let Some(name) = open_name(line) else {
        return Err(ParseFault::MissingTool);
    };
    let close = format!("</{name}>");
    let mut value_lines = Vec::new();
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
    inline_pair(line).is_some() || open_name(line).is_some()
}

fn non_pair_fault(needs_tool: bool, line: &str) -> ParseFault {
    if needs_tool {
        ParseFault::MissingTool
    } else {
        ParseFault::BadParams {
            missing: Vec::new(),
            unknown: vec![line.to_string()],
        }
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
        Err(ParseFault::BadParams { missing, unknown })
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

fn is_open(line: &str, name: &str) -> bool {
    line.trim_end() == format!("<{name}>")
}

fn open_name(line: &str) -> Option<String> {
    let trimmed = line.trim_end();
    if !trimmed.starts_with('<') || !trimmed.ends_with('>') || trimmed.starts_with("</") {
        return None;
    }
    let name = &trimmed[1..trimmed.len().saturating_sub(1)];
    valid_name(name).then(|| name.to_string())
}

fn inline_pair(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_end();
    let open_end = trimmed.find('>')?;
    if !trimmed.starts_with('<') || trimmed.starts_with("</") {
        return None;
    }
    let name = &trimmed[1..open_end];
    if !valid_name(name) {
        return None;
    }
    let close = format!("</{name}>");
    if !trimmed.ends_with(&close) {
        return None;
    }
    let value_start = open_end + 1;
    let value_end = trimmed.len().saturating_sub(close.len());
    (value_start <= value_end).then(|| {
        (
            name.to_string(),
            trimmed[value_start..value_end].to_string(),
        )
    })
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}
