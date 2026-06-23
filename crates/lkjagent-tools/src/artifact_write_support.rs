use lkjagent_store::artifact_cursor::mark_paths_completed;
use rusqlite::Connection;

use crate::error::ToolResult;

pub fn record_written_paths(conn: &Connection, paths: &[String], now: &str) -> ToolResult<()> {
    mark_paths_completed(conn, paths, now)?;
    Ok(())
}
