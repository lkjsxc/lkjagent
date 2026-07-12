use crate::runtime_action_xml::{decode_entities, parse_document, ActionXmlError, Element};
use crate::runtime_admission::workspace_relative_path;
use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolFieldSpec, ToolValueClass};

pub const MODEL_ENVELOPES: &[&str] = &["tool_call", "final"];
pub const TOOL_CALL_FIELDS: &[&str] = &["tool", "input"];
pub const FINAL_FIELDS: &[&str] = &["message"];
const MAX_MODEL_BYTES: usize = 16_384;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCall {
    pub tool_name: String,
    pub args: Vec<(String, String)>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelValue {
    ToolCall(ToolCall),
    Final(String),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum ToolCallError {
    TooLarge, MissingRoot, MultipleRoots, EnvelopeMalformed, Attribute,
    ForbiddenSyntax, SelfClosing, UnclosedTag, CrossedTag, NestedTag,
    UnknownRoot, UnknownTag, DuplicateTag, BadEntity, WrongGrammarPhase,
    HiddenTool, MissingField, FieldOrder, ValueClass, Placeholder, UnsafePath, Bounds,
}
pub fn parse_model_value(
    raw: &str,
    decision: &RuntimeDecision,
) -> Result<ModelValue, ToolCallError> {
    if raw.len() > MAX_MODEL_BYTES {
        return Err(ToolCallError::TooLarge);
    }
    if raw.trim().is_empty() || !raw.trim_start().starts_with('<') {
        return Err(ToolCallError::MissingRoot);
    }
    let roots = raw.matches("<tool_call>").count() + raw.matches("<final>").count();
    if roots > 1 {
        return Err(ToolCallError::MultipleRoots);
    }
    let root = parse_document(raw).map_err(ToolCallError::from)?;
    match root.name {
        "tool_call" if decision.expected_envelope == OutputEnvelope::Action => {
            parse_tool_root(&root, decision).map(ModelValue::ToolCall)
        }
        "final" if decision.expected_envelope == OutputEnvelope::Message => {
            parse_final_root(&root).map(ModelValue::Final)
        }
        "tool_call" | "final" => Err(ToolCallError::WrongGrammarPhase),
        _ => Err(ToolCallError::UnknownRoot),
    }
}
pub fn parse_tool_call(raw: &str, decision: &RuntimeDecision) -> Result<ToolCall, ToolCallError> {
    match parse_model_value(raw, decision)? {
        ModelValue::ToolCall(call) => Ok(call),
        ModelValue::Final(_) => Err(ToolCallError::WrongGrammarPhase),
    }
}

fn parse_tool_root(
    root: &Element<'_>,
    decision: &RuntimeDecision,
) -> Result<ToolCall, ToolCallError> {
    exact_children(root, &["tool", "input"])?;
    let tool = leaf_text(&root.children[0])?;
    let entry = decision
        .tool_view
        .entry(&tool)
        .ok_or(ToolCallError::HiddenTool)?;
    let input = &root.children[1];
    if !input.text.is_empty() {
        return Err(ToolCallError::NestedTag);
    }
    let order = entry
        .field_specs
        .iter()
        .map(|spec| spec.name.as_str())
        .collect::<Vec<_>>();
    let mut last = None;
    let mut args = Vec::with_capacity(input.children.len());
    for field in &input.children {
        let index = order
            .iter()
            .position(|name| *name == field.name)
            .ok_or(ToolCallError::UnknownTag)?;
        if last.is_some_and(|prior| index <= prior) {
            return Err(if last == Some(index) {
                ToolCallError::DuplicateTag
            } else {
                ToolCallError::FieldOrder
            });
        }
        let spec = entry
            .field_spec(field.name)
            .ok_or(ToolCallError::UnknownTag)?;
        let value = leaf_text(field)?;
        validate_value(spec, &value)?;
        args.push((field.name.to_string(), value));
        last = Some(index);
    }
    if entry
        .field_specs
        .iter()
        .filter(|spec| spec.required)
        .any(|spec| !args.iter().any(|(name, _)| name == &spec.name))
    {
        return Err(ToolCallError::MissingField);
    }
    Ok(ToolCall {
        tool_name: tool,
        args,
    })
}

fn parse_final_root(root: &Element<'_>) -> Result<String, ToolCallError> {
    exact_children(root, &["message"])?;
    let message = leaf_text(&root.children[0])?;
    if message.is_empty() {
        Err(ToolCallError::MissingField)
    } else if message.len() > 4096 {
        Err(ToolCallError::Bounds)
    } else {
        Ok(message)
    }
}

fn exact_children(root: &Element<'_>, expected: &[&str]) -> Result<(), ToolCallError> {
    if !root.text.is_empty() {
        return Err(ToolCallError::NestedTag);
    }
    if root.children.len() < expected.len() {
        return Err(ToolCallError::MissingField);
    }
    if root.children.len() > expected.len() {
        return Err(ToolCallError::UnknownTag);
    }
    for (index, child) in root.children.iter().enumerate() {
        if child.name != expected[index] {
            if root
                .children
                .iter()
                .filter(|item| item.name == child.name)
                .count()
                > 1
            {
                return Err(ToolCallError::DuplicateTag);
            }
            return Err(if expected.contains(&child.name) {
                ToolCallError::FieldOrder
            } else {
                ToolCallError::UnknownTag
            });
        }
    }
    Ok(())
}

fn leaf_text(element: &Element<'_>) -> Result<String, ToolCallError> {
    if !element.children.is_empty() {
        return Err(ToolCallError::NestedTag);
    }
    decode_entities(element.text).map_err(ToolCallError::from)
}

fn validate_value(spec: &ToolFieldSpec, value: &str) -> Result<(), ToolCallError> {
    let upper = value.trim().to_ascii_uppercase();
    if matches!(
        upper.as_str(),
        "..." | "PATH" | "TOOL" | "TODO" | "VALUE" | "FIELD_VALUE" | "REPLACE_ME"
    ) {
        return Err(ToolCallError::Placeholder);
    }
    if !spec.accepts_size(value) {
        return Err(ToolCallError::Bounds);
    }
    match spec.value_class {
        ToolValueClass::WorkspacePath if !workspace_relative_path(value) => {
            Err(ToolCallError::UnsafePath)
        }
        ToolValueClass::Count if spec.canonical_count(value).is_none() => {
            Err(ToolCallError::ValueClass)
        }
        _ => Ok(()),
    }
}

impl From<ActionXmlError> for ToolCallError {
    fn from(error: ActionXmlError) -> Self {
        match error {
            ActionXmlError::Malformed => Self::EnvelopeMalformed,
            ActionXmlError::Attribute => Self::Attribute,
            ActionXmlError::ForbiddenSyntax => Self::ForbiddenSyntax,
            ActionXmlError::SelfClosing => Self::SelfClosing,
            ActionXmlError::Unclosed => Self::UnclosedTag,
            ActionXmlError::Crossed => Self::CrossedTag,
            ActionXmlError::Nested => Self::NestedTag,
            ActionXmlError::BadEntity => Self::BadEntity,
        }
    }
}
