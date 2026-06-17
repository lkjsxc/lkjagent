use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueFilter {
    All,
    Pending,
    Delivered,
    Deleted,
}

impl QueueFilter {
    pub fn parse(value: &str) -> ToolResult<Self> {
        match value {
            "all" => Ok(Self::All),
            "pending" => Ok(Self::Pending),
            "delivered" => Ok(Self::Delivered),
            "deleted" => Ok(Self::Deleted),
            other => Err(ToolError::invalid(format!("unknown queue status: {other}"))),
        }
    }

    fn accepts(self, status: &str) -> bool {
        matches!(self, Self::All)
            || matches!(self, Self::Pending if status == "pending")
            || matches!(self, Self::Delivered if status == "delivered")
            || matches!(self, Self::Deleted if status == "deleted")
    }
}

pub fn list(conn: &Connection, filter: QueueFilter, limit: usize) -> ToolResult<String> {
    if limit == 0 {
        return Err(ToolError::invalid("limit must be positive"));
    }
    let rows = lkjagent_store::queue::list(conn)?;
    let mut lines = Vec::new();
    for row in rows
        .iter()
        .filter(|row| filter.accepts(&row.status))
        .take(limit)
    {
        lines.push(format!(
            "id={} status={} source_queue_id={} created_at={} updated_at={} preview={}",
            row.id,
            row.status,
            row.source_queue_id
                .map_or_else(|| "null".to_string(), |id| id.to_string()),
            row.created_at,
            row.updated_at,
            preview(&row.content)
        ));
    }
    if lines.is_empty() {
        Ok("queue empty".to_string())
    } else {
        Ok(lines.join("\n"))
    }
}

pub fn enqueue(
    conn: &mut Connection,
    content: &str,
    reason: &str,
    now: &str,
) -> ToolResult<String> {
    require_content(content)?;
    require_reason(reason)?;
    let id = lkjagent_store::queue::enqueue(conn, content, reason, now)?;
    Ok(format!("queued id={id}"))
}

pub fn edit(
    conn: &mut Connection,
    id: i64,
    content: &str,
    reason: &str,
    now: &str,
) -> ToolResult<String> {
    require_id(id)?;
    require_reason(reason)?;
    lkjagent_store::queue::edit(conn, id, content, reason, now)?;
    Ok(format!("edited id={id}"))
}

pub fn delete(conn: &mut Connection, id: i64, reason: &str, now: &str) -> ToolResult<String> {
    require_id(id)?;
    require_reason(reason)?;
    lkjagent_store::queue::delete(conn, id, reason, now)?;
    Ok(format!("deleted id={id}"))
}

pub fn redeliver(
    conn: &mut Connection,
    id: i64,
    content: Option<&str>,
    reason: &str,
    now: &str,
) -> ToolResult<String> {
    require_id(id)?;
    require_reason(reason)?;
    let new_id = lkjagent_store::queue::redeliver(conn, id, content, reason, now)?;
    Ok(format!("redelivered source_id={id}\nqueued id={new_id}"))
}

fn require_id(id: i64) -> ToolResult<()> {
    if id <= 0 {
        Err(ToolError::invalid("id must be positive"))
    } else {
        Ok(())
    }
}

fn require_content(content: &str) -> ToolResult<()> {
    if content.is_empty() {
        Err(ToolError::invalid("content must not be empty"))
    } else {
        Ok(())
    }
}

fn require_reason(reason: &str) -> ToolResult<()> {
    if reason.trim().is_empty() {
        Err(ToolError::invalid("reason must not be empty"))
    } else {
        Ok(())
    }
}

fn preview(content: &str) -> String {
    content.chars().take(80).collect()
}
