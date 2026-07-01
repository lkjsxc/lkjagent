use super::model::{FileRow, ProofBundle, WordCountRow};
use super::render_rows::{
    case_rows, contract_rows, count_rows, decision_rows, event_rows, file_rows, queue_rows,
    readiness_rows, word_count_rows,
};

pub fn summary(bundle: &ProofBundle) -> String {
    [
        "# Proof Bundle".to_string(),
        String::new(),
        format!("store_path={}", bundle.store_path),
        format!("store_present={}", bundle.store_present),
        format!("graph_cases={}", bundle.cases.len()),
        format!("artifact_readiness={}", bundle.readiness.len()),
        format!("active_contracts={}", bundle.active_contracts.len()),
        format!("workspace_files={}", bundle.workspace_files.len()),
        format!("model_logs={}", bundle.model_logs.len()),
        format!("warnings={}", bundle.warnings.len()),
        String::new(),
        "Files: status.md, word-counts.md, workspace.md, model-logs.md, warnings.md".to_string(),
    ]
    .join("\n")
}

pub fn status(bundle: &ProofBundle) -> String {
    let mut out = vec!["# Status Snapshot".to_string(), String::new()];
    table(&mut out, "Graph Cases", case_rows(&bundle.cases));
    table(&mut out, "Queue Counts", count_rows(&bundle.queue_counts));
    table(&mut out, "Recent Queue", queue_rows(&bundle.queue_recent));
    table(
        &mut out,
        "Artifact Readiness",
        readiness_rows(&bundle.readiness),
    );
    table(
        &mut out,
        "Active Contracts",
        contract_rows(&bundle.active_contracts),
    );
    table(
        &mut out,
        "Authority Decisions",
        decision_rows(&bundle.decisions),
    );
    table(
        &mut out,
        "Transcript Metadata",
        event_rows(&bundle.transcript),
    );
    out.join("\n")
}

pub fn word_counts(rows: &[WordCountRow]) -> String {
    let mut out = vec!["# Word Counts".to_string(), String::new()];
    table(&mut out, "Artifact Roots", word_count_rows(rows));
    out.join("\n")
}

pub fn file_index(title: &str, rows: &[FileRow]) -> String {
    let mut out = vec![format!("# {title}"), String::new()];
    table(&mut out, "Files", file_rows(rows));
    out.join("\n")
}

pub fn warnings(rows: &[String]) -> String {
    let mut out = vec!["# Warnings".to_string(), String::new()];
    if rows.is_empty() {
        out.push("none".to_string());
    } else {
        out.extend(rows.iter().map(|row| format!("- {}", clean(row))));
    }
    out.join("\n")
}

fn table(out: &mut Vec<String>, title: &str, rows: Vec<Vec<String>>) {
    out.push(format!("## {title}"));
    out.push(String::new());
    if rows.is_empty() {
        out.push("none".to_string());
        out.push(String::new());
        return;
    }
    for row in rows {
        out.push(
            row.into_iter()
                .map(|cell| clean(&cell))
                .collect::<Vec<_>>()
                .join(" | "),
        );
    }
    out.push(String::new());
}

fn clean(value: &str) -> String {
    value
        .replace('\n', " ")
        .replace('|', "/")
        .trim()
        .to_string()
}
