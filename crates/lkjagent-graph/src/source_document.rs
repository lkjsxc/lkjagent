use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, DOC_EVIDENCE, DOC_PACKAGES, DOC_TOOLS};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "document-profile",
        NodeKind::Document,
        "profile document kind, root, count, and language",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "document-topology",
        NodeKind::Document,
        "define README-indexed topology before writing",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "document-scaffold",
        NodeKind::Document,
        "create bounded scaffold and root indexes",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "document-section-plan",
        NodeKind::Document,
        "map sections and coverage before bulk content",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "document-write",
        NodeKind::Document,
        "write bounded document batches",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "document-repair",
        NodeKind::Document,
        "repair topology, links, counts, or coverage",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
    node(
        "document-completion-check",
        NodeKind::Document,
        "prove document audit evidence before closure",
        DOC_EVIDENCE,
        DOC_PACKAGES,
        DOC_TOOLS,
    ),
];
