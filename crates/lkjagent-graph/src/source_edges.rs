use crate::model::{EdgeKind, GraphEdge, GraphEdgeId, GraphNodeId};

pub(crate) const EDGES: &[GraphEdge] = &[
    edge(
        "classify-plan",
        EdgeKind::Start,
        "classify",
        "plan",
        "owner message delivered",
    ),
    edge(
        "plan-context",
        EdgeKind::Plan,
        "plan",
        "context",
        "plan evidence exists",
    ),
    edge(
        "context-execute",
        EdgeKind::SelectContext,
        "context",
        "execute",
        "context packages selected",
    ),
    edge(
        "execute-verify",
        EdgeKind::Verify,
        "execute",
        "verify",
        "action observation exists",
    ),
    edge(
        "verify-complete",
        EdgeKind::Complete,
        "verify",
        "complete",
        "required evidence exists",
    ),
    edge(
        "execute-recover",
        EdgeKind::Recover,
        "execute",
        "recover",
        "tool, parse, endpoint, or repeat failure",
    ),
    edge(
        "recover-plan",
        EdgeKind::Plan,
        "recover",
        "plan",
        "recovery strategy recorded",
    ),
    edge(
        "execute-compact",
        EdgeKind::Compact,
        "execute",
        "compact",
        "orange or red context pressure",
    ),
    edge(
        "compact-context",
        EdgeKind::SelectContext,
        "compact",
        "context",
        "compaction plan rendered",
    ),
    edge(
        "maintain-plan",
        EdgeKind::Maintain,
        "maintain",
        "plan",
        "maintenance case opened",
    ),
    edge(
        "plan-document",
        EdgeKind::Plan,
        "plan",
        "document",
        "documentation family",
    ),
    edge(
        "document-verify",
        EdgeKind::Verify,
        "document",
        "verify",
        "document artifacts observed",
    ),
];

const fn edge(
    id: &'static str,
    kind: EdgeKind,
    from: &'static str,
    to: &'static str,
    guard: &'static str,
) -> GraphEdge {
    GraphEdge {
        id: GraphEdgeId(id),
        kind,
        from: GraphNodeId(from),
        to: GraphNodeId(to),
        guard,
    }
}
