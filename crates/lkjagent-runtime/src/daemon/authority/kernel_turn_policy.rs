use crate::kernel::{RuntimeDecision, ToolName};
use crate::mode::{ActiveMode, ActiveModePolicy};

pub(super) fn policy_from_decision(
    mode: ActiveMode,
    decision: &RuntimeDecision,
) -> ActiveModePolicy {
    ActiveModePolicy {
        mode,
        allowed_tools: static_tools(&decision.admission_view.admitted_tools),
        blocked_tools: static_tools(&decision.admission_view.blocked_tools),
        preferred_next_action: static_next_action(decision),
        completion_condition: completion_condition(decision),
        completion_allowed: decision.completion_allowed || mode == ActiveMode::Maintenance,
        graph_policy_applies: false,
        maintenance_policy_applies: mode == ActiveMode::Maintenance,
        compaction_policy_applies: mode == ActiveMode::Compaction,
    }
}

fn completion_condition(decision: &RuntimeDecision) -> &'static str {
    if decision.completion_allowed {
        "kernel completion admitted"
    } else {
        "kernel completion gate pending"
    }
}

fn static_next_action(decision: &RuntimeDecision) -> &'static str {
    static_tools(&decision.admission_view.admitted_tools)
        .first()
        .copied()
        .unwrap_or("runtime effect")
}

fn static_tools(values: &[ToolName]) -> Vec<&'static str> {
    values
        .iter()
        .filter_map(|tool| static_tool(tool.as_str()))
        .collect()
}

fn static_tool(value: &str) -> Option<&'static str> {
    match value {
        "agent.ask" => Some("agent.ask"),
        "agent.done" => Some("agent.done"),
        "artifact.audit" => Some("artifact.audit"),
        "artifact.next" => Some("artifact.next"),
        "doc.audit" => Some("doc.audit"),
        "fs.batch_write" => Some("fs.batch_write"),
        "fs.list" => Some("fs.list"),
        "fs.read" => Some("fs.read"),
        "fs.read_many" => Some("fs.read_many"),
        "fs.stat" => Some("fs.stat"),
        "fs.tree" => Some("fs.tree"),
        "fs.write" => Some("fs.write"),
        "graph.evidence" => Some("graph.evidence"),
        "graph.next" => Some("graph.next"),
        "graph.note" => Some("graph.note"),
        "graph.plan" => Some("graph.plan"),
        "graph.recover" => Some("graph.recover"),
        "graph.state" => Some("graph.state"),
        "graph.transition" => Some("graph.transition"),
        "memory.find" => Some("memory.find"),
        "memory.prune" => Some("memory.prune"),
        "memory.save" => Some("memory.save"),
        "queue.list" => Some("queue.list"),
        "shell.run" => Some("shell.run"),
        "verify.cargo" => Some("verify.cargo"),
        "verify.xtask" => Some("verify.xtask"),
        "workspace.summary" => Some("workspace.summary"),
        _ => None,
    }
}
