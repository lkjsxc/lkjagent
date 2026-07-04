use std::collections::BTreeMap;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateKeyError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateKey {
    pub namespace: String,
    pub name: String,
}

impl Serialize for StateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_label())
    }
}

impl<'de> Deserialize<'de> for StateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let label = String::deserialize(deserializer)?;
        parse_state_key(&label).map_err(D::Error::custom)
    }
}

impl StateKey {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, StateKeyError> {
        let key = Self {
            namespace: namespace.into(),
            name: name.into(),
        };
        if key.namespace.is_empty() || key.name.is_empty() {
            return Err(StateKeyError {
                message: "state key namespace and name are required".to_string(),
            });
        }
        if key.namespace.contains(':') || key.name.contains(':') {
            return Err(StateKeyError {
                message: "state key parts must not contain ':'".to_string(),
            });
        }
        Ok(key)
    }

    pub fn as_label(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

fn parse_state_key(label: &str) -> Result<StateKey, String> {
    match label.split_once(':') {
        Some((namespace, name)) => StateKey::new(namespace, name).map_err(|err| err.message),
        None => Err("state key label must contain ':'".to_string()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateStatus {
    Active,
    Inactive,
    Suppressed,
    Resolved,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub source_type: String,
    pub source_id: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateCell {
    pub key: StateKey,
    pub status: StateStatus,
    pub priority: i32,
    pub confidence: u8,
    pub payload_schema: String,
    pub payload_json: String,
    pub evidence_refs: Vec<EvidenceRef>,
    pub source_event_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
    pub cooldown_until: Option<String>,
    pub conflict_group: Option<String>,
    pub parent_key: Option<StateKey>,
}

impl StateCell {
    pub fn active(key: StateKey, source_event_id: impl Into<String>) -> Self {
        Self {
            key,
            status: StateStatus::Active,
            priority: 0,
            confidence: 100,
            payload_schema: "empty".to_string(),
            payload_json: "{}".to_string(),
            evidence_refs: Vec::new(),
            source_event_id: source_event_id.into(),
            created_at: String::new(),
            updated_at: String::new(),
            expires_at: None,
            cooldown_until: None,
            conflict_group: None,
            parent_key: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    pub case_id: String,
    pub cells: BTreeMap<StateKey, StateCell>,
}

impl RuntimeSnapshot {
    pub fn empty(case_id: impl Into<String>) -> Self {
        Self {
            case_id: case_id.into(),
            cells: BTreeMap::new(),
        }
    }

    pub fn active_cells(&self) -> Vec<&StateCell> {
        self.cells
            .values()
            .filter(|cell| cell.status == StateStatus::Active)
            .collect()
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }
}
