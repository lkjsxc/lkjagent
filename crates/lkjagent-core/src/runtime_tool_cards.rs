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
                    let count = match (spec.minimum, spec.maximum) {
                        (Some(min), Some(max)) => format!(":count={min}..{max}"),
                        _ => String::new(),
                    };
                    format!(
                        "{}:{need}:{:?}:bytes={}..{}{}",
                        spec.name, spec.value_class, spec.min_bytes, spec.max_bytes, count
                    )
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
        OutputEnvelope::Plan => plan_card(),
        envelope => generic_card(envelope),
    }
}

pub(crate) fn plan_example() -> &'static str {
    "<plan>\nwrite artifacts/task-output.md | Task output | words=300\nexplore | Read the relevant workspace source | budget=3\nrespond | Report created paths and checks\n</plan>"
}

fn plan_card() -> String {
    format!(
        "Output contract for this turn:\n- Return exactly one <plan> block.\n- Do not write prose before or after the block.\n- Put one action on each physical line.\n- Start every line with write, explore, or respond only.\n- Never emit plan, verify, or check actions; harness step labels are not output actions.\n- Use concrete objective-grounded values, never PATH, TITLE, GOAL, SUMMARY, or N.\n- Write paths are relative to the workspace root.\n- Do not start a path with /, ./, or ../; do not use . or .. path components.\n- Close with </plan>.\n\nFilled parser-valid example:\n{}",
        plan_example()
    )
}

fn tool_call_card(decision: &RuntimeDecision) -> String {
    let mut lines = vec![
        "Output contract for this turn:".to_string(),
        "- Return exactly one <lkjagent_action> block.".to_string(),
        "- Do not write prose before or after the block.".to_string(),
        "- Tags have no attributes and the body is not JSON.".to_string(),
        format!("- decision_id: {}", decision.id),
        format!("- harness_state: {}", decision.harness_state.as_str()),
        format!("- recovery_policy: {}", decision.recovery_policy),
        format!(
            "- context_fingerprint: {}",
            decision.context_frame_fingerprint
        ),
        "- Use one tool_name from the Tool view and include required arguments.".to_string(),
        "- Safe filled examples are copyable only when they match your intent.".to_string(),
        "- Schema-only FIELD_VALUE placeholders are rejected unchanged.".to_string(),
        "- Close with </lkjagent_action>.".to_string(),
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
        .map(|param| (param.name.as_str(), param.value.as_str()))
        .collect::<Vec<_>>();
    action_xml(decision, &entry.name, &args)
}

fn schema_block(decision: &RuntimeDecision, entry: Option<&ToolViewEntry>) -> Vec<String> {
    let Some(entry) = entry else {
        return action_xml(decision, "TOOL", &[]);
    };
    let args = entry
        .required_params
        .iter()
        .map(|field| (field.as_str(), "FIELD_VALUE"))
        .collect::<Vec<_>>();
    action_xml(decision, &entry.name, &args)
}

fn action_xml(decision: &RuntimeDecision, tool_name: &str, args: &[(&str, &str)]) -> Vec<String> {
    let mut lines = vec![
        "<lkjagent_action>".to_string(),
        format!("<decision_id>{}</decision_id>", escape_xml(&decision.id)),
        format!(
            "<context_fingerprint>{}</context_fingerprint>",
            escape_xml(&decision.context_frame_fingerprint)
        ),
        format!("<tool_name>{}</tool_name>", escape_xml(tool_name)),
    ];
    lines.push("<input>".to_string());
    for (name, value) in args {
        lines.push(format!("<{name}>{}</{name}>", escape_xml(value)));
    }
    lines.push("</input>".to_string());
    lines.push("</lkjagent_action>".to_string());
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

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}
