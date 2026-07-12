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

pub fn action_pairs(tool: &str, params: &[(&str, &str)]) -> String {
    action_for(
        "__DECISION_ID__",
        "__CONTEXT_FRAME_FINGERPRINT__",
        tool,
        params,
    )
}

pub fn action_for(_decision: &str, _context: &str, tool: &str, params: &[(&str, &str)]) -> String {
    let mut out = format!("<tool_call><tool>{}</tool><input>", xml(tool));
    for (name, value) in params {
        out.push_str(&format!("<{}>{}</{}>", xml(name), xml(value), xml(name)));
    }
    out.push_str("</input></tool_call>");
    out
}

fn field_name(kind: char) -> &'static str {
    match kind {
        'p' => "path",
        'q' => "query",
        _ => "content",
    }
}

fn xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}
