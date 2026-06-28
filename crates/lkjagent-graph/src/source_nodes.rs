use crate::model::{EvidenceRequirement, GraphNode, GraphNodeId, NodeKind};

pub(crate) const PLAN_EVIDENCE: &[EvidenceRequirement] = &[EvidenceRequirement {
    name: "plan",
    description: "structured graph plan with steps, evidence, paths, and checks",
}];

pub(crate) const OBSERVE_EVIDENCE: &[EvidenceRequirement] = &[EvidenceRequirement {
    name: "observation",
    description: "bounded tool observation linked to a plan step",
}];

pub(crate) const VERIFY_EVIDENCE: &[EvidenceRequirement] = &[EvidenceRequirement {
    name: "verification",
    description: "real verification observation or accepted not-run reason",
}];

pub(crate) const DOC_EVIDENCE: &[EvidenceRequirement] = &[EvidenceRequirement {
    name: "document-structure",
    description: "document topology and audit evidence",
}];

pub(crate) const NO_EVIDENCE: &[EvidenceRequirement] = &[];
pub(crate) const PLAN_PACKAGES: &[&str] = &["planning-checklist", "context-slice"];
pub(crate) const CONTEXT_PACKAGES: &[&str] = &["context-slice"];
pub(crate) const EXEC_PACKAGES: &[&str] = &["execution-order"];
pub(crate) const VERIFY_PACKAGES: &[&str] = &["verification-gate"];
pub(crate) const DOC_PACKAGES: &[&str] = &["doc-construction"];
pub(crate) const RECOVERY_PACKAGES: &[&str] = &["recovery-policy"];
pub(crate) const COMPACT_PACKAGES: &[&str] = &["compaction-preserve"];
pub(crate) const MAINT_PACKAGES: &[&str] = &["maintenance-loop"];

pub(crate) const PLAN_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.plan",
    "graph.note",
    "graph.context",
    "graph.transition",
    "fs.list",
    "fs.tree",
    "fs.search",
    "fs.stat",
    "fs.read",
    "fs.read_many",
    "workspace.summary",
    "workspace.index",
    "memory.find",
    "agent.ask",
];
pub(crate) const CONTEXT_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.context",
    "graph.transition",
    "fs.list",
    "fs.tree",
    "fs.search",
    "fs.stat",
    "fs.read",
    "fs.read_many",
    "workspace.summary",
    "workspace.index",
    "memory.find",
];
pub(crate) const EXEC_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.evidence",
    "graph.note",
    "graph.transition",
    "fs.read",
    "fs.read_many",
    "fs.write",
    "fs.edit",
    "fs.patch",
    "fs.batch_write",
    "fs.mkdir",
    "workspace.summary",
    "workspace.index",
];
pub(crate) const VERIFY_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.evidence",
    "graph.transition",
    "fs.read",
    "fs.read_many",
    "fs.list",
    "fs.tree",
    "fs.search",
    "fs.stat",
    "verify.cargo",
    "verify.xtask",
    "doc.audit",
];
pub(crate) const DOC_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.evidence",
    "graph.note",
    "graph.transition",
    "fs.list",
    "fs.tree",
    "fs.stat",
    "artifact.plan",
    "artifact.audit",
    "artifact.next",
    "fs.write",
    "doc.audit",
    "fs.batch_write",
    "fs.mkdir",
];
pub(crate) const RECOVERY_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.recover",
    "graph.note",
    "graph.transition",
    "fs.list",
    "fs.tree",
    "fs.search",
    "fs.stat",
    "fs.read",
    "fs.read_many",
    "workspace.index",
    "verify.xtask",
    "agent.ask",
];
pub(crate) const PARAM_RECOVERY_TOOLS: &[&str] =
    &["graph.state", "fs.list", "workspace.summary", "agent.ask"];
pub(crate) const SHELL_ESCAPE_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.recover",
    "graph.note",
    "graph.transition",
    "fs.read",
    "fs.read_many",
    "workspace.index",
    "shell.run",
    "agent.ask",
];
pub(crate) const COMPLETE_TOOLS: &[&str] =
    &["graph.state", "graph.next", "graph.audit", "agent.done"];
pub(crate) const MAINT_TOOLS: &[&str] = &[
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.note",
    "memory.find",
    "memory.save",
    "queue.list",
    "agent.done",
];

pub(crate) const fn node(
    id: &'static str,
    kind: NodeKind,
    label: &'static str,
    evidence: &'static [EvidenceRequirement],
    packages: &'static [&'static str],
    actions: &'static [&'static str],
) -> GraphNode {
    GraphNode {
        id: GraphNodeId(id),
        kind,
        label,
        instructions: label,
        evidence,
        packages,
        allowed_actions: actions,
        policy: crate::node_policy::DEFAULT_NODE_POLICY,
    }
}
