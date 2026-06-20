use crate::registry::{ToolSpec, TOOLS};

pub fn render_registry_section() -> String {
    TOOLS
        .iter()
        .map(|tool| format!("{}: {}; {}", tool.name, render_params(tool), tool.contract))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_params(tool: &ToolSpec) -> String {
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
