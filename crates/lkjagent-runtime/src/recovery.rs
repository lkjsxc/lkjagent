use lkjagent_protocol::registry::find_tool;
use lkjagent_protocol::ParseFault;

use crate::task::StopReason;

pub const ESCALATE_AFTER: u8 = 3;

pub fn parse_notice(fault: &ParseFault) -> String {
    match fault {
        ParseFault::MissingAct => "parse fault: missing act block".to_string(),
        ParseFault::MultipleAct => "parse fault: multiple act blocks".to_string(),
        ParseFault::MissingTool => "parse fault: missing tool".to_string(),
        ParseFault::UnknownTool { tool } => format!("parse fault: unknown tool {tool}"),
        ParseFault::UnclosedTag { tag } => format!("parse fault: unclosed tag {tag}"),
        ParseFault::DuplicateParam { name } => {
            format!("parse fault: duplicate parameter {name}")
        }
        ParseFault::BadParams {
            tool,
            missing,
            unknown,
        } => param_fault_notice(tool, missing, unknown),
    }
}

pub fn stop_reason(fault: &ParseFault) -> StopReason {
    match fault {
        ParseFault::UnknownTool { .. } => StopReason::UnknownTool,
        ParseFault::DuplicateParam { .. } | ParseFault::BadParams { .. } => StopReason::BadParams,
        _ => StopReason::InvalidAction,
    }
}

pub fn should_escalate(count: u8) -> bool {
    count >= ESCALATE_AFTER
}

pub fn parse_recovery_notice(fault: &ParseFault, count: u8) -> String {
    if matches!(
        fault,
        ParseFault::BadParams { .. } | ParseFault::DuplicateParam { .. }
    ) {
        return params_recovery_notice(count);
    }
    if should_escalate(count) {
        return format!(
            "recovery: parse faults are consecutive count={count}; simplify to one valid act block; prefer typed file/doc tools for large payloads; ask only if blocked"
        );
    }
    "recovery: the previous completion was not executed; emit exactly one valid act block next"
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

fn valid_example(tool: &str) -> String {
    let Some(spec) = find_tool(tool) else {
        return format!("<act>\n<tool>{tool}</tool>\n</act>");
    };
    let mut lines = vec!["<act>".to_string(), format!("<tool>{tool}</tool>")];
    for param in spec.params.iter().filter(|param| param.required) {
        lines.push(format!("<{}>VALUE</{}>", param.name, param.name));
    }
    lines.push("</act>".to_string());
    lines.join("\n")
}
