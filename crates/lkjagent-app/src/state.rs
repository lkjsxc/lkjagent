use lkjagent_core::model::TaskSnapshot;
use rusqlite::Connection;

use lkjagent_store::error::{StoreError, StoreResult};
use lkjagent_store::plan_turn::{config, set_config};

const ACTIVE_SNAPSHOT: &str = "app.active-snapshot";

pub fn save_snapshot(conn: &Connection, snapshot: &TaskSnapshot) -> StoreResult<()> {
    let json =
        serde_json::to_string(snapshot).map_err(|error| StoreError::Sql(error.to_string()))?;
    set_config(conn, ACTIVE_SNAPSHOT, &json)
}

pub fn load_snapshot(conn: &Connection) -> StoreResult<Option<TaskSnapshot>> {
    let Some(json) = config(conn, ACTIVE_SNAPSHOT)? else {
        return Ok(None);
    };
    let snapshot =
        serde_json::from_str(&json).map_err(|error| StoreError::Sql(error.to_string()))?;
    Ok(Some(snapshot))
}
