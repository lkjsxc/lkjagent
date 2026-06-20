use crate::model::GraphNodeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintRecord {
    pub summary: String,
    pub source: String,
    pub strength: ConstraintStrength,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentHypothesis {
    pub label: String,
    pub confidence: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceRecord {
    pub summary: String,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintStrength {
    Hard,
    Soft,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssumptionRecord {
    pub summary: String,
    pub status: FieldStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionRecord {
    pub question: String,
    pub status: FieldStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskRecord {
    pub summary: String,
    pub mitigation: String,
    pub status: FieldStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantRecord {
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessCriterion {
    pub summary: String,
    pub status: FieldStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRecord {
    pub node: GraphNodeId,
    pub summary: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldStatus {
    Open,
    Accepted,
    Resolved,
    Rejected,
}

impl ConstraintRecord {
    pub fn hard(summary: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            source: source.into(),
            strength: ConstraintStrength::Hard,
        }
    }
}
