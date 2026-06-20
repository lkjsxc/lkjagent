use crate::model::{ContextPackage, ContextPackageId, GraphNodeId, PackagePriority, TaskFamily};

const PLAN_NODES: &[GraphNodeId] = &[
    GraphNodeId("intake"),
    GraphNodeId("normalize-objective"),
    GraphNodeId("extract-constraints"),
    GraphNodeId("plan"),
    GraphNodeId("review-plan"),
];
const EXEC_NODES: &[GraphNodeId] = &[GraphNodeId("execute"), GraphNodeId("observe")];
const VERIFY_NODES: &[GraphNodeId] = &[GraphNodeId("verify"), GraphNodeId("complete")];
const DOC_NODES: &[GraphNodeId] = &[GraphNodeId("document"), GraphNodeId("document-audit")];
const RECOVERY_NODES: &[GraphNodeId] = &[GraphNodeId("recover"), GraphNodeId("recover-tool")];
const MAINT_NODES: &[GraphNodeId] = &[GraphNodeId("maintain"), GraphNodeId("refine-graph-policy")];
const ALL_FAMILIES: &[TaskFamily] = &[
    TaskFamily::CodeChange,
    TaskFamily::Documentation,
    TaskFamily::Maintenance,
    TaskFamily::BugFix,
    TaskFamily::Architecture,
    TaskFamily::Benchmark,
    TaskFamily::KnowledgeBase,
    TaskFamily::Verification,
    TaskFamily::Recovery,
    TaskFamily::Compaction,
    TaskFamily::IdleMaintenance,
];
const DOC_FAMILIES: &[TaskFamily] = &[TaskFamily::Documentation, TaskFamily::KnowledgeBase];

pub(crate) const PACKAGES: &[ContextPackage] = &[
    package(
        "planning-checklist",
        "Planning checklist",
        "Before mutation, normalize objective, constraints, assumptions, risks, steps, evidence, checks, and paths.",
        PLAN_NODES,
        ALL_FAMILIES,
        PackagePriority::Core,
    ),
    package(
        "context-slice",
        "Context slice",
        "Load only candidate files, relevant docs, memory hits, and workspace summary selected by the active graph node.",
        PLAN_NODES,
        ALL_FAMILIES,
        PackagePriority::Core,
    ),
    package(
        "execution-order",
        "Execution order",
        "Execute one active step, observe, bind evidence, update touched paths, then advance or recover.",
        EXEC_NODES,
        ALL_FAMILIES,
        PackagePriority::Helpful,
    ),
    package(
        "verification-gate",
        "Verification gate",
        "Completion needs plan, observation, verification when code changed, pending checks clear, and agent.done admission.",
        VERIFY_NODES,
        ALL_FAMILIES,
        PackagePriority::Core,
    ),
    package(
        "doc-construction",
        "Document construction",
        "Use doc.scaffold/doc.audit or bounded batch writes; keep README indexes, topology, coverage, and counts auditable.",
        DOC_NODES,
        DOC_FAMILIES,
        PackagePriority::Core,
    ),
    package(
        "recovery-policy",
        "Recovery policy",
        "Do not repeat failed action fingerprints; choose a different tool class and record the safer branch.",
        RECOVERY_NODES,
        ALL_FAMILIES,
        PackagePriority::Recovery,
    ),
    package(
        "compaction-preserve",
        "Compaction preservation",
        "Preserve objective, constraints, plan steps, selected packages, evidence, touched paths, recovery, and completion guard.",
        RECOVERY_NODES,
        ALL_FAMILIES,
        PackagePriority::Recovery,
    ),
    package(
        "maintenance-loop",
        "Maintenance loop",
        "Idle work records graph policy, context package, memory, and audit candidates without editing source files.",
        MAINT_NODES,
        ALL_FAMILIES,
        PackagePriority::Helpful,
    ),
];

const fn package(
    id: &'static str,
    title: &'static str,
    body: &'static str,
    applies_to: &'static [GraphNodeId],
    families: &'static [TaskFamily],
    priority: PackagePriority,
) -> ContextPackage {
    ContextPackage {
        id: ContextPackageId(id),
        title,
        purpose: title,
        body,
        default_budget: 160,
        priority,
        applies_to,
        families,
    }
}
