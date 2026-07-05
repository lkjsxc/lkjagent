use serde_json::Value;

use crate::runtime_decision::{OutputEnvelope, ToolSetView, ToolViewEntry};
use crate::runtime_state::StateCell;

pub(crate) fn value(cell: &StateCell) -> Value {
    serde_json::from_str(&cell.payload_json).unwrap_or(Value::Null)
}

pub(crate) fn envelope(payload: &Value) -> OutputEnvelope {
    match text(payload, "expected_envelope") {
        Some("Content") => OutputEnvelope::Content,
        Some("Plan") => OutputEnvelope::Plan,
        Some("Action") => OutputEnvelope::Action,
        Some("Message") => OutputEnvelope::Message,
        Some("Verdict") => OutputEnvelope::Verdict,
        _ => OutputEnvelope::None,
    }
}

pub(crate) fn tool_view(payload: &Value) -> ToolSetView {
    let entries = payload
        .get("tool_view")
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(tool_entry).collect())
        .unwrap_or_default();
    ToolSetView::new(entries)
}

pub(crate) fn text<'a>(payload: &'a Value, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str)
}

pub(crate) fn number(payload: &Value, key: &str) -> Option<u32> {
    payload
        .get(key)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
}

pub(crate) fn tier(payload: &Value, default: u8) -> u8 {
    number(payload, "selector_tier")
        .and_then(|value| u8::try_from(value).ok())
        .unwrap_or(default)
}

pub(crate) fn evidence_requirements(payload: &Value) -> Vec<String> {
    strings(payload.get("evidence_requirements"))
}

fn tool_entry(value: &Value) -> Option<ToolViewEntry> {
    let mut entry = ToolViewEntry::new(text(value, "name")?, text(value, "purpose").unwrap_or(""));
    entry.required_params = strings(value.get("required_params"));
    entry.optional_params = strings(value.get("optional_params"));
    Some(entry)
}

fn strings(value: Option<&Value>) -> Vec<String> {
    let mut strings: Vec<String> = value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    strings.sort();
    strings
}
