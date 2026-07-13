use crate::runtime_decision::{
    ToolExampleParam, ToolFieldSpec, ToolSetView, ToolValueClass, ToolViewEntry,
};
use crate::runtime_tool_view::EffectKey;

#[rustfmt::skip]
pub const TOOL_DESCRIPTOR_FIELDS: &[&str] = &["name", "purpose", "field-order", "required-flags", "value-classes", "byte-count-bounds", "safe-example", "state-affordances", "admission-rules", "effect-key", "result-bound", "denial-code"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorField {
    pub name: &'static str,
    pub required: bool,
    pub value_class: ToolValueClass,
    pub min_bytes: usize,
    pub max_bytes: usize,
    pub minimum: Option<u64>,
    pub maximum: Option<u64>,
    pub safe_value: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub purpose: &'static str,
    pub fields: &'static [DescriptorField],
    pub state_affordances: &'static [&'static str],
    pub admission_rules: &'static [&'static str],
    pub effect_key: &'static str,
    pub result_max_bytes: usize,
    pub denial_code: &'static str,
}

const PATH: ToolValueClass = ToolValueClass::WorkspacePath;
const TEXT: ToolValueClass = ToolValueClass::Text;
const QUERY: ToolValueClass = ToolValueClass::Query;
const COUNT: ToolValueClass = ToolValueClass::Count;
const RULES: &[&str] = &["exact-fields", "workspace-relative-path", "bounded-values"];
const ORIENT: &[&str] = &["orient", "recovery"];
const MODIFY: &[&str] = &["modify", "recovery"];
const RECORD: &[&str] = &["orient", "modify", "recovery"];

const fn field(
    name: &'static str,
    required: bool,
    class: ToolValueClass,
    bytes: (usize, usize),
    counts: (Option<u64>, Option<u64>),
    safe: Option<&'static str>,
) -> DescriptorField {
    DescriptorField {
        name,
        required,
        value_class: class,
        min_bytes: bytes.0,
        max_bytes: bytes.1,
        minimum: counts.0,
        maximum: counts.1,
        safe_value: safe,
    }
}

#[rustfmt::skip]
const LIST_FIELDS: &[DescriptorField] = &[
    field("path", true, PATH, (1, 1024), (None, None), Some(".")),
    field("offset", false, COUNT, (1, 7), (Some(0), Some(1_000_000)), None),
    field("count", false, COUNT, (1, 3), (Some(1), Some(120)), Some("20")),
    field("complete", true, TEXT, (4, 5), (None, None), Some("true")),
];
#[rustfmt::skip]
const SEARCH_FIELDS: &[DescriptorField] = &[
    field("path", true, PATH, (1, 1024), (None, None), Some(".")),
    field("query", true, QUERY, (1, 1024), (None, None), Some("TODO")),
    field("offset", false, COUNT, (1, 7), (Some(0), Some(1_000_000)), None),
    field("count", false, COUNT, (1, 3), (Some(1), Some(120)), Some("20")),
];
#[rustfmt::skip]
const READ_FIELDS: &[DescriptorField] = &[
    field("path", true, PATH, (1, 1024), (None, None), Some("README.md")),
    field("offset", false, COUNT, (1, 7), (Some(0), Some(1_000_000)), None),
    field("count", false, COUNT, (1, 3), (Some(1), Some(120)), Some("20")),
    field("complete", true, TEXT, (4, 5), (None, None), Some("false")),
];
#[rustfmt::skip]
const EDIT_FIELDS: &[DescriptorField] = &[
    field("path", true, PATH, (1, 1024), (None, None), Some("notes/today.md")),
    field("old_text", true, TEXT, (1, 8192), (None, None), Some("draft")),
    field("new_text", true, TEXT, (0, 8192), (None, None), Some("final")),
];
#[rustfmt::skip]
const CREATE_FIELDS: &[DescriptorField] = &[
    field("path", true, PATH, (1, 1024), (None, None), Some("notes/new.md")),
    field("content", true, TEXT, (0, 8192), (None, None), Some("New note\n")),
];
#[rustfmt::skip]
const RECORD_FIELDS: &[DescriptorField] = &[
    field("family", true, TEXT, (6, 7), (None, None), Some("journal")),
    field("title", true, TEXT, (1, 256), (None, None), Some("A grounded day")),
    field("body", true, TEXT, (1, 1536), (None, None), Some("I recorded the owner-provided facts for today.")),
];

