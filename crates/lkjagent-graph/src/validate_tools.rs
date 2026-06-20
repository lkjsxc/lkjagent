use crate::model::GraphDefinition;

pub(crate) fn collect_tools(graph: &GraphDefinition, violations: &mut Vec<String>) {
    for node in &graph.nodes {
        for tool in node.allowed_actions {
            if !KNOWN_TOOLS.contains(tool) {
                violations.push(format!("node {} unknown tool {}", node.id.0, tool));
            }
        }
    }
}

const KNOWN_TOOLS: &[&str] = &[
    "fs.read",
    "fs.read_many",
    "fs.write",
    "fs.edit",
    "fs.patch",
    "fs.list",
    "fs.tree",
    "fs.search",
    "fs.stat",
    "fs.mkdir",
    "fs.batch_write",
    "shell.run",
    "queue.list",
    "queue.enqueue",
    "queue.edit",
    "queue.delete",
    "queue.redeliver",
    "memory.save",
    "memory.find",
    "graph.state",
    "graph.next",
    "graph.audit",
    "graph.recover",
    "graph.plan",
    "graph.transition",
    "graph.context",
    "graph.note",
    "graph.evidence",
    "graph.compact",
    "workspace.summary",
    "workspace.index",
    "verify.cargo",
    "verify.xtask",
    "doc.scaffold",
    "doc.audit",
    "agent.done",
    "agent.ask",
];
