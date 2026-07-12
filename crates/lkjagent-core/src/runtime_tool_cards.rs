use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolViewEntry};

pub(crate) fn protocol_card(decision: &RuntimeDecision) -> String {
    match decision.expected_envelope {
        OutputEnvelope::Action => tool_call_card(decision),
        OutputEnvelope::Message => "Output contract: return exactly <final><message>owner-facing answer</message></final>. No prose outside it.".into(),
        OutputEnvelope::None => {
            "Output contract for this turn:\n- No model output expected.".into()
        }
    }
}

fn tool_call_card(decision: &RuntimeDecision) -> String {
    let Some(entry) = decision
        .tool_view
        .entries
        .iter()
        .find(|entry| has_example(entry))
    else {
        return "Output contract: return one compact <tool_call><tool>allowed tool</tool><input></input></tool_call>. No prose or JSON.".into();
    };
    format!(
        "Output contract: return one compact <tool_call> with <tool> then <input>. No prose, attributes, JSON, IDs, or fingerprints. Fields must follow the shown order.\nParser-valid example:\n{}",
        example(entry)
    )
}

fn has_example(entry: &ToolViewEntry) -> bool {
    entry
        .field_specs
        .iter()
        .filter(|spec| spec.required)
        .all(|spec| {
            entry
                .example_params
                .iter()
                .any(|param| param.name == spec.name)
        })
}

fn example(entry: &ToolViewEntry) -> String {
    let mut input = String::new();
    for spec in &entry.field_specs {
        if let Some(param) = entry
            .example_params
            .iter()
            .find(|param| param.name == spec.name)
        {
            input.push_str(&format!(
                "<{}>{}</{}>",
                spec.name,
                escape_xml(&param.value),
                spec.name
            ));
        }
    }
    format!(
        "<tool_call><tool>{}</tool><input>{input}</input></tool_call>",
        entry.name
    )
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}
