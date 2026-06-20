use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, VERIFY_EVIDENCE, VERIFY_PACKAGES, VERIFY_TOOLS};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "verify-focused",
        NodeKind::Verification,
        "run the smallest check that proves the claim",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
    node(
        "verify-style",
        NodeKind::Verification,
        "run formatting, line, and style gates when relevant",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
    node(
        "verify-docs",
        NodeKind::Verification,
        "run documentation consistency gates",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
    node(
        "verify-integration",
        NodeKind::Verification,
        "run integration or full quiet gates when required",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
    node(
        "verify-final",
        NodeKind::Completion,
        "collect final evidence before completion audit",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_TOOLS,
    ),
];
