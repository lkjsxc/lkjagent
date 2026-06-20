use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, COMPLETE_TOOLS, MAINT_PACKAGES, NO_EVIDENCE};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "completion-audit",
        NodeKind::Completion,
        "check completion gates and pending faults",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        COMPLETE_TOOLS,
    ),
    node(
        "completion-evidence",
        NodeKind::Completion,
        "render concise evidence summary for closure",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        COMPLETE_TOOLS,
    ),
    node(
        "completion-memory",
        NodeKind::Memory,
        "save useful graph-linked task memory",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        COMPLETE_TOOLS,
    ),
];
