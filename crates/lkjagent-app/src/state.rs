use lkjagent_core::model::TaskSnapshot;
use rusqlite::Connection;

use lkjagent_store::error::StoreResult;
use lkjagent_store::plan_hydrate::active_snapshot;

use crate::snapshot_state::load_snapshot_cell;

pub fn load_snapshot(conn: &Connection) -> StoreResult<Option<TaskSnapshot>> {
    match load_snapshot_cell(conn) {
        Ok(Some(snapshot)) => Ok(Some(snapshot)),
        Ok(None) => active_snapshot(conn),
        Err(error) => Err(lkjagent_store::error::StoreError::Sql(error)),
    }
}
