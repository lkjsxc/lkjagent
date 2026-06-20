use crate::model::GraphNodeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionRecord {
    pub from: GraphNodeId,
    pub to: GraphNodeId,
    pub decision: TransitionOutcome,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionOutcome {
    Admitted,
    Deferred,
    Refused,
    Recovered,
}
