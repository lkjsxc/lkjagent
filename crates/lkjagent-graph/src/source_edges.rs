use crate::guards::Guard;
use crate::model::{EdgeKind, GraphEdge, GraphEdgeId, GraphNodeId, TaskFamily};
use crate::policy::ContextPressureLevel;

const ALWAYS: &[Guard] = &[Guard::Always];
const PLAN_READY: &[Guard] = &[Guard::PlanReady];
const PLAN_CONTEXT: &[Guard] = &[Guard::PlanReady, Guard::ContextSelected];
const OBSERVED: &[Guard] = &[Guard::HasEvidence("observation")];
const VERIFIED: &[Guard] = &[Guard::CompletionReady];
const DOCUMENT_FAMILIES: &[TaskFamily] = &[TaskFamily::Documentation, TaskFamily::KnowledgeBase];
const DOC_FAMILY: &[Guard] = &[Guard::PlanReady, Guard::FamilyIn(DOCUMENT_FAMILIES)];
const DOC_AUDIT: &[Guard] = &[Guard::DocumentAuditReady];
const SOFT_PRESSURE: &[Guard] = &[Guard::ContextPressureAtLeast(ContextPressureLevel::Orange)];
const HARD_PRESSURE: &[Guard] = &[Guard::ContextPressureAtLeast(ContextPressureLevel::Red)];
const TOOL_FAULT: &[Guard] = &[Guard::FaultCountAtLeast(
    crate::case_recovery::FaultKind::Tool,
    1,
)];
const REPEAT_FAULT: &[Guard] = &[Guard::FaultCountAtLeast(
    crate::case_recovery::FaultKind::Repeat,
    1,
)];

#[rustfmt::skip]
pub(crate) const EDGES: &[GraphEdge] = &[
    edge("intake-normalize", EdgeKind::Start, "intake", "normalize-objective", ALWAYS),
    edge("normalize-constraints", EdgeKind::Plan, "normalize-objective", "extract-constraints", ALWAYS),
    edge("constraints-route", EdgeKind::Plan, "extract-constraints", "route", ALWAYS),
    edge("route-survey", EdgeKind::SelectContext, "route", "survey", ALWAYS),
    edge("survey-context", EdgeKind::SelectContext, "survey", "context", ALWAYS),
    edge("context-plan", EdgeKind::Plan, "context", "plan", ALWAYS),
    edge("plan-review", EdgeKind::Plan, "plan", "review-plan", PLAN_READY),
    edge("review-execute", EdgeKind::Execute, "review-plan", "execute", PLAN_CONTEXT),
    edge("plan-document", EdgeKind::Plan, "plan", "document", DOC_FAMILY),
    edge("document-audit", EdgeKind::Verify, "document", "document-audit", OBSERVED),
    edge("document-verify", EdgeKind::Verify, "document-audit", "verify", DOC_AUDIT),
    edge("execute-observe", EdgeKind::Execute, "execute", "observe", OBSERVED),
    edge("observe-integrate", EdgeKind::Execute, "observe", "integrate-evidence", OBSERVED),
    edge("integrate-verify", EdgeKind::Verify, "integrate-evidence", "verify", OBSERVED),
    edge("verify-complete", EdgeKind::Complete, "verify", "complete", VERIFIED),
    edge("execute-recover", EdgeKind::Recover, "execute", "recover-tool", TOOL_FAULT),
    edge("recover-tool-plan", EdgeKind::Plan, "recover-tool", "plan", ALWAYS),
    edge("recover-repeat-plan", EdgeKind::Plan, "recover-repeat", "plan", ALWAYS),
    edge("execute-repeat", EdgeKind::Recover, "execute", "recover-repeat", REPEAT_FAULT),
    edge("execute-soft-compact", EdgeKind::Compact, "execute", "compact-soft", SOFT_PRESSURE),
    edge("verify-hard-compact", EdgeKind::Compact, "verify", "compact-hard", HARD_PRESSURE),
    edge("compact-rebuild", EdgeKind::SelectContext, "compact-soft", "rebuild-context", ALWAYS),
    edge("hard-compact-rebuild", EdgeKind::SelectContext, "compact-hard", "rebuild-context", ALWAYS),
    edge("rebuild-context", EdgeKind::SelectContext, "rebuild-context", "context", ALWAYS),
    edge("maintain-policy", EdgeKind::Maintain, "maintain", "refine-graph-policy", ALWAYS),
    edge("maintain-prune", EdgeKind::Maintain, "maintain", "prune-memory", ALWAYS),
    edge("maintain-audit", EdgeKind::Maintain, "maintain", "audit-self", ALWAYS),
    edge("policy-complete", EdgeKind::Complete, "refine-graph-policy", "complete", ALWAYS),
];

const fn edge(
    id: &'static str,
    kind: EdgeKind,
    from: &'static str,
    to: &'static str,
    guards: &'static [Guard],
) -> GraphEdge {
    GraphEdge {
        id: GraphEdgeId(id),
        kind,
        from: GraphNodeId(from),
        to: GraphNodeId(to),
        guards,
        rationale: id,
    }
}
