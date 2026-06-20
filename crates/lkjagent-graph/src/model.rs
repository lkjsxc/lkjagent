#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphNodeId(pub &'static str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphEdgeId(pub &'static str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContextPackageId(pub &'static str);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Intent,
    Planning,
    State,
    Context,
    Execution,
    Verification,
    Document,
    Memory,
    Compaction,
    Recovery,
    Completion,
    Maintenance,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Start,
    Plan,
    SelectContext,
    Execute,
    Verify,
    Recover,
    Compact,
    Complete,
    Maintain,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFamily {
    CodeChange,
    Documentation,
    Maintenance,
    BugFix,
    Architecture,
    Benchmark,
    KnowledgeBase,
    Verification,
    Recovery,
    Compaction,
    IdleMaintenance,
}
impl TaskFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CodeChange => "code-change",
            Self::Documentation => "documentation",
            Self::Maintenance => "maintenance",
            Self::BugFix => "bug-fix",
            Self::Architecture => "architecture",
            Self::Benchmark => "benchmark",
            Self::KnowledgeBase => "knowledge-base",
            Self::Verification => "verification",
            Self::Recovery => "recovery",
            Self::Compaction => "compaction",
            Self::IdleMaintenance => "idle-maintenance",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPhase {
    Intake,
    Planning,
    Context,
    Execution,
    Verification,
    Recovery,
    Compaction,
    Completion,
    Maintenance,
    Waiting,
    Closed,
}

impl TaskPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Intake => "intake",
            Self::Planning => "planning",
            Self::Context => "context",
            Self::Execution => "execution",
            Self::Verification => "verification",
            Self::Recovery => "recovery",
            Self::Compaction => "compaction",
            Self::Completion => "completion",
            Self::Maintenance => "maintenance",
            Self::Waiting => "waiting",
            Self::Closed => "closed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseStatus {
    Active,
    Waiting,
    Closed,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    Owner,
    Plan,
    Action,
    Observation,
    Verification,
    File,
    Memory,
    Note,
}

impl EvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Plan => "plan",
            Self::Action => "action",
            Self::Observation => "observation",
            Self::Verification => "verification",
            Self::File => "file",
            Self::Memory => "memory",
            Self::Note => "note",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub requirement: String,
    pub kind: EvidenceKind,
    pub summary: String,
    pub path: Option<String>,
    pub frame_ref: Option<usize>,
    pub event_ref: Option<i64>,
    pub confidence: u8,
    pub satisfies_completion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceRequirement {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextPackage {
    pub id: ContextPackageId,
    pub title: &'static str,
    pub purpose: &'static str,
    pub body: &'static str,
    pub default_budget: usize,
    pub priority: PackagePriority,
    pub applies_to: &'static [GraphNodeId],
    pub families: &'static [TaskFamily],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackagePriority {
    Core,
    Helpful,
    Recovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphNode {
    pub id: GraphNodeId,
    pub kind: NodeKind,
    pub label: &'static str,
    pub instructions: &'static str,
    pub evidence: &'static [EvidenceRequirement],
    pub packages: &'static [&'static str],
    pub allowed_actions: &'static [&'static str],
    pub policy: crate::node_policy::NodePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphEdge {
    pub id: GraphEdgeId,
    pub kind: EdgeKind,
    pub from: GraphNodeId,
    pub to: GraphNodeId,
    pub guards: &'static [crate::guards::Guard],
    pub rationale: &'static str,
    pub policy: crate::node_policy::EdgePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDefinition {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub packages: Vec<ContextPackage>,
    pub policy: crate::policy::GraphPolicy,
}
