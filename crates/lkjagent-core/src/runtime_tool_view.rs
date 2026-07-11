use serde::{Deserialize, Serialize};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub struct ToolFieldSpec {
    pub name: String, pub required: bool, pub value_class: ToolValueClass,
    #[serde(default)] pub min_bytes: usize,
    #[serde(default = "default_max_bytes")] pub max_bytes: usize,
    #[serde(default)] pub minimum: Option<u64>, #[serde(default)] pub maximum: Option<u64>,
}

impl ToolFieldSpec {
    pub fn accepts_size(&self, value: &str) -> bool {
        (self.min_bytes..=self.max_bytes).contains(&value.len())
    }

    #[rustfmt::skip]
    pub fn canonical_count(&self, value: &str) -> Option<u64> {
        if self.value_class != ToolValueClass::Count || value.is_empty()
            || value.len() > 1 && value.starts_with('0')
            || !value.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
        let count = value.parse::<u64>().ok()?;
        if self.minimum.is_some_and(|minimum| count < minimum)
            || self.maximum.is_some_and(|maximum| count > maximum) { None } else { Some(count) }
    }
}

fn default_max_bytes() -> usize {
    4096
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolExampleParam {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolValueClass {
    Text,
    WorkspacePath,
    ShellCommand,
    Count,
    Query,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolViewEntry {
    pub name: String,
    pub purpose: String,
    pub required_params: Vec<String>,
    pub optional_params: Vec<String>,
    pub field_specs: Vec<ToolFieldSpec>,
    pub example_params: Vec<ToolExampleParam>,
}

impl ToolViewEntry {
    pub fn new(name: impl Into<String>, purpose: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            purpose: purpose.into(),
            required_params: Vec::new(),
            optional_params: Vec::new(),
            field_specs: Vec::new(),
            example_params: Vec::new(),
        }
    }

    pub fn with_params(mut self, required: Vec<&str>, optional: Vec<&str>) -> Self {
        self.required_params = sorted_strings(required);
        self.optional_params = sorted_strings(optional);
        self.field_specs = field_specs(&self.required_params, &self.optional_params);
        self
    }

    pub fn with_examples(mut self, examples: Vec<(&str, &str)>) -> Self {
        self.example_params = example_params(examples);
        self
    }

    pub fn accepts_param(&self, param: &str) -> bool {
        self.field_spec(param).is_some()
    }

    pub fn field_spec(&self, param: &str) -> Option<&ToolFieldSpec> {
        self.field_specs.iter().find(|spec| spec.name == param)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolSetView {
    pub entries: Vec<ToolViewEntry>,
}

impl ToolSetView {
    pub fn new(mut entries: Vec<ToolViewEntry>) -> Self {
        for entry in &mut entries {
            entry.required_params.sort();
            entry.optional_params.sort();
            entry
                .example_params
                .sort_by(|left, right| left.name.cmp(&right.name));
            entry.field_specs = field_specs(&entry.required_params, &entry.optional_params);
        }
        entries.sort_by(|left, right| left.name.cmp(&right.name));
        Self { entries }
    }

    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn tool_names(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| entry.name.clone())
            .collect()
    }

    pub fn entry(&self, name: &str) -> Option<&ToolViewEntry> {
        self.entries.iter().find(|entry| entry.name == name)
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }

    pub fn has_current_constraints(&self) -> bool {
        self.entries.iter().all(|entry| {
            entry.field_specs == field_specs(&entry.required_params, &entry.optional_params)
        })
    }
}

fn sorted_strings(values: Vec<&str>) -> Vec<String> {
    let mut strings: Vec<String> = values.into_iter().map(str::to_string).collect();
    strings.sort();
    strings
}

fn example_params(values: Vec<(&str, &str)>) -> Vec<ToolExampleParam> {
    let mut params = values
        .into_iter()
        .map(|(name, value)| ToolExampleParam {
            name: name.to_string(),
            value: value.to_string(),
        })
        .collect::<Vec<_>>();
    params.sort_by(|left, right| left.name.cmp(&right.name));
    params
}

fn field_specs(required: &[String], optional: &[String]) -> Vec<ToolFieldSpec> {
    let mut specs = required
        .iter()
        .map(|name| field_spec(name, true))
        .chain(optional.iter().map(|name| field_spec(name, false)))
        .collect::<Vec<_>>();
    specs.sort_by(|left, right| left.name.cmp(&right.name));
    specs
}

#[rustfmt::skip]
fn field_spec(name: &str, required: bool) -> ToolFieldSpec {
    let value_class = value_class(name);
    let (min_bytes, max_bytes) = if value_class == ToolValueClass::WorkspacePath { (1, 1024) }
        else if value_class == ToolValueClass::Count { (1, 20) } else { (usize::from(required), 4096) };
    let (minimum, maximum) = count_bounds(name);
    ToolFieldSpec { name: name.to_string(), required, value_class, min_bytes, max_bytes, minimum, maximum }
}

fn value_class(name: &str) -> ToolValueClass {
    match name {
        "path" => ToolValueClass::WorkspacePath,
        "command" => ToolValueClass::ShellCommand,
        "count" | "offset" | "depth" | "limit" | "budget" => ToolValueClass::Count,
        "query" => ToolValueClass::Query,
        _ => ToolValueClass::Text,
    }
}

fn count_bounds(name: &str) -> (Option<u64>, Option<u64>) {
    match name {
        "count" => (Some(1), Some(120)),
        "offset" => (Some(0), Some(1_000_000)),
        "depth" => (Some(1), Some(16)),
        "limit" => (Some(1), Some(100)),
        "budget" => (Some(1), Some(4096)),
        _ => (None, None),
    }
}
