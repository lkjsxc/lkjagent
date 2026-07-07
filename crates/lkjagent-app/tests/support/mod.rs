#![allow(dead_code)]

pub fn action_chars(tool: &str, params: &[(char, &str)]) -> String {
    let pairs = params
        .iter()
        .map(|(kind, value)| (field_name(*kind), *value))
        .collect::<Vec<_>>();
    action_pairs(tool, &pairs)
}

pub fn shell_action(command: &str) -> String {
    action_pairs("shell.run", &[("command", command)])
}

pub fn memory_save(topic: &str, content: &str) -> String {
    action_pairs("memory.save", &[("topic", topic), ("content", content)])
}

pub fn finish(summary: &str) -> String {
    action_pairs("finish", &[("summary", summary)])
}

pub fn action_pairs(tool: &str, params: &[(&str, &str)]) -> String {
    action_for(
        "__DECISION_ID__",
        "__CONTEXT_FRAME_FINGERPRINT__",
        tool,
        params,
    )
}

pub fn action_for(decision: &str, context: &str, tool: &str, params: &[(&str, &str)]) -> String {
    let args = params
        .iter()
        .map(|(name, value)| format!("{}:{}", json(name), json(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "<lkjagent_action_v2>{{\"schema_version\":\"lkjagent.tool_call.v2\",\"decision_id\":{},\"tool_name\":{},\"args\":{{{args}}},\"context_frame_fingerprint\":{}}}</lkjagent_action_v2>",
        json(decision),
        json(tool),
        json(context)
    )
}

fn field_name(kind: char) -> &'static str {
    match kind {
        'p' => "path",
        'q' => "query",
        _ => "content",
    }
}

fn json(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}
