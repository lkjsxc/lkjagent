use crate::model::{Action, Param, ACTION_CLOSE, ACTION_OPEN};

pub fn render_action(action: &Action) -> String {
    let mut output = format!("{ACTION_OPEN}\n");
    output.push_str(&render_pair("tool", &action.tool));
    for param in &action.params {
        output.push_str(&render_pair(&param.name, &param.value));
    }
    output.push_str(ACTION_CLOSE);
    output
}

pub fn render_observation(status: &str, content: &str) -> String {
    render_fields(
        "observation",
        &[Param::new("status", status), Param::new("content", content)],
    )
}

pub fn render_notice(kind: &str, content: &str) -> String {
    render_fields(
        "notice",
        &[Param::new("kind", kind), Param::new("content", content)],
    )
}

pub fn render_owner(content: &str) -> String {
    render_raw_frame("owner", content)
}

pub fn render_graph(content: &str) -> String {
    render_raw_frame("graph", content)
}

fn render_fields(tag: &str, fields: &[Param]) -> String {
    let mut output = String::new();
    output.push('<');
    output.push_str(tag);
    output.push_str(">\n");
    for field in fields {
        output.push_str(&render_pair(&field.name, &field.value));
    }
    output.push_str("</");
    output.push_str(tag);
    output.push('>');
    output
}

fn render_raw_frame(tag: &str, content: &str) -> String {
    let mut output = String::new();
    output.push('<');
    output.push_str(tag);
    output.push_str(">\n");
    output.push_str(content);
    output.push_str("\n</");
    output.push_str(tag);
    output.push('>');
    output
}

fn render_pair(name: &str, value: &str) -> String {
    if value.contains('\n') {
        format!("<{name}>\n{value}\n</{name}>\n")
    } else {
        format!("<{name}>{value}</{name}>\n")
    }
}
