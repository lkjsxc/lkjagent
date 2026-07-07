use std::collections::BTreeMap;

use serde_json::Value;

use crate::runtime_decision::{RuntimeDecision, ToolValueClass};
use crate::runtime_json_guard::{reject_duplicate_keys, JsonGuardError};

pub const ACTION_V2_OPEN: &str = "<lkjagent_action_v2>";
pub const ACTION_V2_CLOSE: &str = "</lkjagent_action_v2>";
pub const TOOL_CALL_V2_SCHEMA: &str = "lkjagent.tool_call.v2";

#[derive(Debug, Clone, PartialEq)]
pub struct ToolCallV2 {
    pub decision_id: String,
    pub tool_name: String,
    pub args: BTreeMap<String, Value>,
    pub context_frame_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolCallV2Error {
    NoActionFound,
    MultipleActionsFound,
    EnvelopeMalformed,
    JsonMalformed,
    DuplicateKey(String),
    SchemaVersionMismatch,
    DecisionMismatch,
    ToolUnknown,
    ArgsNotObject,
    UnknownTopLevel(String),
    ArgsSchemaViolation(String),
}

pub fn parse_tool_call_v2(
    raw: &str,
    decision: &RuntimeDecision,
) -> Result<ToolCallV2, ToolCallV2Error> {
    let body = action_body(raw)?;
    let value: Value = serde_json::from_str(body).map_err(|_| ToolCallV2Error::JsonMalformed)?;
    reject_duplicate_keys(body).map_err(map_json_guard_error)?;
    let object = value
        .as_object()
        .ok_or(ToolCallV2Error::EnvelopeMalformed)?;
    reject_unknown_top_level(object.keys())?;

    let schema_version = required_string(object, "schema_version")?;
    if schema_version != TOOL_CALL_V2_SCHEMA {
        return Err(ToolCallV2Error::SchemaVersionMismatch);
    }
    let decision_id = required_string(object, "decision_id")?;
    if decision_id != decision.id {
        return Err(ToolCallV2Error::DecisionMismatch);
    }
    let tool_name = required_string(object, "tool_name")?;
    let entry = decision
        .tool_view
        .entry(tool_name)
        .ok_or(ToolCallV2Error::ToolUnknown)?;
    let context_frame_fingerprint = required_string(object, "context_frame_fingerprint")?;
    let args = object
        .get("args")
        .and_then(Value::as_object)
        .ok_or(ToolCallV2Error::ArgsNotObject)?;

    for (name, value) in args {
        let Some(spec) = entry.field_spec(name) else {
            return Err(ToolCallV2Error::ArgsSchemaViolation(format!(
                "unknown arg {name}"
            )));
        };
        validate_value_class(name, spec.value_class, value)?;
    }
    for required in &entry.required_params {
        if !args.contains_key(required) {
            return Err(ToolCallV2Error::ArgsSchemaViolation(format!(
                "missing arg {required}"
            )));
        }
    }

    Ok(ToolCallV2 {
        decision_id: decision_id.to_string(),
        tool_name: tool_name.to_string(),
        args: args.clone().into_iter().collect(),
        context_frame_fingerprint: context_frame_fingerprint.to_string(),
    })
}

fn action_body(raw: &str) -> Result<&str, ToolCallV2Error> {
    let trimmed = raw.trim();
    let opens = trimmed.matches(ACTION_V2_OPEN).count();
    let closes = trimmed.matches(ACTION_V2_CLOSE).count();
    if opens == 0 && closes == 0 {
        return Err(ToolCallV2Error::NoActionFound);
    }
    if opens > 1 || closes > 1 {
        return Err(ToolCallV2Error::MultipleActionsFound);
    }
    if opens != 1
        || closes != 1
        || !trimmed.starts_with(ACTION_V2_OPEN)
        || !trimmed.ends_with(ACTION_V2_CLOSE)
    {
        return Err(ToolCallV2Error::EnvelopeMalformed);
    }
    let start = ACTION_V2_OPEN.len();
    let end = trimmed.len() - ACTION_V2_CLOSE.len();
    Ok(trimmed[start..end].trim())
}

fn reject_unknown_top_level<'a>(
    keys: impl Iterator<Item = &'a String>,
) -> Result<(), ToolCallV2Error> {
    for key in keys {
        match key.as_str() {
            "schema_version"
            | "decision_id"
            | "tool_name"
            | "args"
            | "context_frame_fingerprint" => {}
            _ => return Err(ToolCallV2Error::UnknownTopLevel(key.clone())),
        }
    }
    Ok(())
}

fn required_string<'a>(
    object: &'a serde_json::Map<String, Value>,
    key: &str,
) -> Result<&'a str, ToolCallV2Error> {
    object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| ToolCallV2Error::ArgsSchemaViolation(format!("missing string {key}")))
}

fn validate_value_class(
    name: &str,
    value_class: ToolValueClass,
    value: &Value,
) -> Result<(), ToolCallV2Error> {
    let valid = match value_class {
        ToolValueClass::Count => value.as_u64().is_some(),
        ToolValueClass::Text
        | ToolValueClass::WorkspacePath
        | ToolValueClass::ShellCommand
        | ToolValueClass::Query => value.is_string(),
    };
    if valid {
        Ok(())
    } else {
        Err(ToolCallV2Error::ArgsSchemaViolation(format!(
            "wrong primitive for {name}"
        )))
    }
}

fn map_json_guard_error(error: JsonGuardError) -> ToolCallV2Error {
    match error {
        JsonGuardError::JsonMalformed => ToolCallV2Error::JsonMalformed,
        JsonGuardError::DuplicateKey(pointer) => ToolCallV2Error::DuplicateKey(pointer),
    }
}
