use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{node, PLAN_EVIDENCE, PLAN_PACKAGES, PLAN_TOOLS};

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "decompose-task",
        NodeKind::Planning,
        "split objective into small evidence-bearing steps",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "draft-plan",
        NodeKind::Planning,
        "draft graph.plan with checks, paths, and rationale",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "risk-review",
        NodeKind::Planning,
        "review plan risks, invariants, and non-goals",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "evidence-plan",
        NodeKind::Planning,
        "attach evidence requirements to each step",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "verification-plan",
        NodeKind::Planning,
        "choose direct verification tools before execution",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
    node(
        "choose-active-step",
        NodeKind::Planning,
        "select the next unblocked executable step",
        PLAN_EVIDENCE,
        PLAN_PACKAGES,
        PLAN_TOOLS,
    ),
];
