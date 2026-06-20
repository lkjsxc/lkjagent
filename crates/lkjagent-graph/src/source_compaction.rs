use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, COMPACT_PACKAGES, NO_EVIDENCE, RECOVERY_TOOLS};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "compact-boundary",
        NodeKind::Compaction,
        "compact at a phase boundary before more endpoint calls",
        NO_EVIDENCE,
        COMPACT_PACKAGES,
        RECOVERY_TOOLS,
    ),
    node(
        "resume-after-compact",
        NodeKind::Context,
        "resume active node from structured snapshot",
        NO_EVIDENCE,
        COMPACT_PACKAGES,
        RECOVERY_TOOLS,
    ),
];
