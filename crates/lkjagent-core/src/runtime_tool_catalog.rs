use crate::runtime_decision::{ToolSetView, ToolViewEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolEffect {
    FsRead,
    FsList,
    FsTree,
    FsSearch,
    FsWrite,
    ShellRun,
    MemoryFind,
    MemorySave,
    PlanNote,
    Finish,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub purpose: &'static str,
    pub effect: ToolEffect,
    pub required_params: &'static [&'static str],
    pub optional_params: &'static [&'static str],
}

const EXPLORE_CATALOG: &[ToolDescriptor] = &[
    descriptor(
        "fs.read",
        "read a workspace file",
        ToolEffect::FsRead,
        &["path"],
        &["offset", "count"],
    ),
    descriptor(
        "fs.list",
        "list a workspace directory",
        ToolEffect::FsList,
        &[],
        &["path", "depth"],
    ),
    descriptor(
        "fs.tree",
        "show a bounded workspace tree",
        ToolEffect::FsTree,
        &[],
        &["path", "depth"],
    ),
    descriptor(
        "fs.search",
        "search workspace text",
        ToolEffect::FsSearch,
        &["query"],
        &["path"],
    ),
    descriptor(
        "fs.write",
        "write a workspace file",
        ToolEffect::FsWrite,
        &["path", "content"],
        &[],
    ),
    descriptor(
        "shell.run",
        "run a bounded shell command",
        ToolEffect::ShellRun,
        &["command"],
        &[],
    ),
    descriptor(
        "memory.find",
        "search durable memory",
        ToolEffect::MemoryFind,
        &["query"],
        &[],
    ),
    descriptor(
        "memory.save",
        "save durable memory",
        ToolEffect::MemorySave,
        &["topic", "content"],
        &[],
    ),
    descriptor(
        "plan.note",
        "record an exploration note",
        ToolEffect::PlanNote,
        &["note"],
        &[],
    ),
    descriptor(
        "finish",
        "finish exploration",
        ToolEffect::Finish,
        &["summary"],
        &[],
    ),
];

pub fn explore_catalog() -> &'static [ToolDescriptor] {
    EXPLORE_CATALOG
}

pub fn explore_tool_view() -> ToolSetView {
    ToolSetView::new(explore_catalog().iter().map(descriptor_entry).collect())
}

pub fn effect_for_tool(name: &str) -> Option<ToolEffect> {
    explore_catalog()
        .iter()
        .find(|descriptor| descriptor.name == name)
        .map(|descriptor| descriptor.effect)
}

pub fn descriptor_entry(descriptor: &ToolDescriptor) -> ToolViewEntry {
    ToolViewEntry::new(descriptor.name, descriptor.purpose).with_params(
        descriptor.required_params.to_vec(),
        descriptor.optional_params.to_vec(),
    )
}

const fn descriptor(
    name: &'static str,
    purpose: &'static str,
    effect: ToolEffect,
    required_params: &'static [&'static str],
    optional_params: &'static [&'static str],
) -> ToolDescriptor {
    ToolDescriptor {
        name,
        purpose,
        effect,
        required_params,
        optional_params,
    }
}
