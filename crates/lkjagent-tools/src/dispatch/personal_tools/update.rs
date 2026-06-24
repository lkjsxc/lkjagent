use std::collections::BTreeMap;

use lkjagent_store::personal::update_status;
use rusqlite::Connection;

use crate::dispatch::params::{param, parse_i64};
use crate::dispatch::ToolRuntime;
use crate::error::{ToolError, ToolResult};

pub fn status(
    conn: &Connection,
    params: &BTreeMap<String, String>,
    runtime: &ToolRuntime,
) -> ToolResult<String> {
    let id = parse_i64(&param(params, "id"))?;
    let Some(status) = params
        .get("status")
        .filter(|value| !value.trim().is_empty())
    else {
        return Err(ToolError::invalid("update currently requires status"));
    };
    update_status(conn, id, status, &runtime.now)?;
    Ok(format!("personal_record_updated\nid={id}\nstatus={status}"))
}
