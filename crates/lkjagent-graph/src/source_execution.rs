use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, EXEC_PACKAGES, EXEC_TOOLS, OBSERVE_EVIDENCE};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "execute-step",
        NodeKind::Execution,
        "execute exactly one active plan step",
        OBSERVE_EVIDENCE,
        EXEC_PACKAGES,
        EXEC_TOOLS,
    ),
    node(
        "observe-result",
        NodeKind::State,
        "convert tool result into evidence or fault",
        OBSERVE_EVIDENCE,
        EXEC_PACKAGES,
        EXEC_TOOLS,
    ),
    node(
        "repair-step",
        NodeKind::Execution,
        "repair a failed step with smaller scope",
        OBSERVE_EVIDENCE,
        EXEC_PACKAGES,
        EXEC_TOOLS,
    ),
    node(
        "advance-plan",
        NodeKind::State,
        "mark completed step and choose the next one",
        OBSERVE_EVIDENCE,
        EXEC_PACKAGES,
        EXEC_TOOLS,
    ),
];
