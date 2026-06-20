use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{
    node, CONTEXT_PACKAGES, CONTEXT_TOOLS, NO_EVIDENCE, PLAN_EVIDENCE, PLAN_PACKAGES, PLAN_TOOLS,
};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "intake",
        NodeKind::Intent,
        "receive owner message as input, not a plan",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "normalize-objective",
        NodeKind::Intent,
        "normalize objective and extract non-goals",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "extract-constraints",
        NodeKind::State,
        "extract constraints, assumptions, questions, risks, invariants",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "route",
        NodeKind::Intent,
        "score task families and select the primary route",
        NO_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "survey",
        NodeKind::Context,
        "inspect workspace shape and candidate paths",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
    node(
        "context",
        NodeKind::Context,
        "select graph context packages and bounded context",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        CONTEXT_TOOLS,
    ),
    node(
        "plan",
        NodeKind::Planning,
        "synthesize structured plan before mutation",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "review-plan",
        NodeKind::Planning,
        "review plan against constraints and evidence requirements",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
];
