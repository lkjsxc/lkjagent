use lkjagent_store::error::StoreResult;
use lkjagent_store::runtime_authority::{record_dense_runtime_row, DenseRuntimeRowInput};
use rusqlite::Connection;

use crate::kernel::{RuntimeDecision, RuntimeSnapshot};

pub fn persist_dense_rows(
    conn: &Connection,
    decision_id: i64,
    snapshot: &RuntimeSnapshot,
    decision: &RuntimeDecision,
    created_at: &str,
) -> StoreResult<()> {
    let mut rows = dense_rows(decision_id, snapshot, decision, created_at);
    for row in rows.drain(..) {
        record_dense_runtime_row(
            conn,
            &DenseRuntimeRowInput {
                decision_id,
                row_kind: &row.kind,
                subject: &row.subject,
                predicate: &row.predicate,
                object: &row.object,
                created_at,
            },
        )?;
    }
    Ok(())
}

fn dense_rows(
    decision_id: i64,
    snapshot: &RuntimeSnapshot,
    decision: &RuntimeDecision,
    _created_at: &str,
) -> Vec<OwnedDenseRow> {
    let mut rows = vec![
        row(
            "fact",
            "root",
            "status",
            snapshot.artifact.root.as_deref().unwrap_or("none"),
        ),
        row(
            "fact",
            "evidence",
            "missing",
            &decision.missing_evidence.join(","),
        ),
        row(
            "fact",
            "weak_paths",
            "count",
            &decision.weak_paths.len().to_string(),
        ),
    ];
    rows.extend(
        decision
            .missing_evidence
            .iter()
            .map(|item| row("obligation", item, "required_by", "missing-evidence")),
    );
    if let Some(plan) = &decision.resolver_plan {
        rows.push(row("resolver_plan", "selected", "label", plan));
        if let Some(rule) = resolver_rule(plan) {
            rows.push(row("resolver_rule", "selected", "id", rule));
        }
    }
    if let Some(progress) = &decision.progress_key {
        rows.push(row("progress", "selected", "key", progress));
    }
    rows.extend(
        decision
            .completion_gate_inputs
            .iter()
            .map(|input| row("completion_input", "completion", "input", input)),
    );
    rows.into_iter()
        .map(|mut row| {
            row.subject = format!("decision:{decision_id}:{}", row.subject);
            row
        })
        .collect()
}

fn resolver_rule(plan: &str) -> Option<&str> {
    plan.strip_prefix("rule=")
        .and_then(|value| value.split_once(' '))
        .map(|parts| parts.0)
}

fn row(kind: &str, subject: &str, predicate: &str, object: &str) -> OwnedDenseRow {
    OwnedDenseRow {
        kind: kind.to_string(),
        subject: subject.to_string(),
        predicate: predicate.to_string(),
        object: object.to_string(),
    }
}

struct OwnedDenseRow {
    kind: String,
    subject: String,
    predicate: String,
    object: String,
}
