use crate::model::{EvidenceRequirement, GraphNode, GraphNodeId, NodeKind};

const CORE_EVIDENCE: &[EvidenceRequirement] = &[
    EvidenceRequirement {
        name: "plan",
        description: "structured plan with constraints, risks, paths, and checks",
    },
    EvidenceRequirement {
        name: "observation",
        description: "at least one observed file, command, or tool result",
    },
];

const VERIFY_EVIDENCE: &[EvidenceRequirement] = &[EvidenceRequirement {
    name: "verification",
    description: "focused verification output or explicit not-run reason",
}];

const NO_EVIDENCE: &[EvidenceRequirement] = &[];

const PLANNING_PACKAGES: &[&str] = &["planning-checklist"];
const CONTEXT_PACKAGES: &[&str] = &["context-slice"];
const EXECUTION_PACKAGES: &[&str] = &["execution-order"];
const VERIFY_PACKAGES: &[&str] = &["verification-gate"];
const RECOVERY_PACKAGES: &[&str] = &["recovery-policy"];
const COMPACTION_PACKAGES: &[&str] = &["compaction-preserve"];
const DOC_PACKAGES: &[&str] = &["doc-construction"];
const MAINT_PACKAGES: &[&str] = &["maintenance-loop"];

const READ_ACTIONS: &[&str] = &["fs.read", "shell.run", "memory.find", "graph.state"];
const EXEC_ACTIONS: &[&str] = &["fs.read", "fs.write", "fs.edit", "shell.run", "memory.find"];
const VERIFY_ACTIONS: &[&str] = &["shell.run", "fs.read", "graph.evidence", "agent.done"];
const GRAPH_ACTIONS: &[&str] = &["graph.state", "graph.evidence", "agent.ask"];

pub(crate) const NODES: &[GraphNode] = &[
    node(
        "classify",
        NodeKind::Intent,
        "classify task intent",
        NO_EVIDENCE,
        PLANNING_PACKAGES,
        READ_ACTIONS,
    ),
    node(
        "plan",
        NodeKind::Planning,
        "build durable task plan",
        CORE_EVIDENCE,
        PLANNING_PACKAGES,
        READ_ACTIONS,
    ),
    node(
        "context",
        NodeKind::Context,
        "select context packages",
        NO_EVIDENCE,
        CONTEXT_PACKAGES,
        READ_ACTIONS,
    ),
    node(
        "execute",
        NodeKind::Execution,
        "act inside the plan",
        CORE_EVIDENCE,
        EXECUTION_PACKAGES,
        EXEC_ACTIONS,
    ),
    node(
        "verify",
        NodeKind::Execution,
        "verify observed behavior",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_ACTIONS,
    ),
    node(
        "recover",
        NodeKind::Recovery,
        "recover from a failed turn",
        NO_EVIDENCE,
        RECOVERY_PACKAGES,
        GRAPH_ACTIONS,
    ),
    node(
        "compact",
        NodeKind::Compaction,
        "preserve graph state",
        NO_EVIDENCE,
        COMPACTION_PACKAGES,
        GRAPH_ACTIONS,
    ),
    node(
        "complete",
        NodeKind::Completion,
        "close with evidence",
        VERIFY_EVIDENCE,
        VERIFY_PACKAGES,
        VERIFY_ACTIONS,
    ),
    node(
        "document",
        NodeKind::Document,
        "construct structured docs",
        CORE_EVIDENCE,
        DOC_PACKAGES,
        EXEC_ACTIONS,
    ),
    node(
        "memory",
        NodeKind::Memory,
        "manage durable memory",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        GRAPH_ACTIONS,
    ),
    node(
        "maintain",
        NodeKind::Maintenance,
        "improve graph and memory",
        NO_EVIDENCE,
        MAINT_PACKAGES,
        GRAPH_ACTIONS,
    ),
];

const fn node(
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
    }
}
