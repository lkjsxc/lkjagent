use lkjagent_core::model::TaskSnapshot;
use rusqlite::Connection;

use lkjagent_store::error::StoreResult;
use lkjagent_store::plan_hydrate::active_snapshot;

pub fn load_snapshot(conn: &Connection) -> StoreResult<Option<TaskSnapshot>> {
    active_snapshot(conn)
}
