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
