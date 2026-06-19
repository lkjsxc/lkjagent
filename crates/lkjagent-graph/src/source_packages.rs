use crate::model::ContextPackage;

pub(crate) const PACKAGES: &[ContextPackage] = &[
    package(
        "planning-checklist",
        "objective; constraints; assumptions; risks; evidence; next action",
    ),
    package(
        "context-slice",
        "load only graph-selected facts, docs, memory, and workspace brief",
    ),
    package(
        "execution-order",
        "inspect, plan, edit, verify, then summarize with evidence",
    ),
    package(
        "verification-gate",
        "agent.done requires required evidence and pending checks resolved",
    ),
    package(
        "recovery-policy",
        "narrow after faults; record recovery evidence; do not repeat actions",
    ),
    package(
        "compaction-preserve",
        "preserve case, phase, node, plan, evidence, paths, and packages",
    ),
    package(
        "doc-construction",
        "build README-indexed trees, nuclei, maps, queues, and rebalance plans",
    ),
    package(
        "maintenance-loop",
        "distill, improve-graph, prune-memory, audit-self",
    ),
];

const fn package(name: &'static str, body: &'static str) -> ContextPackage {
    ContextPackage {
        name,
        budget: 160,
        body,
    }
}
