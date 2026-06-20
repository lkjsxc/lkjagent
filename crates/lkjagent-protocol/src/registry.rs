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
pub struct ToolSpec {
    pub name: &'static str,
    pub params: &'static [ParamSpec],
    pub contract: &'static str,
}

pub fn find_tool(name: &str) -> Option<&'static ToolSpec> {
    TOOLS.iter().find(|tool| tool.name == name)
}
