use std::path::Path;

use crate::error::CliError;
use crate::store::open_store;

pub fn memory(data_dir: &Path, query: &str) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let rows = lkjagent_store::memory::find(&conn, query, 5)?;
    let lines = rows
        .iter()
        .map(|row| {
            format!(
                "id={} kind={} title={} tags={}",
                row.id, row.kind, row.title, row.tags
            )
        })
        .collect::<Vec<_>>();
    if lines.is_empty() {
        Ok("memory_results=0".to_string())
    } else {
        Ok(lines.join("\n"))
    }
}
