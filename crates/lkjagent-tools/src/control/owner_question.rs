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
        return refusal(reason, tool);
    }
    if !external_owner_question(&lower) {
        return refusal(
            "owner questions require concrete external missing input",
            "workspace.summary",
        );
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
            "artifact.next",
        ));
    }
    if mentions_internal_recovery(lower) {
        return Some(("graph recovery strategy is runtime-owned", "graph.recover"));
    }
    if mentions_runtime_audit(lower) {
        return Some(("audits are runtime-owned", "doc.audit"));
    }
    if mentions_placeholder_repair(lower) {
        return Some(("artifact repair is runtime-owned", "artifact.next"));
    }
    if mentions_compaction(lower) {
        return Some(("compaction is runtime-owned", "graph.compact"));
    }
    if mentions_preemption(lower) {
        return Some(("maintenance preemption is runtime-owned", "queue.list"));
    }
    if mentions_completion_refusal(lower) {
        return Some(("completion refusal is runtime-owned", "artifact.audit"));
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
        || (lower.contains("memory row") && lower.contains("stale"))
        || (lower.contains("memory rows") && lower.contains("prune"))
        || lower.contains("rows need pruning")
        || lower.contains("what should be distilled")
}

fn mentions_recovery_how_to(lower: &str) -> bool {
    (lower.contains("how should i")
        || lower.contains("what should i do")
        || lower.contains("how do i")
        || lower.contains("how to use")
        || lower.contains("how to proceed"))
        && (lower.contains("fs.write")
            || lower.contains("scaffold writer")
            || lower.contains("blocked")
            || lower.contains("unclosed")
            || lower.contains("parse fault"))
}

fn mentions_internal_recovery(lower: &str) -> bool {
    (lower.contains("recovery strategy") || lower.contains("graph recovery"))
        && (lower.contains("internal") || lower.contains("runtime") || lower.contains("tool"))
}

fn mentions_runtime_audit(lower: &str) -> bool {
    lower.contains("run doc.audit")
        || lower.contains("run artifact.audit")
        || lower.contains("perform audit")
        || lower.contains("run an audit")
}

fn mentions_placeholder_repair(lower: &str) -> bool {
    lower.contains("repair placeholder")
        || lower.contains("fix placeholder")
        || lower.contains("weak path")
}

fn mentions_compaction(lower: &str) -> bool {
    lower.contains("compaction") || lower.contains("memory.save to preserve")
}

fn mentions_preemption(lower: &str) -> bool {
    lower.contains("preempt maintenance") || lower.contains("queued owner work")
}

fn mentions_completion_refusal(lower: &str) -> bool {
    lower.contains("refuse completion") || lower.contains("missing evidence")
}

fn external_owner_question(lower: &str) -> bool {
    has_owner_choice(lower) || requires_owner_fact(lower)
}

fn has_owner_choice(lower: &str) -> bool {
    (lower.contains("should") || lower.contains("which") || lower.contains("prefer"))
        && lower.contains(" or ")
}

fn requires_owner_fact(lower: &str) -> bool {
    lower.contains("secret")
        || lower.contains("credential")
        || lower.contains("api key")
        || lower.contains("endpoint")
        || lower.contains("external path")
        || lower.contains("owner preference")
}

fn refusal(reason: &'static str, tool: &'static str) -> OwnerQuestionDecision {
    OwnerQuestionDecision::Refuse {
        reason: reason.to_string(),
        internal_next_action: example(tool),
    }
}

fn example(tool: &str) -> ActionExample {
    match valid_example_for(tool, ExampleContext::default()) {
        Ok(example) => example,
        Err(_) => ActionExample {
            action: Action::new("graph.state", Vec::new()),
        },
    }
}
