use crate::dispatch::{valid_example_for, ActionExample, ExampleContext};
use lkjagent_protocol::Action;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnerQuestionDecision {
    Admit {
        question: String,
    },
    Refuse {
        reason: String,
        internal_next_action: ActionExample,
    },
}

pub fn decide_owner_question(question: &str) -> OwnerQuestionDecision {
    let trimmed = question.trim();
    let lower = trimmed.to_ascii_lowercase();
    if let Some((reason, tool)) = internal_question(&lower) {
        return OwnerQuestionDecision::Refuse {
            reason: reason.to_string(),
            internal_next_action: example(tool),
        };
    }
    OwnerQuestionDecision::Admit {
        question: trimmed.to_string(),
    }
}

fn internal_question(lower: &str) -> Option<(&'static str, &'static str)> {
    if mentions_tool_schema(lower) {
        return Some(("tool schema questions are runtime-owned", "graph.note"));
    }
    if mentions_maintenance_scan(lower) {
        return Some(("maintenance must inspect records directly", "memory.find"));
    }
    if mentions_recovery_how_to(lower) {
        return Some((
            "tool recovery must choose a smaller valid action",
            "doc.scaffold",
        ));
    }
    None
}

fn mentions_tool_schema(lower: &str) -> bool {
    (lower.contains("valid kind") || lower.contains("valid kinds"))
        || lower.contains("tool schema")
        || lower.contains("parameter schema")
        || lower.contains("graph.note")
        || lower.contains("memory.save kind")
}

fn mentions_maintenance_scan(lower: &str) -> bool {
    lower.contains("transcript span")
        || lower.contains("stale memory")
        || lower.contains("stale rows")
        || lower.contains("rows need pruning")
        || lower.contains("what should be distilled")
}

fn mentions_recovery_how_to(lower: &str) -> bool {
    (lower.contains("how should i") || lower.contains("what should i do"))
        && (lower.contains("fs.write")
            || lower.contains("blocked")
            || lower.contains("unclosed")
            || lower.contains("parse fault"))
}

fn example(tool: &str) -> ActionExample {
    match valid_example_for(tool, ExampleContext::default()) {
        Ok(example) => example,
        Err(_) => ActionExample {
            action: Action::new("graph.state", Vec::new()),
        },
    }
}
