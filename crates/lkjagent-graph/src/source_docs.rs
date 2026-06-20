use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{
    node, DOC_EVIDENCE, DOC_PACKAGES, DOC_TOOLS, NO_EVIDENCE, VERIFY_EVIDENCE, VERIFY_PACKAGES,
    VERIFY_TOOLS,
};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "document",
        NodeKind::Document,
        "construct document topology using doc tools",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "document-audit",
        NodeKind::Document,
        "audit README index, counts, links, and coverage",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "benchmark",
        NodeKind::Execution,
        "run benchmark or evaluation checks through typed gates",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
    node(
        "docs-code-consistency",
        NodeKind::Completion,
        "verify docs and code agree before closure",
        NO_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
];
