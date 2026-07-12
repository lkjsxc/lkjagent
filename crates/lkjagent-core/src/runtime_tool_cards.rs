use crate::prompt_policy::envelope_tag;
use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry};
use crate::runtime_tool_catalog::ToolEffect;
use crate::runtime_tool_view::EffectKey;

pub(crate) fn render_tool_view(view: &ToolSetView) -> String {
    view.entries
        .iter()
        .map(|entry| {
            let fields = entry
                .field_specs
                .iter()
                .map(|spec| {
                    let need = if spec.required { "!" } else { "?" };
                    match (spec.minimum, spec.maximum) {
                        (Some(min), Some(max)) => format!("{}{}={min}..{max}", spec.name, need),
                        _ => format!(
                            "{}{}={}..{}B",
                            spec.name, need, spec.min_bytes, spec.max_bytes
                        ),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            format!("- {}: {} [{}]", entry.name, entry.purpose, fields)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn protocol_card(decision: &RuntimeDecision) -> String {
    match decision.expected_envelope {
        OutputEnvelope::Action => tool_call_card(decision),
        OutputEnvelope::Plan => plan_card(),
        OutputEnvelope::Message => final_card(),
        envelope => generic_card(envelope),
    }
}

pub(crate) fn plan_example() -> &'static str {
    "<plan>\nwrite artifacts/task-output.md | Task output | words=300\nexplore | Read the relevant workspace source | budget=3\nrespond | Report created paths and checks\n</plan>"
}

fn plan_card() -> String {
    format!(
        "Output contract for this turn:\n- Return exactly one <plan> block.\n- Do not write prose before or after the block.\n- Put one action on each physical line.\n- Start every line with write, explore, or respond only.\n- Never emit plan, verify, or check actions; harness step labels are not output actions.\n- Use concrete objective-grounded values, never PATH, TITLE, GOAL, SUMMARY, or N.\n- Write paths are relative to the workspace root.\n- Do not start a path with /, ./, or ../; do not use . or .. path components.\n- Close with </plan>.\n\nFilled parser-valid example:\n{}",
        plan_example()
    )
}

fn tool_call_card(decision: &RuntimeDecision) -> String {
    let Some(entry) = decision
        .tool_view
        .entries
        .iter()
        .find(|entry| has_example(entry))
    else {
        return "Output contract: return one compact <tool_call><tool>allowed tool</tool><input></input></tool_call>. No prose or JSON.".into();
    };
    format!(
        "Output contract: return one compact <tool_call> with <tool> then <input>. No prose, attributes, JSON, IDs, or fingerprints. Fields must follow the shown order.\nParser-valid example:\n{}",
        example(entry)
    )
}

fn final_card() -> String {
    "Output contract: return exactly <final><message>owner-facing answer</message></final>. No prose outside it.".into()
}

fn has_example(entry: &ToolViewEntry) -> bool {
    entry
        .field_specs
        .iter()
        .filter(|spec| spec.required)
        .all(|spec| {
            entry
                .example_params
                .iter()
                .any(|param| param.name == spec.name)
        })
}

fn example(entry: &ToolViewEntry) -> String {
    let mut input = String::new();
    for spec in &entry.field_specs {
        if let Some(param) = entry
            .example_params
            .iter()
            .find(|param| param.name == spec.name)
        {
            input.push_str(&format!(
                "<{}>{}</{}>",
                spec.name,
                escape_xml(&param.value),
                spec.name
            ));
        }
    }
    format!(
        "<tool_call><tool>{}</tool><input>{input}</input></tool_call>",
        entry.name
    )
}

fn generic_card(envelope: OutputEnvelope) -> String {
    let tag = envelope_tag(envelope).unwrap_or("no_output");
    if envelope == OutputEnvelope::None {
        return "Output contract for this turn:\n- No model output expected.".into();
    }
    format!("Output contract for this turn:\n- Return exactly one <{tag}> block.\n- Do not write prose before or after the block.\n- Close with </{tag}>.\n\nCopy this shape:\n<{tag}>\n...\n</{tag}>")
}

// Retired APIs stay separate from direct descriptors and decision projections.
type LegacyDescriptor = (
    &'static str,
    &'static str,
    ToolEffect,
    &'static [&'static str],
    &'static [&'static str],
);
#[rustfmt::skip]
const EXPLORE: &[LegacyDescriptor] = &[
    ("fs.read", "read a workspace file", ToolEffect::FsRead, &["path"], &["offset", "count"]),
    ("fs.list", "list a workspace directory", ToolEffect::FsList, &[], &["path", "depth"]),
    ("fs.tree", "show a bounded workspace tree", ToolEffect::FsTree, &[], &["path", "depth"]),
    ("fs.search", "search workspace text", ToolEffect::FsSearch, &["query"], &["path"]),
    ("fs.write", "write a workspace file", ToolEffect::FsWrite, &["path", "content"], &[]),
    ("shell.run", "run a bounded shell command", ToolEffect::ShellRun, &["command"], &[]),
    ("memory.find", "search durable memory", ToolEffect::MemoryFind, &["query"], &[]),
    ("memory.save", "save durable memory", ToolEffect::MemorySave, &["topic", "content"], &[]),
    ("plan.note", "record an exploration note", ToolEffect::PlanNote, &["note"], &[]),
];

pub fn explore_tool_view() -> ToolSetView {
    legacy_view(EXPLORE.iter().map(|item| item.0))
}
pub fn default_explore_tool_view() -> ToolSetView {
    legacy_view(["fs.read", "fs.search", "memory.find", "plan.note"].into_iter())
}
pub fn shell_tool_view() -> ToolSetView {
    legacy_view(["shell.run"].into_iter())
}
pub fn effect_for_tool(name: &str) -> Option<ToolEffect> {
    EXPLORE
        .iter()
        .find(|item| item.0 == name)
        .map(|item| item.2)
}
pub fn explore_catalog() -> Vec<ToolViewEntry> {
    explore_tool_view().entries
}
pub fn tool_view_for_names(names: &[&str]) -> ToolSetView {
    legacy_view(names.iter().copied())
}
fn legacy_view<'a>(names: impl Iterator<Item = &'a str>) -> ToolSetView {
    ToolSetView::new(
        names
            .filter_map(|name| EXPLORE.iter().find(|item| item.0 == name))
            .map(|item| {
                let mut entry = ToolViewEntry::new(item.0, item.1)
                    .with_params(item.3.to_vec(), item.4.to_vec());
                entry.effect_key = EffectKey(format!("retired.{}", item.0));
                entry.result_max_bytes = 4096;
                entry.denial_code = "retired-tool-denied".into();
                entry
            })
            .collect(),
    )
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}
