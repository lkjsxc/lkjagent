use crate::runtime_decision::{ToolSetView, ToolViewEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub purpose: &'static str,
    pub required_params: &'static [&'static str],
    pub optional_params: &'static [&'static str],
}

const EXPLORE_CATALOG: &[ToolDescriptor] = &[
    descriptor(
        "fs.read",
        "read a workspace file",
        &["path"],
        &["offset", "count"],
    ),
    descriptor(
        "fs.list",
        "list a workspace directory",
        &[],
        &["path", "depth"],
    ),
    descriptor(
        "fs.tree",
        "show a bounded workspace tree",
        &[],
        &["path", "depth"],
    ),
    descriptor("fs.search", "search workspace text", &["query"], &["path"]),
    descriptor(
        "fs.write",
        "write a workspace file",
        &["path", "content"],
        &[],
    ),
    descriptor(
        "shell.run",
        "run a bounded shell command",
        &["command"],
        &[],
    ),
    descriptor("memory.find", "search durable memory", &["query"], &[]),
    descriptor(
        "memory.save",
        "save durable memory",
        &["topic", "content"],
        &[],
    ),
    descriptor("plan.note", "record an exploration note", &["note"], &[]),
    descriptor("finish", "finish exploration", &["summary"], &[]),
];

pub fn explore_catalog() -> &'static [ToolDescriptor] {
    EXPLORE_CATALOG
}

pub fn explore_tool_view() -> ToolSetView {
    ToolSetView::new(explore_catalog().iter().map(descriptor_entry).collect())
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
    required_params: &'static [&'static str],
    optional_params: &'static [&'static str],
) -> ToolDescriptor {
    ToolDescriptor {
        name,
        purpose,
        required_params,
        optional_params,
    }
}
