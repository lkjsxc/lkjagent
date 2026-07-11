use std::collections::BTreeMap;

use serde_json::{Number, Value};

use crate::runtime_action_xml::ActionXmlError;
use crate::runtime_decision::{RuntimeDecision, ToolFieldSpec, ToolValueClass};
use crate::runtime_tool_fields::parse_fields;

pub const ACTION_OPEN: &str = "<lkjagent_action>";
pub const ACTION_CLOSE: &str = "</lkjagent_action>";
const MAX_ACTION_BYTES: usize = 16_384;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub decision_id: String,
    pub tool_name: String,
    pub args: BTreeMap<String, Value>,
    pub context_frame_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolCallError {
    NoActionFound,
    MultipleActionsFound,
    EnvelopeMalformed,
    UnclosedTag(String),
    CrossedTag(String),
    Attribute(String),
    UnknownTag(String),
    DuplicateTag(String),
    JsonLike,
    BadEntity(String),
    DecisionMismatch,
    ContextMismatch,
    ToolUnknown,
    ArgsSchemaViolation(String),
}

pub fn parse_tool_call(raw: &str, decision: &RuntimeDecision) -> Result<ToolCall, ToolCallError> {
    if raw.len() > MAX_ACTION_BYTES {
        return Err(schema("action too large".to_string()));
    }
    let body = action_body(raw)?.trim();
    if body.starts_with('{') || body.starts_with('[') {
        return Err(ToolCallError::JsonLike);
    }
    let fields = parse_fields(body)?;
    let decision_id = required(&fields.scalars, "decision_id")?;
    if decision_id != decision.id {
        return Err(ToolCallError::DecisionMismatch);
    }
    let context = required(&fields.scalars, "context_fingerprint")?;
    if context != decision.context_frame_fingerprint {
        return Err(ToolCallError::ContextMismatch);
    }
    let tool_name = required(&fields.scalars, "tool_name")?;
    let entry = decision
        .tool_view
        .entry(tool_name)
        .ok_or(ToolCallError::ToolUnknown)?;
    let mut args = BTreeMap::new();
    for (name, text) in fields.args {
        let Some(spec) = entry.field_spec(&name) else {
            return Err(schema(format!("unknown arg {name}")));
        };
        args.insert(name.clone(), value_for_spec(&name, spec, &text)?);
    }
    for required in &entry.required_params {
        if !args.contains_key(required) {
            return Err(schema(format!("missing arg {required}")));
        }
    }
    Ok(ToolCall {
        decision_id: decision_id.to_string(),
        tool_name: tool_name.to_string(),
        args,
        context_frame_fingerprint: context.to_string(),
    })
}

fn action_body(raw: &str) -> Result<&str, ToolCallError> {
    let trimmed = raw.trim();
    let opens = trimmed.matches("<lkjagent_action").count();
    let closes = trimmed.matches(ACTION_CLOSE).count();
    if opens == 0 && closes == 0 {
        return Err(ToolCallError::NoActionFound);
    }
    if opens > 1 || closes > 1 {
        return Err(ToolCallError::MultipleActionsFound);
    }
    if trimmed.starts_with("<lkjagent_action ") || trimmed.starts_with("<lkjagent_action\t") {
        return Err(ToolCallError::Attribute("lkjagent_action".into()));
    }
    if !trimmed.starts_with(ACTION_OPEN) || !trimmed.ends_with(ACTION_CLOSE) {
        return Err(ToolCallError::EnvelopeMalformed);
    }
    Ok(&trimmed[ACTION_OPEN.len()..trimmed.len() - ACTION_CLOSE.len()])
}

fn required<'a>(map: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str, ToolCallError> {
    map.get(key)
        .map(String::as_str)
        .ok_or_else(|| schema(format!("missing {key}")))
}

fn value_for_spec(name: &str, spec: &ToolFieldSpec, text: &str) -> Result<Value, ToolCallError> {
    if !spec.accepts_size(text) {
        return Err(schema(format!("value size out of bounds for {name}")));
    }
    match spec.value_class {
        ToolValueClass::Count => spec
            .canonical_count(text)
            .map(|number| Value::Number(Number::from(number)))
            .ok_or_else(|| schema(format!("wrong primitive for {name}"))),
        _ => Ok(Value::String(text.to_string())),
    }
}

fn schema(message: String) -> ToolCallError {
    ToolCallError::ArgsSchemaViolation(message)
}

impl From<ActionXmlError> for ToolCallError {
    fn from(error: ActionXmlError) -> Self {
        match error {
            ActionXmlError::EnvelopeMalformed => Self::EnvelopeMalformed,
            ActionXmlError::UnclosedTag(tag) => Self::UnclosedTag(tag),
            ActionXmlError::CrossedTag(tag) => Self::CrossedTag(tag),
            ActionXmlError::Attribute(tag) => Self::Attribute(tag),
            ActionXmlError::UnknownTag(tag) => Self::UnknownTag(tag),
            ActionXmlError::BadEntity(entity) => Self::BadEntity(entity),
        }
    }
}
