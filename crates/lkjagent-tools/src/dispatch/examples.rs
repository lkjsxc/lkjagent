use lkjagent_protocol::registry::{find_tool, ParamSpec};
use lkjagent_protocol::{render_action, Action, Param};
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExampleContext {
    pub evidence_requirement: Option<String>,
    pub missing_evidence: Vec<String>,
    pub allowed_packages: Vec<String>,
    pub legal_transitions: Vec<String>,
    pub artifact_root: Option<String>,
    pub owner_objective: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionExample {
    pub action: Action,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExampleError {
    UnknownTool(String),
}

impl fmt::Display for ExampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExampleError::UnknownTool(tool) => write!(formatter, "unknown tool: {tool}"),
        }
    }
}

impl std::error::Error for ExampleError {}

pub fn registry_valid_example(tool: &str) -> Option<String> {
    valid_example_for(tool, ExampleContext::default())
        .ok()
        .map(|example| example.render())
}

pub fn valid_example(tool: &str, specs: &[ParamSpec]) -> String {
    let params = example_params(tool, specs, &ExampleContext::default());
    render_action(&Action::new(tool, params))
}

pub fn valid_example_for(
    tool: &str,
    context: ExampleContext,
) -> Result<ActionExample, ExampleError> {
    let spec = find_tool(tool).ok_or_else(|| ExampleError::UnknownTool(tool.to_string()))?;
    Ok(ActionExample {
        action: Action::new(tool, example_params(tool, spec.params, &context)),
    })
}

impl ActionExample {
    pub fn render(&self) -> String {
        render_action(&self.action)
    }
}

fn example_params(tool: &str, specs: &[ParamSpec], context: &ExampleContext) -> Vec<Param> {
    let mut params = specs
        .iter()
        .filter(|spec| spec.required)
        .map(|spec| Param::new(spec.name, example_value(tool, spec.name, context)))
        .collect::<Vec<_>>();
    if tool == "graph.plan" {
        params.push(Param::new("checks", "dispatch accepts semantic plan"));
        params.push(Param::new(
            "paths",
            context
                .artifact_root
                .clone()
                .unwrap_or_else(|| ".".to_string()),
        ));
    }
    params
}

fn example_value(tool: &str, name: &str, context: &ExampleContext) -> String {
    if (tool, name) == ("graph.evidence", "kind") {
        return context
            .evidence_requirement
            .clone()
            .or_else(|| context.missing_evidence.first().cloned())
            .unwrap_or_else(|| "observation".to_string());
    }
    if (tool, name) == ("graph.transition", "target") {
        return context
            .legal_transitions
            .first()
            .cloned()
            .unwrap_or_else(|| "plan".to_string());
    }
    if (tool, name) == ("graph.context", "packages") {
        return context
            .allowed_packages
            .first()
            .cloned()
            .unwrap_or_else(|| "planning-checklist".to_string());
    }
    if (tool, name) == ("artifact.plan", "root")
        || (tool, name) == ("artifact.apply", "root")
        || (tool, name) == ("artifact.audit", "root")
        || (tool, name) == ("artifact.next", "root")
        || (tool, name) == ("doc.audit", "root")
    {
        return context
            .artifact_root
            .clone()
            .unwrap_or_else(|| "stories/example-story".to_string());
    }
    match (tool, name) {
        ("doc.scaffold", "root") | ("doc.audit", "root") => "docs",
        ("doc.scaffold", "title") => "Project Documentation",
        ("artifact.plan", "title") => "Example Story",
        ("artifact.plan", "kind") => "story",
        ("fs.read", "path") | ("fs.stat", "path") | ("fs.mkdir", "path") => "README.md",
        ("fs.write", "path") | ("fs.edit", "path") | ("fs.patch", "path") => "README.md",
        ("fs.read_many", "paths") => "README.md",
        ("fs.write", "content") => "Example content",
        ("fs.edit", "find") => "Example",
        ("fs.edit", "replace") => "Updated",
        ("fs.patch", "patch") => "*** Begin Patch\n*** End Patch",
        ("fs.batch_write", "files") => return batch_files_value(context),
        ("fs.search", "query") | ("memory.find", "query") => "README",
        ("graph.note", "kind") => "decision",
        ("graph.evidence", "summary") => "Read README.md",
        ("graph.note", "summary") => "Chose smaller recovery action",
        ("graph.plan", "objective") => {
            return context
                .owner_objective
                .clone()
                .unwrap_or_else(|| "Complete the owner task".to_string())
        }
        ("graph.plan", "steps") => "Inspect state\nAct in one bounded step",
        ("graph.plan", "reason") => "Graph planning requirement",
        ("graph.transition", "reason") => "Best legal next node",
        ("graph.context", "reason") => "Need planning context",
        ("graph.compact", "reason") => "Context pressure",
        ("memory.save", "kind") => "lesson",
        ("memory.save", "title") => "Useful lesson",
        ("memory.save", "content") => "Record only observed durable facts.",
        ("diary.record", "title") => "Prototype notes after Chronos run",
        ("diary.record", "content") => "Write refusals were valid; recovery routing changed.",
        ("schedule.add", "title") => "Review recovery smoke",
        ("schedule.add", "start") => "2026-06-25T09:00:00+09:00",
        ("todo.add", "title") => "Finish transition-kernel proof",
        ("diary.find", "query") | ("schedule.list", "query") | ("todo.list", "query") => {
            "Chronos recovery"
        }
        ("schedule.update", "id") | ("todo.update", "id") => "1",
        ("schedule.update", "status") | ("todo.update", "status") => "done",
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
    .to_string()
}

fn batch_files_value(context: &ExampleContext) -> String {
    let root = context
        .artifact_root
        .clone()
        .unwrap_or_else(|| "docs".to_string());
    format!(
        "path: {root}/README.md\ncontent:\n# Artifact Guide\n\nConcrete content tied to the active artifact."
    )
}
