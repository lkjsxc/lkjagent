use crate::runtime_decision::{ToolSetView, ToolViewEntry};

#[rustfmt::skip]
pub const TOOL_DESCRIPTOR_FIELDS: &[&str] = &["name", "purpose", "field-order", "required-flags", "value-classes", "byte-count-bounds", "safe-example", "state-affordances", "admission-rules", "effect-key", "result-bound", "denial-code"];

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub purpose: &'static str,
    pub effect: ToolEffect,
    pub required_params: &'static [&'static str],
    pub optional_params: &'static [&'static str],
    pub example_params: &'static [(&'static str, &'static str)],
}

#[rustfmt::skip]
const DIRECT_CATALOG: &[ToolDescriptor] = &[
    descriptor("list_directory", "bounded no-follow directory listing", ToolEffect::FsList, &["path"], &["offset", "count"]),
    descriptor("search_text", "bounded UTF-8 search below one path", ToolEffect::FsSearch, &["path", "query"], &["offset", "count"]),
    descriptor_with_examples("read_file", "numbered page with current SHA-256 revision", ToolEffect::FsRead, &["path"], &["offset", "count"], &[("path", "README.md"), ("count", "20")]),
    descriptor("edit_file", "single exact replacement against an observed revision", ToolEffect::FsWrite, &["path", "revision", "old_text", "new_text"], &[]),
    descriptor("create_file", "create observed-absent UTF-8 file without overwrite", ToolEffect::FsWrite, &["path", "content"], &[]),
];

#[rustfmt::skip]
const EXPLORE_CATALOG: &[ToolDescriptor] = &[
    descriptor_with_examples("fs.read", "read a workspace file", ToolEffect::FsRead, &["path"], &["offset", "count"], &[("path", "README.md"), ("count", "20")]),
    descriptor("fs.list", "list a workspace directory", ToolEffect::FsList, &[], &["path", "depth"]),
    descriptor("fs.tree", "show a bounded workspace tree", ToolEffect::FsTree, &[], &["path", "depth"]),
    descriptor("fs.search", "search workspace text", ToolEffect::FsSearch, &["query"], &["path"]),
    descriptor("fs.write", "write a workspace file", ToolEffect::FsWrite, &["path", "content"], &[]),
    descriptor("shell.run", "run a bounded shell command", ToolEffect::ShellRun, &["command"], &[]),
    descriptor("memory.find", "search durable memory", ToolEffect::MemoryFind, &["query"], &[]),
    descriptor("memory.save", "save durable memory", ToolEffect::MemorySave, &["topic", "content"], &[]),
    descriptor("plan.note", "record an exploration note", ToolEffect::PlanNote, &["note"], &[]),
];

pub fn direct_catalog() -> &'static [ToolDescriptor] {
    DIRECT_CATALOG
}

pub fn direct_tool_view() -> ToolSetView {
    ToolSetView::new(direct_catalog().iter().map(descriptor_entry).collect())
}

pub fn explore_catalog() -> &'static [ToolDescriptor] {
    EXPLORE_CATALOG
}

pub fn explore_tool_view() -> ToolSetView {
    ToolSetView::new(explore_catalog().iter().map(descriptor_entry).collect())
}

pub fn default_explore_tool_view() -> ToolSetView {
    tool_view_for_names(&["fs.read", "fs.search", "memory.find", "plan.note"])
}

pub fn shell_tool_view() -> ToolSetView {
    tool_view_for_names(&["shell.run"])
}

pub fn tool_view_for_names(names: &[&str]) -> ToolSetView {
    let entries = explore_catalog()
        .iter()
        .filter(|descriptor| names.contains(&descriptor.name))
        .map(descriptor_entry)
        .collect();
    ToolSetView::new(entries)
}

pub fn effect_for_tool(name: &str) -> Option<ToolEffect> {
    explore_catalog()
        .iter()
        .find(|descriptor| descriptor.name == name)
        .map(|descriptor| descriptor.effect)
}

pub fn descriptor_entry(descriptor: &ToolDescriptor) -> ToolViewEntry {
    ToolViewEntry::new(descriptor.name, descriptor.purpose)
        .with_params(
            descriptor.required_params.to_vec(),
            descriptor.optional_params.to_vec(),
        )
        .with_examples(descriptor.example_params.to_vec())
}

const fn descriptor(
    name: &'static str,
    purpose: &'static str,
    effect: ToolEffect,
    required_params: &'static [&'static str],
    optional_params: &'static [&'static str],
) -> ToolDescriptor {
    descriptor_with_examples(name, purpose, effect, required_params, optional_params, &[])
}

const fn descriptor_with_examples(
    name: &'static str,
    purpose: &'static str,
    effect: ToolEffect,
    required_params: &'static [&'static str],
    optional_params: &'static [&'static str],
    example_params: &'static [(&'static str, &'static str)],
) -> ToolDescriptor {
    ToolDescriptor {
        name,
        purpose,
        effect,
        required_params,
        optional_params,
        example_params,
    }
}
