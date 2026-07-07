use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Number, Value};

use crate::runtime_action_xml::{decode_entities, next_element, ActionXmlError};
use crate::runtime_decision::{RuntimeDecision, ToolValueClass};

pub const ACTION_OPEN: &str = "<lkjagent_action>";
pub const ACTION_CLOSE: &str = "</lkjagent_action>";

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
        args.insert(
            name.clone(),
            value_for_class(&name, spec.value_class, &text)?,
        );
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

#[derive(Default)]
struct Fields {
    scalars: BTreeMap<String, String>,
    args: BTreeMap<String, String>,
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

fn parse_fields(body: &str) -> Result<Fields, ToolCallError> {
    let mut at = 0;
    let mut fields = Fields::default();
    while let Some((tag, inner)) = next_element(body, &mut at, true)? {
        match tag {
            "decision_id" | "context_fingerprint" | "tool_name" => {
                insert_once(&mut fields.scalars, tag, decode_entities(inner.trim())?)?;
            }
            "argument" => insert_argument(&mut fields.args, inner)?,
            other => return Err(ToolCallError::UnknownTag(other.into())),
        }
    }
    Ok(fields)
}

fn insert_argument(args: &mut BTreeMap<String, String>, body: &str) -> Result<(), ToolCallError> {
    let mut at = 0;
    let mut seen = BTreeSet::new();
    let (mut name, mut value) = (None, None);
    while let Some((tag, inner)) = next_element(body, &mut at, false)? {
        if !matches!(tag, "name" | "value") {
            return Err(ToolCallError::UnknownTag(tag.into()));
        }
        if !seen.insert(tag.to_string()) {
            return Err(ToolCallError::DuplicateTag(tag.into()));
        }
        if tag == "name" {
            name = Some(decode_entities(inner.trim())?);
        } else {
            value = Some(decode_entities(inner)?);
        }
    }
    let name = name.ok_or_else(|| schema("missing arg name".into()))?;
    let value = value.ok_or_else(|| schema("missing arg value".into()))?;
    if args.insert(name.clone(), value).is_some() {
        return Err(ToolCallError::DuplicateTag(format!("argument/{name}")));
    }
    Ok(())
}

fn insert_once(
    scalars: &mut BTreeMap<String, String>,
    tag: &str,
    value: String,
) -> Result<(), ToolCallError> {
    if scalars.insert(tag.to_string(), value).is_some() {
        return Err(ToolCallError::DuplicateTag(tag.into()));
    }
    Ok(())
}

fn required<'a>(map: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str, ToolCallError> {
    map.get(key)
        .map(String::as_str)
        .ok_or_else(|| schema(format!("missing {key}")))
}

fn value_for_class(name: &str, class: ToolValueClass, text: &str) -> Result<Value, ToolCallError> {
    match class {
        ToolValueClass::Count => text
            .trim()
            .parse::<u64>()
            .map(|n| Value::Number(Number::from(n)))
            .map_err(|_| schema(format!("wrong primitive for {name}"))),
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
