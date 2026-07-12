use std::collections::BTreeMap;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};
use crate::runtime_state_edge::{active_edges, StateEdge};

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
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.as_label())
    }
}

impl<'de> Deserialize<'de> for StateKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let label = String::deserialize(deserializer)?;
        Self::from_label(&label).map_err(|error| D::Error::custom(error.message))
    }
}

impl StateKey {
    pub fn from_label(label: &str) -> Result<Self, StateKeyError> {
        let (namespace, name) = label.split_once(':').ok_or_else(|| StateKeyError {
            message: "state key label must contain ':'".to_string(),
        })?;
        Self::new(namespace, name)
    }

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
                message: "state key namespace and name are required".into(),
            });
        }
        if key.namespace.contains(':') || key.name.contains(':') {
            return Err(StateKeyError {
                message: "state key parts must not contain ':'".into(),
            });
        }
        Ok(key)
    }

    pub fn as_label(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
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

pub const STATE_STATUSES: &[&str] = &["active", "inactive", "suppressed", "resolved", "blocked"];
pub const MATTER_STATES: &[&str] = &["open", "waiting", "blocked", "closed"];
pub const RUNTIME_PHASES: &[&str] = &["orient", "modify", "review", "respond", "idle"];
pub const NEED_KINDS: &[&str] = &[
    "target",
    "source-revision",
    "edit",
    "check",
    "response",
    "owner-fact",
];
pub const FAULT_KINDS: &[&str] = &[
    "protocol",
    "admission",
    "stale-file",
    "effect",
    "endpoint",
    "check",
    "stasis",
];
pub const WAKE_KINDS: &[&str] = &[
    "immediate",
    "time",
    "owner-input",
    "file-change",
    "config-change",
];

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
            payload_schema: "empty".into(),
            payload_json: "{}".into(),
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
    pub edges: BTreeMap<String, StateEdge>,
}

impl RuntimeSnapshot {
    pub fn empty(case_id: impl Into<String>) -> Self {
        Self {
            case_id: case_id.into(),
            cells: BTreeMap::new(),
            edges: BTreeMap::new(),
        }
    }

    pub fn active_cells(&self) -> Vec<&StateCell> {
        self.cells
            .values()
            .filter(|cell| cell.status == StateStatus::Active)
            .collect()
    }

    pub fn active_edges(&self) -> Vec<StateEdge> {
        active_edges(self.edges.values().cloned())
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }
}

pub use crate::runtime_eligibility::CurrentTime;
pub use crate::runtime_operation::{MatterLifecycle, RuntimePhase, RuntimeState};

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRequirement { pub check_name: String, pub artifact_fingerprint: String }
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckEvidence { pub check_name: String, pub artifact_fingerprint: String, pub passed: bool,
    pub decision_id: String, pub created_at: String }
#[rustfmt::skip]
pub fn can_close(requirements: &[CompletionRequirement], evidence: &[CheckEvidence]) -> bool {
    !requirements.is_empty() && requirements.iter().all(|required| evidence.iter().any(|item|
        item.passed && item.check_name == required.check_name && item.artifact_fingerprint == required.artifact_fingerprint))
}
