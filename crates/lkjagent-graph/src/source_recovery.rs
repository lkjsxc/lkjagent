use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{
    node, COMPACT_PACKAGES, NO_EVIDENCE, PLAN_TOOLS, RECOVERY_PACKAGES, RECOVERY_TOOLS,
};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "recover",
        NodeKind::Recovery,
        "choose a different safe action after a fault",
        NO_EVIDENCE,
        RECOVERY_PACKAGES,
        RECOVERY_TOOLS,
    ),
    node(
        "recover-tool",
        NodeKind::Recovery,
        "recover from tool or shell failure without repeating",
        NO_EVIDENCE,
        RECOVERY_PACKAGES,
        RECOVERY_TOOLS,
    ),
    node(
        "recover-repeat",
        NodeKind::Recovery,
        "break repeated action loop with a different action class",
        NO_EVIDENCE,
        RECOVERY_PACKAGES,
        RECOVERY_TOOLS,
    ),
    node(
        "owner-question",
        NodeKind::Recovery,
        "ask only when a blocking owner question remains",
        NO_EVIDENCE,
        RECOVERY_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "compact-soft",
        NodeKind::Compaction,
        "preserve snapshot under soft context pressure",
        NO_EVIDENCE,
        COMPACT_PACKAGES,
        RECOVERY_TOOLS,
    ),
    node(
        "compact-hard",
        NodeKind::Compaction,
        "preserve snapshot under hard context pressure before more action",
        NO_EVIDENCE,
        COMPACT_PACKAGES,
        RECOVERY_TOOLS,
    ),
    node(
        "rebuild-context",
        NodeKind::Context,
        "rebuild bounded context after compaction",
        NO_EVIDENCE,
        COMPACT_PACKAGES,
        RECOVERY_TOOLS,
    ),
];
