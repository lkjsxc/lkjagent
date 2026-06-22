use serde_json::{Map, Value};

use crate::error::ParseResult;
use crate::model::{Action, Param, ParseFault};
use crate::registry::{find_tool, ToolSpec};

pub fn parse_json_action(text: &str) -> ParseResult<Action> {
    let trimmed = text.trim();
    let value: Value = serde_json::from_str(trimmed).map_err(|error| ParseFault::BadEnvelope {
        reason: format!("invalid JSON action envelope: {error}"),
    })?;
    let root = as_object(&value, "root envelope")?;
    reject_unknown(root, &["schema", "action"], "root envelope")?;
    if let Some(schema) = root.get("schema") {
        require_string(schema, "schema").and_then(|name| {
            if name == "lkj-action" {
                Ok(())
            } else {
                Err(ParseFault::BadEnvelope {
                    reason: "schema must be lkj-action".to_string(),
                })
            }
        })?;
    }
    let Some(action) = root.get("action") else {
        return Err(ParseFault::MissingAct);
    };
    parse_action_object(action)
}

fn parse_action_object(value: &Value) -> ParseResult<Action> {
    let object = as_object(value, "action")?;
    reject_unknown(object, &["tool", "params"], "action")?;
    let Some(tool_value) = object.get("tool") else {
        return Err(ParseFault::MissingTool);
    };
    let tool = require_string(tool_value, "tool")?.to_string();
    let Some(spec) = find_tool(&tool) else {
        return Err(ParseFault::UnknownTool { tool });
    };
    let params = match object.get("params") {
        Some(value) => parse_params(&tool, value)?,
        None => Vec::new(),
    };
    validate_params(spec, &tool, &params)?;
    Ok(Action::new(tool, params))
}

fn parse_params(tool: &str, value: &Value) -> ParseResult<Vec<Param>> {
    let object = as_object(value, "params")?;
    let mut params = Vec::new();
    for (name, value) in object {
        params.push(Param::new(name.clone(), param_value(tool, name, value)?));
    }
    Ok(params)
}

fn param_value(tool: &str, name: &str, value: &Value) -> ParseResult<String> {
    if tool == "fs.batch_write" && name == "files" {
        return batch_files_value(value);
    }
    match value {
        Value::String(text) => Ok(text.clone()),
        Value::Number(number) => Ok(number.to_string()),
        Value::Bool(flag) => Ok(flag.to_string()),
        Value::Null => Err(ParseFault::BadEnvelope {
            reason: format!("param '{name}' must not be null"),
        }),
        Value::Array(_) | Value::Object(_) => Err(ParseFault::BadEnvelope {
            reason: format!("param '{name}' must be a string, number, or bool"),
        }),
    }
}

fn batch_files_value(value: &Value) -> ParseResult<String> {
    match value {
        Value::String(text) => Ok(text.clone()),
        Value::Array(items) => batch_file_blocks(items),
        _ => Err(ParseFault::BadEnvelope {
            reason: "fs.batch_write files must be a string or array".to_string(),
        }),
    }
}

fn batch_file_blocks(items: &[Value]) -> ParseResult<String> {
    let mut blocks = Vec::new();
    for item in items {
        let object = as_object(item, "fs.batch_write file")?;
        reject_unknown(object, &["path", "content"], "fs.batch_write file")?;
        let path = object
            .get("path")
            .ok_or_else(|| missing_param("fs.batch_write", "path"))
            .and_then(|value| require_string(value, "path"))?;
        let content = object
            .get("content")
            .ok_or_else(|| missing_param("fs.batch_write", "content"))
            .and_then(|value| require_string(value, "content"))?;
        blocks.push(format!("path: {path}\ncontent:\n{content}"));
    }
    if blocks.is_empty() {
        return Err(ParseFault::BadParams {
            tool: "fs.batch_write".to_string(),
            missing: vec!["files".to_string()],
            unknown: Vec::new(),
        });
    }
    Ok(blocks.join("\n-- lkjagent-next-file --\n"))
}

fn as_object<'a>(value: &'a Value, label: &str) -> ParseResult<&'a Map<String, Value>> {
    value.as_object().ok_or_else(|| ParseFault::BadEnvelope {
        reason: format!("{label} must be an object"),
    })
}

fn require_string<'a>(value: &'a Value, label: &str) -> ParseResult<&'a str> {
    value.as_str().ok_or_else(|| ParseFault::BadEnvelope {
        reason: format!("{label} must be a string"),
    })
}

fn reject_unknown(object: &Map<String, Value>, allowed: &[&str], label: &str) -> ParseResult<()> {
    let unknown: Vec<String> = object
        .keys()
        .filter(|key| {
            !allowed
                .iter()
                .any(|allowed_key| allowed_key == &key.as_str())
        })
        .cloned()
        .collect();
    if unknown.is_empty() {
        Ok(())
    } else {
        Err(ParseFault::BadEnvelope {
            reason: format!("{label} has unknown fields: {}", unknown.join(", ")),
        })
    }
}

fn validate_params(spec: &ToolSpec, tool: &str, params: &[Param]) -> ParseResult<()> {
    let missing: Vec<String> = spec
        .params
        .iter()
        .filter(|param| param.required)
        .filter(|param| !params.iter().any(|given| given.name == param.name))
        .map(|param| param.name.to_string())
        .collect();
    let unknown: Vec<String> = params
        .iter()
        .filter(|given| !spec.params.iter().any(|param| param.name == given.name))
        .map(|given| given.name.clone())
        .collect();
    if missing.is_empty() && unknown.is_empty() {
        Ok(())
    } else {
        Err(ParseFault::BadParams {
            tool: tool.to_string(),
            missing,
            unknown,
        })
    }
}

fn missing_param(tool: &str, name: &str) -> ParseFault {
    ParseFault::BadParams {
        tool: tool.to_string(),
        missing: vec![name.to_string()],
        unknown: Vec::new(),
    }
}
