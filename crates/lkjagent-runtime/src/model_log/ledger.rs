use std::collections::BTreeSet;

use rusqlite::Connection;

use super::text::{cell, cell_limited, section, MAX_TRANSCRIPT_CELL_CHARS};
use crate::error::RuntimeResult;

type CaseRow = lkjagent_store::graph::GraphCaseRow;
type EventRow = lkjagent_store::events::EventRow;

pub fn queue_depth(conn: &Connection) -> RuntimeResult<usize> {
    Ok(lkjagent_store::queue::list(conn)?
        .iter()
        .filter(|row| row.status == "pending")
        .count())
}

pub fn touched_paths(
    out: &mut String,
    conn: &Connection,
    case: Option<&CaseRow>,
) -> RuntimeResult<()> {
    section(out, "Touched Paths");
    let mut paths = BTreeSet::new();
    if let Some(case) = case {
        for evidence in lkjagent_store::graph::evidence_for_case(conn, case.id)? {
            if let Some(path) = evidence.path {
                paths.insert(path);
            }
        }
        add_artifact_paths(conn, case.id, &mut paths)?;
    }
    if paths.is_empty() {
        out.push_str("* none\n\n");
        return Ok(());
    }
    for path in paths {
        out.push_str(&format!("* `{}`\n", path));
    }
    out.push('\n');
    Ok(())
}

fn add_artifact_paths(
    conn: &Connection,
    case_id: i64,
    paths: &mut BTreeSet<String>,
) -> RuntimeResult<()> {
    let Some(artifact) = lkjagent_store::artifact_ledger::latest_for_case(conn, case_id)? else {
        return Ok(());
    };
    paths.insert(artifact.root.clone());
    if let Some(cursor) = lkjagent_store::artifact_cursor::latest_batch_cursor(conn, artifact.id)? {
        for path in split_paths(&cursor.completed_paths) {
            paths.insert(format!("{}/{}", artifact.root, path));
        }
        for path in split_paths(&cursor.failed_paths) {
            paths.insert(format!("{}/{}", artifact.root, path));
        }
    }
    Ok(())
}

fn split_paths(value: &str) -> impl Iterator<Item = &str> {
    value
        .split(',')
        .map(str::trim)
        .filter(|path| !path.is_empty())
}

pub fn evidence(out: &mut String, conn: &Connection, case: Option<&CaseRow>) -> RuntimeResult<()> {
    section(out, "Evidence Ledger");
    out.push_str("| kind | requirement | summary | path | confidence |\n");
    out.push_str("| --- | --- | --- | --- | --- |\n");
    let rows = match case {
        Some(case) => lkjagent_store::graph::evidence_for_case(conn, case.id)?,
        None => Vec::new(),
    };
    if rows.is_empty() {
        out.push_str("| none | none | none | none | low |\n\n");
        return Ok(());
    }
    for row in rows.iter().rev().take(12).rev() {
        out.push_str(&format!(
            "| {} | {} | {} | {} | medium |\n",
            cell(&row.kind),
            cell(&row.requirement),
            cell(&row.summary),
            cell(row.path.as_deref().unwrap_or("none"))
        ));
    }
    out.push('\n');
    Ok(())
}

pub fn faults(out: &mut String, case: Option<&CaseRow>, events: &[EventRow]) {
    section(out, "Fault Ledger");
    out.push_str("| turn | kind | message | recovery |\n");
    out.push_str("| --- | --- | --- | --- |\n");
    let rows = events
        .iter()
        .filter(|event| event.kind == "error" || event.content.contains("fault"))
        .rev()
        .take(8)
        .collect::<Vec<_>>();
    if rows.is_empty() {
        out.push_str("| none | none | none | none |\n\n");
        return;
    }
    for event in rows.into_iter().rev() {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            event
                .turn
                .map_or_else(|| "none".to_string(), |turn| turn.to_string()),
            cell(&event.kind),
            cell(&event.content),
            cell(case.map_or("inspect recent transcript", |case| {
                case.active_node.as_str()
            }))
        ));
    }
    out.push('\n');
}

pub fn transcript(out: &mut String, events: &[EventRow], budget_chars: usize) {
    section(out, "Recent Transcript");
    out.push_str("| id | turn | kind | preview |\n");
    out.push_str("| --- | --- | --- | --- |\n");
    if events.is_empty() {
        out.push_str("| 0 | none | none | none |\n\n");
        return;
    }
    let mut rows = Vec::new();
    let mut used = 0usize;
    for event in events.iter().rev() {
        let remaining = budget_chars.saturating_sub(used);
        if remaining < 160 {
            break;
        }
        let content_limit = remaining.min(MAX_TRANSCRIPT_CELL_CHARS);
        let row = format!(
            "| {} | {} | {} | {} |\n",
            event.id,
            event
                .turn
                .map_or_else(|| "none".to_string(), |turn| turn.to_string()),
            cell(&event.kind),
            cell_limited(&event.content, content_limit)
        );
        used += row.chars().count();
        rows.push(row);
        if used >= budget_chars {
            break;
        }
    }
    if rows.is_empty() {
        out.push_str("| 0 | none | none | transcript budget exhausted |\n");
    } else {
        for row in rows.into_iter().rev() {
            out.push_str(&row);
        }
    }
    out.push('\n');
}

pub fn verification(out: &mut String, case: Option<&CaseRow>) {
    section(out, "Verification");
    out.push_str("| command | result | notes |\n");
    out.push_str("| --- | --- | --- |\n");
    let Some(case) = case else {
        out.push_str("| none | not-run | no active case |\n\n");
        return;
    };
    if case.pending_checks.is_empty() {
        out.push_str("| none | unknown | no pending checks recorded |\n\n");
        return;
    }
    for check in &case.pending_checks {
        out.push_str(&format!(
            "| {} | pending | graph case check |\n",
            cell(check)
        ));
    }
    out.push('\n');
}
