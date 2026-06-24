use lkjagent_protocol::registry::find_tool;
use lkjagent_protocol::ParseFault;

use crate::task::StopReason;

pub const ESCALATE_AFTER: u8 = 3;

pub fn parse_notice(fault: &ParseFault) -> String {
    match fault {
        ParseFault::MissingActionEnvelope => "parse fault: missing action envelope".to_string(),
        ParseFault::MultipleActionEnvelopes => "parse fault: multiple action envelopes".to_string(),
        ParseFault::UnclosedActionEnvelope => "parse fault: unclosed action envelope".to_string(),
        ParseFault::MissingTool => "parse fault: missing tool".to_string(),
        ParseFault::UnknownTool { tool } => format!("parse fault: unknown tool {tool}"),
        ParseFault::UnclosedTag { tag } => format!("parse fault: unclosed tag {tag}"),
        ParseFault::DuplicateParam { name } => {
            format!("parse fault: duplicate parameter {name}")
        }
        ParseFault::MalformedTag { line, .. } => format!("parse fault: malformed tag {line}"),
        ParseFault::AttributeLikeTag {
            tag_name,
            value_hint,
        } => attribute_like_notice(tag_name, value_hint.as_deref()),
        ParseFault::BadParams {
            tool,
            missing,
            unknown,
        } => param_fault_notice(tool, missing, unknown),
        ParseFault::BadEnvelope { reason } => format!("parse fault: bad envelope {reason}"),
        ParseFault::JsonActionRejected => "parse fault: json action rejected".to_string(),
    }
}

pub fn stop_reason(fault: &ParseFault) -> StopReason {
    match fault {
        ParseFault::UnknownTool { .. } => StopReason::UnknownTool,
        ParseFault::DuplicateParam { .. } | ParseFault::BadParams { .. } => StopReason::BadParams,
        ParseFault::BadEnvelope { .. } => StopReason::InvalidAction,
        _ => StopReason::InvalidAction,
    }
}

pub fn should_escalate(count: u8) -> bool {
    count >= ESCALATE_AFTER
}

pub fn parse_recovery_notice(fault: &ParseFault, count: u8) -> String {
    if matches!(fault, ParseFault::AttributeLikeTag { .. }) {
        return attribute_like_recovery_notice(count);
    }
    if matches!(
        fault,
        ParseFault::BadParams { .. } | ParseFault::DuplicateParam { .. }
    ) {
        return params_recovery_notice(count);
    }
    if should_escalate(count) {
        return format!(
            "recovery: parse faults are consecutive count={count}; simplify to one valid action block; prefer typed file/doc tools for large payloads; ask only if blocked"
        );
    }
    "recovery: the previous completion was not executed; emit exactly one valid action block next"
        .to_string()
}

fn attribute_like_recovery_notice(count: u8) -> String {
    if count >= ESCALATE_AFTER {
        return "recovery: attribute-like tag repeated; run deterministic graph.state inspection or record blocked handoff before asking for another repair".to_string();
    }
    if count >= 2 {
        return "recovery: attribute-like tag repeated; switch action class to graph.state before another graph.plan repair".to_string();
    }
    "recovery: attribute-like tag recorded; emit the next_executable_action with values between tags"
        .to_string()
}

pub fn params_recovery_notice(count: u8) -> String {
    if should_escalate(count) {
        return format!(
            "recovery: parameter faults are consecutive count={count}; use the valid_example exactly, inspect graph.state with no params, or choose fs.list/workspace.summary when you need a path"
        );
    }
    "recovery: parameter fault recorded; use the valid_example exactly or choose a tool whose schema accepts the parameter".to_string()
}

pub fn repeat_recovery_notice(count: u8) -> String {
    if should_escalate(count) {
        return format!(
            "recovery: repeat actions are consecutive count={count}; choose a different tool action, inspect state, or switch to typed batch/doc tools"
        );
    }
    "recovery: repeated action was refused; change the next action instead of resending it"
        .to_string()
}

