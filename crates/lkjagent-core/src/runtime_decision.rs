use serde::{Deserialize, Serialize};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OperationKey(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputEnvelope {
    Content,
    Plan,
    Action,
    Message,
    Verdict,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolFieldSpec {
    pub name: String,
    pub required: bool,
    pub value_class: ToolValueClass,
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
}

impl ToolViewEntry {
    pub fn new(name: impl Into<String>, purpose: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            purpose: purpose.into(),
            required_params: Vec::new(),
            optional_params: Vec::new(),
            field_specs: Vec::new(),
        }
    }

    pub fn with_params(mut self, required: Vec<&str>, optional: Vec<&str>) -> Self {
        self.required_params = sorted_strings(required);
        self.optional_params = sorted_strings(optional);
        self.field_specs = field_specs(&self.required_params, &self.optional_params);
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDecision {
    pub id: String,
    pub case_id: String,
    pub operation: OperationKey,
    pub snapshot_fingerprint: String,
    pub state_vector_fingerprint: String,
    pub context_frame_fingerprint: String,
    pub tool_view: ToolSetView,
    pub expected_envelope: OutputEnvelope,
    pub model_budget_tokens: Option<u32>,
    pub evidence_requirements: Vec<String>,
    pub recovery_policy: String,
}

impl RuntimeDecision {
    pub fn new(
        id: impl Into<String>,
        case_id: impl Into<String>,
        operation: OperationKey,
        tool_view: ToolSetView,
        expected_envelope: OutputEnvelope,
    ) -> Self {
        Self {
            id: id.into(),
            case_id: case_id.into(),
            operation,
            snapshot_fingerprint: String::new(),
            state_vector_fingerprint: String::new(),
            context_frame_fingerprint: String::new(),
            tool_view,
            expected_envelope,
            model_budget_tokens: None,
            evidence_requirements: Vec::new(),
            recovery_policy: "default".to_string(),
        }
    }

    pub fn tool_view_fingerprint(&self) -> Result<String, FingerprintError> {
        self.tool_view.fingerprint()
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }
}

fn sorted_strings(values: Vec<&str>) -> Vec<String> {
    let mut strings: Vec<String> = values.into_iter().map(str::to_string).collect();
    strings.sort();
    strings
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

fn field_spec(name: &str, required: bool) -> ToolFieldSpec {
    ToolFieldSpec {
        name: name.to_string(),
        required,
        value_class: value_class(name),
    }
}

fn value_class(name: &str) -> ToolValueClass {
    match name {
        "path" => ToolValueClass::WorkspacePath,
        "command" => ToolValueClass::ShellCommand,
        "count" | "limit" | "budget" => ToolValueClass::Count,
        "query" => ToolValueClass::Query,
        _ => ToolValueClass::Text,
    }
}
