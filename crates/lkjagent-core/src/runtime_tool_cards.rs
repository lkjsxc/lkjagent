use serde_json::json;

use crate::prompt_policy::envelope_tag;
use crate::runtime_decision::{
    OutputEnvelope, RuntimeDecision, ToolSetView, ToolValueClass, ToolViewEntry,
};

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
        OutputEnvelope::Action => tool_call_card(decision),
        envelope => generic_card(envelope),
    }
}

fn tool_call_card(decision: &RuntimeDecision) -> String {
    let mut lines = vec![
        "Output contract for this turn:".to_string(),
        "- Return exactly one <lkjagent_action_v2> block.".to_string(),
        "- Do not write prose before or after the block.".to_string(),
        "- The block body is canonical JSON, not XML fields.".to_string(),
        format!("- decision_id: {}", decision.id),
        format!(
            "- context_frame_fingerprint: {}",
            decision.context_frame_fingerprint
        ),
        "- Use one tool_name from the Tool view and include required args.".to_string(),
        "- Safe filled examples are copyable only when they match your intent.".to_string(),
        "- Schema-only FIELD_VALUE placeholders are rejected unchanged.".to_string(),
        "- Close with </lkjagent_action_v2>.".to_string(),
    ];
    if let Some(entry) = decision
        .tool_view
        .entries
        .iter()
        .find(|entry| has_safe_example(entry))
    {
        lines.push("\nSafe filled example:".to_string());
        lines.extend(example_block(decision, entry));
    } else {
        lines.push("\nSchema-only shape, not copyable:".to_string());
        lines.extend(schema_block(decision, decision.tool_view.entries.first()));
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

fn example_block(decision: &RuntimeDecision, entry: &ToolViewEntry) -> Vec<String> {
    let args = entry
        .example_params
        .iter()
        .map(|param| {
            (
                param.name.clone(),
                example_value(entry, &param.name, &param.value),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    action_json(decision, &entry.name, args)
}

fn schema_block(decision: &RuntimeDecision, entry: Option<&ToolViewEntry>) -> Vec<String> {
    let Some(entry) = entry else {
        return action_json(decision, "TOOL", serde_json::Map::new());
    };
    let args = entry
        .required_params
        .iter()
        .map(|field| ((*field).to_string(), json!("FIELD_VALUE")))
        .collect::<serde_json::Map<_, _>>();
    action_json(decision, &entry.name, args)
}

fn action_json(
    decision: &RuntimeDecision,
    tool_name: &str,
    args: serde_json::Map<String, serde_json::Value>,
) -> Vec<String> {
    let value = json!({
        "schema_version": "lkjagent.tool_call.v2",
        "decision_id": decision.id,
        "tool_name": tool_name,
        "args": args,
        "context_frame_fingerprint": decision.context_frame_fingerprint,
    });
    vec![
        "<lkjagent_action_v2>".to_string(),
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".to_string()),
        "</lkjagent_action_v2>".to_string(),
    ]
}

fn example_value(entry: &ToolViewEntry, name: &str, value: &str) -> serde_json::Value {
    match entry.field_spec(name).map(|spec| spec.value_class) {
        Some(ToolValueClass::Count) => value.parse::<u64>().map_or_else(|_| json!(0), |n| json!(n)),
        _ => json!(value),
    }
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
