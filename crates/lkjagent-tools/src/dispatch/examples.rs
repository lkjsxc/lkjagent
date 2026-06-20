use lkjagent_protocol::registry::{find_tool, ParamSpec};
use lkjagent_protocol::{render_action, Action, Param};

pub fn registry_valid_example(tool: &str) -> Option<String> {
    find_tool(tool).map(|spec| valid_example(tool, spec.params))
}

pub fn valid_example(tool: &str, specs: &[ParamSpec]) -> String {
    let params = specs
        .iter()
        .filter(|spec| spec.required)
        .map(|spec| Param::new(spec.name, example_value(tool, spec.name)))
        .collect::<Vec<_>>();
    render_action(&Action::new(tool, params))
}

fn example_value(tool: &str, name: &str) -> &'static str {
    match (tool, name) {
        ("doc.scaffold", "root") | ("doc.audit", "root") => "docs",
        ("doc.scaffold", "title") => "Project Documentation",
        ("fs.read", "path") | ("fs.stat", "path") | ("fs.mkdir", "path") => "README.md",
        ("fs.write", "path") | ("fs.edit", "path") | ("fs.patch", "path") => "README.md",
        ("fs.read_many", "paths") => "README.md",
        ("fs.write", "content") => "Example content",
        ("fs.edit", "find") => "Example",
        ("fs.edit", "replace") => "Updated",
        ("fs.patch", "patch") => "*** Begin Patch\n*** End Patch",
        ("fs.batch_write", "files") => "docs/example.md|# Example",
        ("fs.search", "query") | ("memory.find", "query") => "README",
        ("graph.evidence", "kind") => "observation",
        ("graph.note", "kind") => "decision",
        ("graph.evidence", "summary") => "Read README.md",
        ("graph.note", "summary") => "Chose smaller recovery action",
        ("graph.plan", "objective") => "Complete the owner task",
        ("graph.plan", "steps") => "Inspect state\nAct in one bounded step",
        ("graph.plan", "reason") => "Graph planning requirement",
        ("graph.transition", "target") => "plan",
        ("graph.transition", "reason") => "Best legal next node",
        ("graph.context", "packages") => "planning-checklist",
        ("graph.context", "reason") => "Need planning context",
        ("graph.compact", "reason") => "Context pressure",
        ("memory.save", "kind") => "note",
        ("memory.save", "title") => "Useful lesson",
        ("memory.save", "content") => "Record only observed durable facts.",
        ("agent.done", "summary") => "Completed with evidence",
        ("agent.ask", "question") => "What specific target should I use?",
        ("queue.enqueue", "content") | ("queue.edit", "content") => "Owner task",
        ("queue.enqueue", "reason") | ("queue.edit", "reason") => "owner request",
        ("queue.edit", "id") | ("queue.delete", "id") | ("queue.redeliver", "id") => "1",
        ("queue.delete", "reason") | ("queue.redeliver", "reason") => "owner request",
        ("shell.run", "command") => "pwd",
        ("verify.cargo", "gate") => "test",
        ("verify.xtask", "gate") => "check-docs",
        _ => "value",
    }
}
