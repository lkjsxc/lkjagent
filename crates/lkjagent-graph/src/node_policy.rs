use crate::model::GraphNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoAdmission {
    Manual,
    Safe,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodePolicy {
    pub entry_conditions: &'static [&'static str],
    pub exit_conditions: &'static [&'static str],
    pub state_reads: &'static [&'static str],
    pub state_writes: &'static [&'static str],
    pub preferred_tools: &'static [&'static str],
    pub blocked_tools: &'static [&'static str],
    pub tool_demotions: &'static [&'static str],
    pub package_compression: &'static [&'static str],
    pub recovery_ladder: &'static [&'static str],
    pub completion_contribution: &'static str,
    pub maintenance_contribution: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgePolicy {
    pub priority: u16,
    pub auto_admission: AutoAdmission,
    pub state_delta: &'static str,
    pub recovery_target: Option<GraphNodeId>,
    pub compaction_behavior: &'static str,
}

pub const DEFAULT_NODE_POLICY: NodePolicy = NodePolicy {
    entry_conditions: &[],
    exit_conditions: &[],
    state_reads: &[],
    state_writes: &[],
    preferred_tools: &[],
    blocked_tools: &["shell.run", "agent.done"],
    tool_demotions: &["prefer native graph/fs/doc/verify tools before shell.run"],
    package_compression: &["yellow=narrow", "orange=checkpoint", "red=compact"],
    recovery_ladder: &[
        "correct-action-format",
        "inspect-graph-state",
        "reduce-scope",
        "alternate-native-tool",
        "admitted-shell-escape",
        "block-step-and-replan",
    ],
    completion_contribution: "none",
    maintenance_contribution: "none",
};

pub const DEFAULT_EDGE_POLICY: EdgePolicy = EdgePolicy {
    priority: 100,
    auto_admission: AutoAdmission::Manual,
    state_delta: "none",
    recovery_target: None,
    compaction_behavior: "preserve-case-state",
};
