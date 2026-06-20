use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, COMPLETE_TOOLS, MAINT_PACKAGES, MAINT_TOOLS, NO_EVIDENCE};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "complete",
        NodeKind::Completion,
        "close only when completion guard is ready",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        COMPLETE_TOOLS,
    ),
    node(
        "memory",
        NodeKind::Memory,
        "link durable memory rows to graph cases",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        MAINT_TOOLS,
    ),
    node(
        "maintain",
        NodeKind::Maintenance,
        "bounded idle graph and memory maintenance",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        MAINT_TOOLS,
    ),
    node(
        "refine-graph-policy",
        NodeKind::Maintenance,
        "record graph policy and context package improvement candidates",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        MAINT_TOOLS,
    ),
    node(
        "prune-memory",
        NodeKind::Maintenance,
        "merge, correct, or retire stale memory rows",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        MAINT_TOOLS,
    ),
    node(
        "audit-self",
        NodeKind::Maintenance,
        "record mismatches between docs, code, graph, and tests",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        MAINT_TOOLS,
    ),
];
