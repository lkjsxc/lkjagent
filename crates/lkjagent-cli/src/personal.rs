#[path = "personal_projection.rs"]
mod personal_projection;

use std::path::Path;

use lkjagent_store::personal::{list, PersonalListFilter, PersonalRecord};

use crate::args::PersonalCommand;
use crate::error::CliError;
use crate::store::open_store;

pub fn personal(data_dir: &Path, command: PersonalCommand) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    match command {
        PersonalCommand::List {
            kind,
            status,
            project,
            limit,
        } => list_records(&conn, kind, status, project, limit),
        PersonalCommand::Render => render_projection(data_dir, &conn),
    }
}

fn list_records(
    conn: &rusqlite::Connection,
    kind: Option<String>,
    status: Option<String>,
    project: Option<String>,
    limit: usize,
) -> Result<String, CliError> {
    let records = list(
        conn,
        &PersonalListFilter {
            kind: kind.as_deref(),
            status: status.as_deref(),
            project: project.as_deref(),
            start: None,
            end: None,
            limit,
        },
    )?;
    Ok(render_list(&records))
}

fn render_projection(data_dir: &Path, conn: &rusqlite::Connection) -> Result<String, CliError> {
    let paths = personal_projection::render(data_dir, conn)?;
    let mut lines = vec![format!("personal_projection\nwritten={}", paths.len())];
    for path in paths {
        lines.push(format!("- {}", path.display()));
    }
    Ok(lines.join("\n"))
}

fn render_list(records: &[PersonalRecord]) -> String {
    let mut lines = vec![format!("personal_records\nreturned={}", records.len())];
    for record in records {
        lines.push(format!(
            "- id={} kind={} status={} title={} project={} start_at={} due_at={}",
            record.id,
            record.kind,
            record.status,
            one_line(&record.title),
            record.project.as_deref().unwrap_or("none"),
            record.start_at.as_deref().unwrap_or("none"),
            record.due_at.as_deref().unwrap_or("none")
        ));
    }
    lines.join("\n")
}

fn one_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
