use crate::prompt_policy::envelope_tag;
use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry};

pub(crate) fn render_tool_view(view: &ToolSetView) -> String {
    view.entries
        .iter()
        .map(|entry| {
            let required = entry.required_params.join(",");
            let optional = entry.optional_params.join(",");
            let fields = entry
                .field_specs
                .iter()
                .map(|spec| {
                    let need = if spec.required {
                        "required"
                    } else {
                        "optional"
                    };
                    format!("{}:{need}:{:?}", spec.name, spec.value_class)
                })
                .collect::<Vec<_>>()
                .join(",");
            let example = example_summary(entry);
            format!(
                "- {}: {} required={required} optional={optional} fields={fields}{example}",
                entry.name, entry.purpose
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn protocol_card(decision: &RuntimeDecision) -> String {
    match decision.expected_envelope {
        OutputEnvelope::Action => tool_call_card(&decision.tool_view),
        envelope => generic_card(envelope),
    }
}

fn tool_call_card(view: &ToolSetView) -> String {
    let mut lines = vec![
        "Output contract for this turn:".to_string(),
        "- Return exactly one <tool_call> block.".to_string(),
        "- Do not write prose before or after the block.".to_string(),
        "- Use one tool_name from the Tool view and include required fields.".to_string(),
        "- Safe filled examples are copyable only when they match your intent.".to_string(),
        "- Schema-only FIELD_VALUE placeholders are rejected unchanged.".to_string(),
        "- Close with </tool_call>.".to_string(),
    ];
    if let Some(entry) = view.entries.iter().find(|entry| has_safe_example(entry)) {
        lines.push("\nSafe filled example:".to_string());
        lines.extend(example_block(entry));
    } else {
        lines.push("\nSchema-only shape, not copyable:".to_string());
        lines.extend(schema_block(view.entries.first()));
    }
    lines.join("\n")
}

fn has_safe_example(entry: &ToolViewEntry) -> bool {
    !entry.example_params.is_empty()
        && entry
            .required_params
            .iter()
            .all(|name| entry.example_params.iter().any(|param| &param.name == name))
}

fn example_block(entry: &ToolViewEntry) -> Vec<String> {
    let mut lines = vec![
        "<tool_call>".to_string(),
        format!("<tool_name>{}</tool_name>", entry.name),
    ];
    for param in &entry.example_params {
        lines.push(format!("<{}>{}</{}>", param.name, param.value, param.name));
    }
    lines.push("</tool_call>".to_string());
    lines
}

fn schema_block(entry: Option<&ToolViewEntry>) -> Vec<String> {
    let tool_name = entry.map_or("TOOL", |entry| entry.name.as_str());
    let mut lines = vec![
        "<tool_call>".to_string(),
        format!("<tool_name>{tool_name}</tool_name>"),
    ];
    if let Some(entry) = entry {
        for field in &entry.required_params {
            lines.push(format!("<{field}>FIELD_VALUE</{field}>"));
        }
    }
    lines.push("</tool_call>".to_string());
    lines
}

fn generic_card(envelope: OutputEnvelope) -> String {
    let tag = envelope_tag(envelope).unwrap_or("no_output");
    if envelope == OutputEnvelope::None {
        return "Output contract for this turn:\n- No model output expected.".to_string();
    }
    format!(
        "Output contract for this turn:\n- Return exactly one <{tag}> block.\n- Do not write prose before or after the block.\n- Close with </{tag}>.\n\nCopy this shape:\n<{tag}>\n...\n</{tag}>"
    )
}

fn example_summary(entry: &ToolViewEntry) -> String {
    if entry.example_params.is_empty() {
        return String::new();
    }
    let values = entry
        .example_params
        .iter()
        .map(|param| format!("{}={}", param.name, param.value))
        .collect::<Vec<_>>()
        .join(",");
    format!(" example={values}")
}
