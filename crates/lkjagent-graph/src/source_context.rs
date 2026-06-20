use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, CONTEXT_PACKAGES, CONTEXT_TOOLS, NO_EVIDENCE};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "workspace-survey",
        NodeKind::Context,
        "map workspace shape with bounded native tools",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
    node(
        "candidate-paths",
        NodeKind::Context,
        "select likely files, dirs, and manifests",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
    node(
        "memory-retrieval",
        NodeKind::Memory,
        "retrieve graph-linked memory for this case and node",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
    node(
        "context-select",
        NodeKind::Context,
        "bind graph packages and compression levels",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
    node(
        "context-review",
        NodeKind::Context,
        "check selected context against missing evidence",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
    node(
        "context-refresh",
        NodeKind::Context,
        "refresh stale paths after faults or compaction",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
];
