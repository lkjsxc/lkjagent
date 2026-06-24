pub use crate::registry_render::render_registry_section;
pub use crate::registry_spec::TOOLS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamSpec {
    pub name: &'static str,
    pub required: bool,
    pub default: Option<&'static str>,
}

impl ParamSpec {
    pub const fn req(name: &'static str) -> Self {
        Self {
            name,
            required: true,
            default: None,
        }
    }

    pub const fn opt(name: &'static str, default: Option<&'static str>) -> Self {
        Self {
            name,
            required: false,
            default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequiredAnySpec {
    pub names: &'static [&'static str],
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: &'static str,
    pub params: &'static [ParamSpec],
    pub required_any: &'static [RequiredAnySpec],
    pub contract: &'static str,
}

pub fn find_tool(name: &str) -> Option<&'static ToolSpec> {
    TOOLS.iter().find(|tool| tool.name == name)
}

pub fn missing_required(spec: &ToolSpec, names: &[&str]) -> Vec<String> {
    spec.params
        .iter()
        .filter(|param| param.required)
        .filter(|param| !names.contains(&param.name))
        .map(|param| param.name.to_string())
        .collect()
}

pub fn missing_required_any(spec: &ToolSpec, names: &[&str]) -> Vec<String> {
    spec.required_any
        .iter()
        .filter(|group| !group.names.iter().any(|name| names.contains(name)))
        .map(|group| group.label.to_string())
        .collect()
}

pub fn unknown_params(spec: &ToolSpec, names: &[&str]) -> Vec<String> {
    names
        .iter()
        .filter(|name| !spec.params.iter().any(|param| param.name == **name))
        .map(|name| (*name).to_string())
        .collect()
}
