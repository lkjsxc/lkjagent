use crate::registry::{ToolSpec, TOOLS};

pub fn render_registry_section() -> String {
    TOOLS.iter().map(render_tool).collect::<Vec<_>>().join("\n")
}

fn render_tool(tool: &ToolSpec) -> String {
    let mut line = format!("{}: {}; {}", tool.name, render_params(tool), tool.contract);
    if let Some(hint) = hint(tool.name) {
        line.push('\n');
        line.push_str(hint);
    }
    if let Some(example) = example(tool.name) {
        line.push_str("\nexample:\n");
        line.push_str(example);
    }
    line
}

fn render_params(tool: &ToolSpec) -> String {
    if tool.params.is_empty() {
        return "no params".to_string();
    }
    tool.params
        .iter()
        .map(|param| match (param.required, param.default) {
            (true, _) => format!("{} req", param.name),
            (false, Some(default)) => format!("{} opt {}", param.name, default),
            (false, None) => format!("{} opt", param.name),
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn hint(tool: &str) -> Option<&'static str> {
    match tool {
        "doc.scaffold" | "doc.audit" => Some("do not use path; use root"),
        "graph.state" | "graph.next" | "graph.audit" | "graph.recover" => {
            Some("do not add path; this tool takes no parameters")
        }
        _ => None,
    }
}

fn example(tool: &str) -> Option<&'static str> {
    match tool {
        "graph.state" => Some("<act>\n<tool>graph.state</tool>\n</act>"),
        "doc.scaffold" => Some(
            "<act>\n<tool>doc.scaffold</tool>\n<root>docs</root>\n<title>Project Documentation</title>\n<kind>documentation</kind>\n</act>",
        ),
        "doc.audit" => Some(
            "<act>\n<tool>doc.audit</tool>\n<root>docs</root>\n</act>",
        ),
        "fs.list" => Some(
            "<act>\n<tool>fs.list</tool>\n<path>.</path>\n<depth>2</depth>\n</act>",
        ),
        "workspace.summary" => Some(
            "<act>\n<tool>workspace.summary</tool>\n<path>.</path>\n</act>",
        ),
        _ => None,
    }
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
