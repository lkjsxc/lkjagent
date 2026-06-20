use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, NO_EVIDENCE, PLAN_PACKAGES, PLAN_TOOLS};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "extract-non-goals",
        NodeKind::State,
        "separate explicit non-goals from the objective",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "extract-risks",
        NodeKind::State,
        "name task risks and mitigations before execution",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "extract-success-criteria",
        NodeKind::State,
        "extract completion criteria and evidence needs",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "route-family",
        NodeKind::Intent,
        "classify family, subroute, confidence, and route reason",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "detect-blockers",
        NodeKind::Recovery,
        "ask only if all legal graph progress is blocked",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
];
