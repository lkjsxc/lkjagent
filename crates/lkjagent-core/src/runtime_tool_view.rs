use serde::{Deserialize, Serialize};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct EffectKey(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolFieldSpec {
    pub name: String,
    pub required: bool,
    pub value_class: ToolValueClass,
    #[serde(default)]
    pub min_bytes: usize,
    #[serde(default = "default_max_bytes")]
    pub max_bytes: usize,
    #[serde(default)]
    pub minimum: Option<u64>,
    #[serde(default)]
    pub maximum: Option<u64>,
}

impl ToolFieldSpec {
    pub fn accepts_size(&self, value: &str) -> bool {
        (self.min_bytes..=self.max_bytes).contains(&value.len())
    }

    pub fn canonical_count(&self, value: &str) -> Option<u64> {
        if self.value_class != ToolValueClass::Count
            || value.is_empty()
            || value.len() > 1 && value.starts_with('0')
            || !value.bytes().all(|byte| byte.is_ascii_digit())
        {
            return None;
        }
        let count = value.parse::<u64>().ok()?;
        (self.minimum.is_none_or(|minimum| count >= minimum)
            && self.maximum.is_none_or(|maximum| count <= maximum))
        .then_some(count)
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
    #[serde(default)]
    pub required_params: Vec<String>,
    #[serde(default)]
    pub optional_params: Vec<String>,
    #[serde(default)]
    pub field_specs: Vec<ToolFieldSpec>,
    #[serde(default)]
    pub example_params: Vec<ToolExampleParam>,
    #[serde(default)]
    pub state_affordances: Vec<String>,
    #[serde(default)]
    pub admission_rules: Vec<String>,
    #[serde(default)]
    pub effect_key: EffectKey,
    #[serde(default)]
    pub result_max_bytes: usize,
    #[serde(default)]
    pub denial_code: String,
}

impl ToolViewEntry {
    #[rustfmt::skip]
    pub fn new(name: impl Into<String>, purpose: impl Into<String>) -> Self {
        Self { name: name.into(), purpose: purpose.into(), required_params: Vec::new(),
            optional_params: Vec::new(), field_specs: Vec::new(), example_params: Vec::new(),
            state_affordances: Vec::new(), admission_rules: Vec::new(),
            effect_key: EffectKey::default(), result_max_bytes: 0, denial_code: String::new() }
    }

    pub fn with_params(mut self, required: Vec<&str>, optional: Vec<&str>) -> Self {
        self.required_params = strings(required);
        self.optional_params = strings(optional);
        self.field_specs = inferred_specs(&self.required_params, &self.optional_params);
        self
    }

    pub fn with_examples(mut self, examples: Vec<(&str, &str)>) -> Self {
        self.example_params = examples
            .into_iter()
            .map(|(name, value)| ToolExampleParam {
                name: name.into(),
                value: value.into(),
            })
            .collect();
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
    #[rustfmt::skip]
    pub fn has_current_constraints(&self) -> bool {
        self.entries.iter().all(|entry| {
            let direct = crate::runtime_tool_catalog::direct_catalog().iter()
                .find(|descriptor| descriptor.name == entry.name).map(crate::runtime_tool_catalog::descriptor_entry);
            direct.as_ref() == Some(entry)
        })
    }
}

fn strings(values: Vec<&str>) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn inferred_specs(required: &[String], optional: &[String]) -> Vec<ToolFieldSpec> {
    required
        .iter()
        .map(|name| inferred_spec(name, true))
        .chain(optional.iter().map(|name| inferred_spec(name, false)))
        .collect()
}

#[rustfmt::skip]
fn inferred_spec(name: &str, required: bool) -> ToolFieldSpec {
    let value_class = match name {
        "path" => ToolValueClass::WorkspacePath,
        "command" => ToolValueClass::ShellCommand,
        "count" | "offset" | "depth" | "limit" | "budget" => ToolValueClass::Count,
        "query" => ToolValueClass::Query,
        _ => ToolValueClass::Text,
    };
    let (minimum, maximum) = match name {
        "count" => (Some(1), Some(120)),
        "offset" => (Some(0), Some(1_000_000)),
        "depth" => (Some(1), Some(16)),
        "limit" => (Some(1), Some(100)),
        "budget" => (Some(1), Some(4096)),
        _ => (None, None),
    };
    let (min_bytes, max_bytes) = match value_class {
        ToolValueClass::WorkspacePath => (1, 1024),
        ToolValueClass::Count => (1, 20),
        _ => (usize::from(required), 4096),
    };
    ToolFieldSpec { name: name.into(), required, value_class, min_bytes, max_bytes,
        minimum, maximum }
}
