use super::model::{ActiveMode, ActiveModePolicy};

pub fn policy_for_mode(mode: ActiveMode) -> ActiveModePolicy {
    match mode {
        ActiveMode::OwnerTask => ActiveModePolicy {
            mode,
            allowed_tools: Vec::new(),
            blocked_tools: Vec::new(),
            preferred_next_action: "follow active graph policy",
            completion_condition: "graph completion gate admits done",
            graph_policy_applies: true,
            maintenance_policy_applies: false,
            compaction_policy_applies: false,
        },
        ActiveMode::Recovery => ActiveModePolicy {
            mode,
            allowed_tools: Vec::new(),
            blocked_tools: Vec::new(),
            preferred_next_action: "follow active recovery node",
            completion_condition: "fault route resolved or blocked handoff recorded",
            graph_policy_applies: true,
            maintenance_policy_applies: false,
            compaction_policy_applies: false,
        },
        ActiveMode::Maintenance => maintenance_policy(mode),
        ActiveMode::Compaction => ActiveModePolicy {
            mode,
            allowed_tools: Vec::new(),
            blocked_tools: vec!["memory.save"],
            preferred_next_action: "runtime-owned compaction snapshot",
            completion_condition: "snapshot applied without model memory action",
            graph_policy_applies: false,
            maintenance_policy_applies: false,
            compaction_policy_applies: true,
        },
        ActiveMode::ClosedIdle => ActiveModePolicy {
            mode,
            allowed_tools: Vec::new(),
            blocked_tools: Vec::new(),
            preferred_next_action: "wait for queue row or due maintenance",
            completion_condition: "no endpoint turn required",
            graph_policy_applies: false,
            maintenance_policy_applies: false,
            compaction_policy_applies: false,
        },
    }
}

fn maintenance_policy(mode: ActiveMode) -> ActiveModePolicy {
    ActiveModePolicy {
        mode,
        allowed_tools: vec![
            "memory.find",
            "memory.save",
            "queue.list",
            "agent.done",
            "agent.ask",
        ],
        blocked_tools: vec![
            "graph.state",
            "graph.next",
            "graph.plan",
            "graph.transition",
            "doc.scaffold",
            "fs.write",
            "shell.run",
        ],
        preferred_next_action: "bounded maintenance bookkeeping or agent.done",
        completion_condition: "one real maintenance effect or no-op close",
        graph_policy_applies: false,
        maintenance_policy_applies: true,
        compaction_policy_applies: false,
    }
}
