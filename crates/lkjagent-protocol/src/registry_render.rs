use crate::model::{ACTION_CLOSE, ACTION_OPEN};
use crate::registry::{ToolSpec, TOOLS};

pub fn render_registry_section() -> String {
    let mut lines = TOOLS.iter().map(render_tool).collect::<Vec<_>>();
    lines.push("doc.scaffold/doc.audit/artifact.*: do not use path; use root".to_string());
    lines.push(format!(
        "example graph.state: {ACTION_OPEN}<tool>graph.state</tool>{ACTION_CLOSE}"
    ));
    lines.push(format!(
        "example doc.audit: {ACTION_OPEN}<tool>doc.audit</tool><root>docs</root>{ACTION_CLOSE}"
    ));
    lines.join("\n")
}

fn render_tool(tool: &ToolSpec) -> String {
    format!("{}: {}; {}", tool.name, render_params(tool), tool.contract)
}

fn render_params(tool: &ToolSpec) -> String {
    if tool.params.is_empty() && tool.required_any.is_empty() {
        return "no params".to_string();
    }
    let mut rendered = tool
        .params
        .iter()
        .map(|param| match (param.required, param.default) {
            (true, _) => format!("{}!", param.name),
            (false, Some(default)) => format!("{}?={}", param.name, default),
            (false, None) => format!("{}?", param.name),
        })
        .collect::<Vec<_>>();
    rendered.extend(
        tool.required_any
            .iter()
            .map(|group| format!("{}!any", group.label)),
    );
    rendered.join(",")
}

#[cfg(test)]
mod tests {
    use super::render_registry_section;

    #[test]
    fn renders_no_param_tools_explicitly() {
        let rendered = render_registry_section();
        assert!(rendered.contains("graph.state: no params"));
        assert!(rendered.contains("<tool>graph.state</tool>"));
    }

    #[test]
    fn renders_root_hint_for_doc_tools() {
        let rendered = render_registry_section();
        assert!(rendered.contains("doc.scaffold"));
        assert!(rendered.contains("do not use path; use root"));
        assert!(rendered.contains("<root>docs</root>"));
    }
}
