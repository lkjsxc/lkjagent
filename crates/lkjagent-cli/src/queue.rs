use std::path::Path;

use crate::args::QueueCommand;
use crate::error::CliError;
use crate::store::open_store;

pub fn queue(data_dir: &Path, command: QueueCommand) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let rows = lkjagent_store::queue::list(&conn)?;
    match command {
        QueueCommand::List { limit } => Ok(render_list(&rows, limit)),
        QueueCommand::Show { id } => render_show(&rows, id),
    }
}

fn render_list(rows: &[lkjagent_store::queue::QueueRow], limit: Option<usize>) -> String {
    let selected = rows
        .iter()
        .rev()
        .take(limit.unwrap_or(rows.len()))
        .collect::<Vec<_>>();
    let mut lines = vec![format!("queue_rows={}", selected.len())];
    for row in selected.into_iter().rev() {
        lines.push(format!(
            "id={} status={} delivered_turn={} source_queue_id={} preview={}",
            row.id,
            row.status,
            optional_i64(row.delivered_turn),
            optional_i64(row.source_queue_id),
            preview(&row.content)
        ));
    }
    lines.join("\n")
}

fn render_show(rows: &[lkjagent_store::queue::QueueRow], id: i64) -> Result<String, CliError> {
    let Some(row) = rows.iter().find(|row| row.id == id) else {
        return Err(CliError::failure(format!("queue_not_found={id}")));
    };
    Ok([
        format!("queue_id={}", row.id),
        format!("status={}", row.status),
        format!("created_at={}", row.created_at),
        format!("updated_at={}", row.updated_at),
        format!("source_queue_id={}", optional_i64(row.source_queue_id)),
        format!("delivered_turn={}", optional_i64(row.delivered_turn)),
        format!("content={}", row.content),
    ]
    .join("\n"))
}

fn optional_i64(value: Option<i64>) -> String {
    value.map_or_else(|| "none".to_string(), |value| value.to_string())
}

fn preview(value: &str) -> String {
    let mut out = value.chars().take(80).collect::<String>();
    if value.chars().count() > 80 {
        out.push_str("...");
    }
    out.replace('\n', " ")
}
