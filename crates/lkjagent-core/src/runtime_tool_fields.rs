use std::collections::{BTreeMap, BTreeSet};

use crate::runtime_action_xml::{decode_entities, next_element};
use crate::runtime_tool_call::ToolCallError;

const MAX_FIELD_BYTES: usize = 4096;

#[derive(Default)]
pub struct Fields {
    pub scalars: BTreeMap<String, String>,
    pub args: BTreeMap<String, String>,
}

pub fn parse_fields(body: &str) -> Result<Fields, ToolCallError> {
    let mut at = 0;
    let mut fields = Fields::default();
    let mut input_seen = false;
    while let Some((tag, inner)) = next_element(body, &mut at, true)? {
        match tag {
            "decision_id" | "context_fingerprint" | "tool_name" => {
                insert_once(&mut fields.scalars, tag, decode_entities(inner.trim())?)?;
            }
            "input" if input_seen => return Err(ToolCallError::DuplicateTag("input".into())),
            "input" => {
                input_seen = true;
                insert_input(&mut fields.args, inner)?;
            }
            other => return Err(ToolCallError::UnknownTag(other.into())),
        }
    }
    if !input_seen {
        return Err(ToolCallError::ArgsSchemaViolation(
            "missing input".to_string(),
        ));
    }
    Ok(fields)
}

fn insert_input(args: &mut BTreeMap<String, String>, body: &str) -> Result<(), ToolCallError> {
    let mut at = 0;
    let mut seen = BTreeSet::new();
    while let Some((tag, inner)) = next_element(body, &mut at, false)? {
        if !seen.insert(tag.to_string()) {
            return Err(ToolCallError::DuplicateTag(format!("input/{tag}")));
        }
        let value = decode_entities(inner)?;
        if value.len() > MAX_FIELD_BYTES {
            return Err(ToolCallError::ArgsSchemaViolation(format!(
                "value too large for {tag}"
            )));
        }
        args.insert(tag.to_string(), value);
    }
    Ok(())
}

fn insert_once(
    scalars: &mut BTreeMap<String, String>,
    tag: &str,
    value: String,
) -> Result<(), ToolCallError> {
    if value.len() > MAX_FIELD_BYTES {
        return Err(ToolCallError::ArgsSchemaViolation(format!(
            "scalar too large for {tag}"
        )));
    }
    if scalars.insert(tag.to_string(), value).is_some() {
        return Err(ToolCallError::DuplicateTag(tag.into()));
    }
    Ok(())
}
