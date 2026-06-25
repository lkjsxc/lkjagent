use rusqlite::Connection;

use lkjagent_graph::case_document::DocumentState;
use lkjagent_graph::case_evidence::EvidenceRequirementState;
use lkjagent_graph::case_fields::ConstraintRecord;
use lkjagent_graph::{completion::refresh_completion_state, TaskFamily, TaskGraphState};
use lkjagent_store::state as store_state;
use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::count_guard::{CountKind, CountMode};

use crate::error::RuntimeResult;

pub fn append_store_guard(conn: &Connection, graph: String) -> RuntimeResult<String> {
    let value = store_state::get(conn, "completion guard")?.unwrap_or_else(|| "none".to_string());
    Ok(append_guard(
        graph,
        CompletionGuard::from_state_value(&value),
    ))
}

pub fn append_case_guard(state: &mut TaskGraphState, guard: CompletionGuard) {
    if guard.is_recursive() {
        apply_recursive_guard(state, guard);
    }
    if let Some(text) = guard_text(guard) {
        state
            .constraints
            .push(ConstraintRecord::hard(text, "completion guard"));
    }
}

fn apply_recursive_guard(state: &mut TaskGraphState, guard: CompletionGuard) {
    state.family = if guard.is_knowledge() {
        TaskFamily::KnowledgeBase
    } else {
        TaskFamily::Documentation
    };
    state.subroute = "document-construction".to_string();
    state.route_reason = "recursive completion guard requires document construction".to_string();
    state.document = Some(DocumentState::planned("docs", "recursive-structure"));
    push_package(state, "doc-construction");
    push_requirement(state, "document-structure");
    push_requirement(state, "verification");
    push_check(state, "document audit");
    push_check(state, "focused verification");
    refresh_completion_state(state);
}

fn push_package(state: &mut TaskGraphState, package: &str) {
    if !state
        .context
        .selected_packages
        .iter()
        .any(|item| item == package)
    {
        state.context.selected_packages.push(package.to_string());
    }
}

fn push_requirement(state: &mut TaskGraphState, id: &str) {
    if state.evidence.knows_requirement(id) {
        return;
    }
    state.evidence.requirements.push(EvidenceRequirementState {
        id: id.to_string(),
        description: format!("{id} evidence"),
        required_for_completion: true,
    });
}

fn push_check(state: &mut TaskGraphState, check: &str) {
    if !state
        .evidence
        .pending_checks
        .iter()
        .any(|item| item == check)
    {
        state.evidence.pending_checks.push(check.to_string());
    }
}

fn append_guard(graph: String, guard: CompletionGuard) -> String {
    if graph.contains("completion_guard=") {
        return graph;
    }
    if let Some(text) = guard_text(guard) {
        return format!("{text}\n{graph}");
    }
    graph
}

fn guard_text(guard: CompletionGuard) -> Option<String> {
    if guard.is_recursive() {
        return Some(format!(
            "completion_guard={}\nrecursive_guard_instruction=README-indexed recursive tree required; prefer doc.scaffold, doc.audit, fs.batch_write, and verification evidence before agent.done",
            guard.as_state_value()
        ));
    }
    let count = guard.count_guard()?;
    Some(format!(
        "completion_guard={}\ncount_guard_instruction={}",
        guard.as_state_value(),
        count_instruction(count.kind, count.mode)
    ))
}

fn count_instruction(kind: CountKind, mode: CountMode) -> &'static str {
    match (kind, mode) {
        (CountKind::File, CountMode::Exact) => {
            "exact file count active; prefer doc.scaffold, fs.list, fs.stat, doc.audit, or fs.batch_write; shell.run is an escape hatch only when graph policy admits it"
        }
        (CountKind::File, CountMode::Approximate) => {
            "approximate file scale active; treat the number as a size hint and prefer doc.scaffold plus doc.audit; shell.run is an escape hatch only when graph policy admits it"
        }
        (CountKind::Markdown, CountMode::Exact) => {
            "exact markdown count active; prefer doc.scaffold, fs.list, fs.stat, doc.audit, or fs.batch_write; shell.run is an escape hatch only when graph policy admits it"
        }
        (CountKind::Markdown, CountMode::Approximate) => {
            "approximate markdown count active; prefer doc.scaffold plus doc.audit within tolerance; shell.run is an escape hatch only when graph policy admits it"
        }
    }
}