pub fn tool_recovery_notice(error: &str) -> String {
    let preview = error.lines().next().unwrap_or("tool error");
    format!(
        "recovery: tool error recorded ({preview}); inspect the observation, adjust the path/command/params, and continue with a narrower action"
    )
}

fn param_fault_notice(tool: &str, missing: &[String], unknown: &[String]) -> String {
    format!(
        "action params refused\ntool={tool}\nexpected={}\nreceived={}\nvalid_example:\n{}\nnext_action=emit the valid_example exactly, or choose fs.list/workspace.summary if you need a path",
        expected_params(tool),
        received_params(missing, unknown),
        valid_example(tool)
    )
}

fn expected_params(tool: &str) -> String {
    let Some(spec) = find_tool(tool) else {
        return "unknown tool schema".to_string();
    };
    if spec.params.is_empty() {
        return "no parameters".to_string();
    }
    spec.params
        .iter()
        .map(|param| {
            if param.required {
                format!("{} required", param.name)
            } else {
                format!("{} optional", param.name)
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn received_params(missing: &[String], unknown: &[String]) -> String {
    format!(
        "missing [{}]; unknown [{}]",
        missing.join(", "),
        unknown.join(", ")
    )
}

fn attribute_like_notice(tag_name: &str, value_hint: Option<&str>) -> String {
    let repair_value = value_hint.unwrap_or("stories/chronos-fracture");
    format!(
        "parse fault: attribute-like tag {tag_name}\nfault=attribute_like_tag\ninvalid_tag_name={tag_name}\nrepair_rule=tag names cannot contain values\nrepair_tag=paths\nrepair_value={repair_value}\nnext_executable_action:\n{}",
        graph_plan_repair_example(repair_value)
    )
}

fn graph_plan_repair_example(path: &str) -> String {
    format!(
        "<action>\n<tool>graph.plan</tool>\n<objective>Create a structured science-fiction story bible for Chronos Fracture.</objective>\n<steps>\n1. Record the plan.\n2. Create the root README and catalog.\n3. Write bounded story-bible content batches.\n4. Audit document structure.\n5. Audit artifact readiness.\n</steps>\n<paths>{path}</paths>\n<reason>The owner requires evidence-gated story-bible construction.</reason>\n</action>"
    )
}

fn valid_example(tool: &str) -> String {
    let Some(spec) = find_tool(tool) else {
        return format!("<action>\n<tool>{tool}</tool>\n</action>");
    };
    let mut lines = vec!["<action>".to_string(), format!("<tool>{tool}</tool>")];
    for param in spec.params.iter().filter(|param| param.required) {
        lines.push(render_param_example(param.name));
    }
    lines.push("</action>".to_string());
    lines.join("\n")
}

fn render_param_example(name: &str) -> String {
    let value = match name {
        "command" => "cargo test -p lkjagent-protocol",
        "content" => "# Note\n\nConcrete content for the requested file.",
        "files" => "path: notes/example.md\ncontent:\n# Example\n\nConcrete content.",
        "find" => "old text",
        "gate" => "check-docs",
        "id" => "1",
        "kind" => "story",
        "objective" => "Create a structured story bible for Chronos Fracture.",
        "packages" => "repo-current-state,protocol-contract",
        "patch" => "--- a/notes/example.md\n+++ b/notes/example.md",
        "path" => "README.md",
        "paths" => "stories/chronos-fracture",
        "query" => "runtime authority",
        "question" => "Which artifact root should I use?",
        "reason" => "The runtime requires one schema-valid action.",
        "replace" => "new text",
        "root" => "stories/chronos-fracture",
        "steps" => "1. Inspect state.\n2. Take one bounded action.",
        "summary" => "The requested artifact is complete and audited.",
        "target" => "document-completion-check",
        "title" => "Chronos Fracture",
        _ => "concrete value",
    };
    if value.contains('\n') {
        format!("<{name}>\n{value}\n</{name}>")
    } else {
        format!("<{name}>{value}</{name}>")
    }
}
