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
        ParseFault::BadParams { missing, unknown } => format!(
            "parse fault: missing params [{}]; unknown params [{}]",
            missing.join(", "),
            unknown.join(", ")
        ),
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

pub fn parse_recovery_notice(count: u8) -> String {
    if should_escalate(count) {
        return format!(
            "recovery: parse faults are consecutive count={count}; simplify to one valid act block; prefer typed file/doc tools for large payloads; ask only if blocked"
        );
    }
    "recovery: the previous completion was not executed; emit exactly one valid act block next"
        .to_string()
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
