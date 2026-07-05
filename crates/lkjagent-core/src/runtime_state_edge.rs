use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StateRef {
    pub kind: String,
    pub id: String,
}

impl StateRef {
    pub fn new(kind: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            id: id.into(),
        }
    }

    pub fn label(&self) -> String {
        format!("{}:{}", self.kind, self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StateEdgeRelation(pub String);

impl StateEdgeRelation {
    pub fn new(relation: impl Into<String>) -> Self {
        Self(relation.into())
    }

    pub fn depends_on() -> Self {
        Self::new("depends-on")
    }

    pub fn blocks() -> Self {
        Self::new("blocks")
    }

    pub fn derived_from() -> Self {
        Self::new("derived-from")
    }

    pub fn supersedes() -> Self {
        Self::new("supersedes")
    }

    pub fn conflicts_with() -> Self {
        Self::new("conflicts-with")
    }

    pub fn resolves() -> Self {
        Self::new("resolves")
    }

    pub fn verifies() -> Self {
        Self::new("verifies")
    }

    pub fn schedules() -> Self {
        Self::new("schedules")
    }

    pub fn owns() -> Self {
        Self::new("owns")
    }

    pub fn references() -> Self {
        Self::new("references")
    }

    pub fn tags() -> Self {
        Self::new("tags")
    }

    pub fn repeats() -> Self {
        Self::new("repeats")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateEdgeStatus {
    Active,
    Suppressed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeEvidenceRef {
    pub source_type: String,
    pub source_id: String,
    pub fingerprint: String,
}

impl EdgeEvidenceRef {
    pub fn new(
        source_type: impl Into<String>,
        source_id: impl Into<String>,
        fingerprint: impl Into<String>,
    ) -> Self {
        Self {
            source_type: source_type.into(),
            source_id: source_id.into(),
            fingerprint: fingerprint.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateEdge {
    pub id: String,
    pub scope: String,
    pub from_ref: StateRef,
    pub to_ref: StateRef,
    pub relation: StateEdgeRelation,
    pub reason: String,
    pub evidence_refs: Vec<EdgeEvidenceRef>,
    pub created_at: String,
    pub source_event_id: String,
    pub status: StateEdgeStatus,
    pub suppression_reason: Option<String>,
}

impl StateEdge {
    pub fn active(
        id: impl Into<String>,
        scope: impl Into<String>,
        from_ref: StateRef,
        to_ref: StateRef,
        relation: StateEdgeRelation,
        source_event_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            scope: scope.into(),
            from_ref,
            to_ref,
            relation,
            reason: String::new(),
            evidence_refs: Vec::new(),
            created_at: String::new(),
            source_event_id: source_event_id.into(),
            status: StateEdgeStatus::Active,
            suppression_reason: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = reason.into();
        self
    }

    pub fn with_evidence(mut self, evidence_refs: Vec<EdgeEvidenceRef>) -> Self {
        self.evidence_refs = evidence_refs;
        self
    }

    pub fn suppress(mut self, reason: impl Into<String>) -> Self {
        self.status = StateEdgeStatus::Suppressed;
        self.suppression_reason = Some(reason.into());
        self
    }

    pub fn sort_key(&self) -> (String, String, String, String, String) {
        (
            self.scope.clone(),
            self.relation.0.clone(),
            self.from_ref.label(),
            self.to_ref.label(),
            self.id.clone(),
        )
    }
}

pub fn sorted_edges(edges: impl IntoIterator<Item = StateEdge>) -> Vec<StateEdge> {
    let mut items: Vec<StateEdge> = edges.into_iter().collect();
    items.sort_by_key(StateEdge::sort_key);
    items
}

pub fn active_edges(edges: impl IntoIterator<Item = StateEdge>) -> Vec<StateEdge> {
    sorted_edges(
        edges
            .into_iter()
            .filter(|edge| edge.status == StateEdgeStatus::Active),
    )
}
