#[path = "personal_projection_text.rs"]
mod text;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_store::personal::{list, PersonalListFilter, PersonalRecord};
use rusqlite::Connection;

use crate::error::CliError;
use text::{empty, one_line, slug};

const MAX_ROWS_PER_FILE: usize = 40;

pub fn render(data_dir: &Path, conn: &Connection) -> Result<Vec<PathBuf>, CliError> {
    let root = data_dir.join("personal");
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(&root)?;
    let mut written = Vec::new();
    write_diary(&root, &records(conn, "diary")?, &mut written)?;
    write_schedule(&root, &records(conn, "schedule")?, &mut written)?;
    write_todos(&root, &records(conn, "todo")?, &mut written)?;
    Ok(written)
}

fn records(conn: &Connection, kind: &str) -> Result<Vec<PersonalRecord>, CliError> {
    Ok(list(
        conn,
        &PersonalListFilter {
            kind: Some(kind),
            status: None,
            project: None,
            start: None,
            end: None,
            limit: 100,
        },
    )?)
}

fn write_diary(
    root: &Path,
    records: &[PersonalRecord],
    written: &mut Vec<PathBuf>,
) -> Result<(), CliError> {
    let mut groups: BTreeMap<String, Vec<&PersonalRecord>> = BTreeMap::new();
    for record in records {
        groups.entry(date_key(record)).or_default().push(record);
    }
    for (date, group) in groups {
        let path = root
            .join("journal")
            .join(date.get(0..4).unwrap_or("undated"))
            .join(date.get(5..7).unwrap_or("00"))
            .join(format!("{date}.md"));
        write_doc(
            &path,
            &render_entries(&format!("Journal {date}"), &group),
            written,
        )?;
    }
    Ok(())
}

fn write_schedule(
    root: &Path,
    records: &[PersonalRecord],
    written: &mut Vec<PathBuf>,
) -> Result<(), CliError> {
    let mut months: BTreeMap<String, Vec<&PersonalRecord>> = BTreeMap::new();
    for record in records {
        let month = date_key(record).get(0..7).unwrap_or("undated").to_string();
        months.entry(month).or_default().push(record);
        let file = format!("{}-{}.md", record.id, slug(&record.title));
        let path = root.join("schedule/events").join(file);
        write_doc(&path, &render_record("Schedule Event", record), written)?;
    }
    for (month, group) in months {
        let path = root.join("schedule/months").join(format!("{month}.md"));
        write_doc(
            &path,
            &render_entries(&format!("Schedule {month}"), &group),
            written,
        )?;
    }
    Ok(())
}

fn write_todos(
    root: &Path,
    records: &[PersonalRecord],
    written: &mut Vec<PathBuf>,
) -> Result<(), CliError> {
    let open = records
        .iter()
        .filter(|record| record.status != "done" && record.status != "canceled")
        .collect::<Vec<_>>();
    write_doc(
        &root.join("todos/open.md"),
        &render_entries("Open TODOs", &open),
        written,
    )?;
    let mut projects: BTreeMap<String, Vec<&PersonalRecord>> = BTreeMap::new();
    for record in records {
        if let Some(project) = record.project.as_deref().filter(|value| !value.is_empty()) {
            projects
                .entry(project.to_string())
                .or_default()
                .push(record);
        }
    }
    for (project, group) in projects {
        let path = root
            .join("todos/projects")
            .join(format!("{}.md", slug(&project)));
        write_doc(
            &path,
            &render_entries(&format!("TODOs {project}"), &group),
            written,
        )?;
    }
    Ok(())
}

fn render_entries(title: &str, records: &[&PersonalRecord]) -> String {
    let mut lines = vec![
        format!("# {title}"),
        String::new(),
        "Generated from SQLite.".to_string(),
    ];
    for record in records.iter().take(MAX_ROWS_PER_FILE) {
        lines.push(String::new());
        lines.push(format!("## id={} {}", record.id, one_line(&record.title)));
        lines.push(facts(record));
        if !record.body.trim().is_empty() {
            lines.push(one_line(&record.body));
        }
    }
    if records.len() > MAX_ROWS_PER_FILE {
        lines.push(format!(
            "omitted={} over projection limit",
            records.len() - MAX_ROWS_PER_FILE
        ));
    }
    lines.join("\n")
}

fn render_record(title: &str, record: &PersonalRecord) -> String {
    render_entries(title, &[record])
}

fn facts(record: &PersonalRecord) -> String {
    format!(
        "kind={} status={} tags={} start_at={} due_at={} project={}",
        record.kind,
        record.status,
        empty(&record.tags),
        record.start_at.as_deref().unwrap_or("none"),
        record.due_at.as_deref().unwrap_or("none"),
        record.project.as_deref().unwrap_or("none")
    )
}

fn write_doc(path: &Path, content: &str, written: &mut Vec<PathBuf>) -> Result<(), CliError> {
    if content.lines().count() > 200 {
        return Err(CliError::failure(format!(
            "projection too long: {}",
            path.display()
        )));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    written.push(path.to_path_buf());
    Ok(())
}

fn date_key(record: &PersonalRecord) -> String {
    let value = record.start_at.as_deref().unwrap_or(&record.created_at);
    value.get(0..10).unwrap_or("undated").to_string()
}
