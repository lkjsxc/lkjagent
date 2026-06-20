use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{
    node, EXEC_PACKAGES, EXEC_TOOLS, NO_EVIDENCE, OBSERVE_EVIDENCE, VERIFY_EVIDENCE,
    VERIFY_PACKAGES, VERIFY_TOOLS,
};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "execute",
        NodeKind::Execution,
        "execute the active plan step with graph-allowed tools",
        OBSERVE_EVIDENCE,
        EXEC_PACKAGES,
        EXEC_TOOLS,
    ),
    node(
        "observe",
        NodeKind::State,
        "observe tool result and bind evidence to the graph case",
        OBSERVE_EVIDENCE,
        EXEC_PACKAGES,
        EXEC_TOOLS,
    ),
    node(
        "integrate-evidence",
        NodeKind::State,
        "update plan step, touched paths, evidence, and next branch",
        OBSERVE_EVIDENCE,
        EXEC_PACKAGES,
        EXEC_TOOLS,
    ),
    node(
        "verify",
        NodeKind::Execution,
        "run focused verification through typed verification tools",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
    node(
        "escape",
        NodeKind::Execution,
        "escape hatch for unsupported operations after graph admission",
        NO_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
];