#[rustfmt::skip]
const DIRECT_CATALOG: &[ToolDescriptor] = &[
    descriptor("list_directory", "list one workspace directory; complete is required: true only for a no-change inventory report, false while locating a target", LIST_FIELDS, ORIENT, "workspace.list", 16_384, "list-denied"),
    descriptor("search_text", "search bounded workspace text", SEARCH_FIELDS, ORIENT, "workspace.search", 16_384, "search-denied"),
    descriptor("read_file", "read a numbered page; complete is required: true only when this read satisfies a no-change report objective, false when an edit may follow", READ_FIELDS, &["orient", "modify", "recovery"], "workspace.read", 32_768, "read-denied"),
    descriptor("edit_file", "replace one exact observed text span", EDIT_FIELDS, MODIFY, "workspace.edit", 8_192, "edit-denied"),
    descriptor("create_file", "create one observed-absent UTF-8 file", CREATE_FIELDS, MODIFY, "workspace.create", 8_192, "create-denied"),
    descriptor("write_record", "write one bounded grounded owner-visible journal or memory record", RECORD_FIELDS, RECORD, "workspace.record", 8_192, "record-denied"),
];

#[rustfmt::skip]
const fn descriptor(name: &'static str, purpose: &'static str, fields: &'static [DescriptorField],
    states: &'static [&'static str], effect: &'static str, result: usize,
    denial: &'static str) -> ToolDescriptor {
    ToolDescriptor { name, purpose, fields, state_affordances: states, admission_rules: RULES,
        effect_key: effect, result_max_bytes: result, denial_code: denial }
}

pub fn direct_catalog() -> &'static [ToolDescriptor] {
    DIRECT_CATALOG
}
pub fn direct_tool_view() -> ToolSetView {
    project(DIRECT_CATALOG.iter())
}

pub fn direct_tool_view_for_state(state: &str, intended_tool: Option<&str>) -> ToolSetView {
    if matches!(state, "review" | "respond" | "wait" | "idle") {
        return ToolSetView::empty();
    }
    let entries = DIRECT_CATALOG
        .iter()
        .filter(|tool| tool.state_affordances.contains(&state))
        .filter(|tool| state != "recovery" || intended_tool == Some(tool.name));
    project(entries)
}

fn project<'a>(descriptors: impl Iterator<Item = &'a ToolDescriptor>) -> ToolSetView {
    ToolSetView::new(descriptors.map(descriptor_entry).collect())
}

pub trait ToolProjection {
    fn projection(&self) -> ToolViewEntry;
}
pub fn descriptor_entry(tool: &impl ToolProjection) -> ToolViewEntry {
    tool.projection()
}
impl ToolProjection for ToolViewEntry {
    fn projection(&self) -> ToolViewEntry {
        self.clone()
    }
}
#[rustfmt::skip]
impl ToolProjection for ToolDescriptor {
    fn projection(&self) -> ToolViewEntry {
        let specs = self.fields.iter().map(|field| ToolFieldSpec { name: field.name.into(),
            required: field.required, value_class: field.value_class, min_bytes: field.min_bytes,
            max_bytes: field.max_bytes, minimum: field.minimum, maximum: field.maximum }).collect();
        let examples = self.fields.iter().filter_map(|field| field.safe_value.map(|value|
            ToolExampleParam { name: field.name.into(), value: value.into() })).collect();
        ToolViewEntry { name: self.name.into(), purpose: self.purpose.into(),
            required_params: self.fields.iter().filter(|field| field.required)
                .map(|field| field.name.into()).collect(),
            optional_params: self.fields.iter().filter(|field| !field.required)
                .map(|field| field.name.into()).collect(), field_specs: specs, example_params: examples,
            state_affordances: self.state_affordances.iter().map(|value| (*value).into()).collect(),
            admission_rules: self.admission_rules.iter().map(|value| (*value).into()).collect(),
            effect_key: EffectKey(self.effect_key.into()), result_max_bytes: self.result_max_bytes,
            denial_code: self.denial_code.into() }
    }
}
