mod admission_bridge;
mod arg_helpers;
pub mod args;
mod artifact_effects;
pub mod cli;
pub mod clock;
pub mod config;
pub mod console;
pub mod state {
    use lkjagent_core::model::TaskSnapshot;
    use lkjagent_store::error::{StoreError, StoreResult};
    use lkjagent_store::plan_hydrate::active_snapshot;
    use rusqlite::Connection;

    pub fn load_snapshot(conn: &Connection) -> StoreResult<Option<TaskSnapshot>> {
        match crate::snapshot_state::load_snapshot_cell(conn) {
            Ok(Some(snapshot)) => Ok(Some(snapshot)),
            Ok(None) => active_snapshot(conn),
            Err(error) => Err(StoreError::Sql(error)),
        }
    }
}
mod context_admin;
mod context_bridge;
mod context_resolution_bridge;
pub mod daemon;
mod daemon_intake;
mod daemon_lock;
mod daemon_owner_routes;
mod daemon_route_effects;
mod diagnostics;
mod diagnostics_support;
mod effect_error;
pub mod endpoint;
mod exchange_bridge;
mod exchange_record;
mod explore;
mod inspect;
mod lease_status;
mod log_view;
mod model_call;
mod model_io;
mod observation_bridge;
mod prompt_bridge;
mod record_args;
mod record_files;
mod record_identity;
mod record_state;
mod recovery_bridge;
mod runtime_bridge;
mod runtime_cell;
mod runtime_projection;
mod snapshot_state;
pub mod status;
mod task_view;
mod token_status;
pub mod tui_event {
    pub use crate::tui_state::TuiEvent;
}
mod tui_keys;
mod tui_palette;
mod tui_reduce;
pub mod tui_render;
pub mod tui_snapshot;
pub mod tui_state {
    pub use crate::tui_reduce::reduce;
    pub use crate::tui_types::*;
}
mod tui_terminal;
pub mod tui_transcript;
mod tui_types;
mod tui_view;
mod tui_window;
mod turn_effects;
mod watch_view;
pub mod workbench;
mod workbench_commands;
mod workbench_line;
mod workbench_render;
pub mod workbench_state;
mod workspace_index;
mod workspace_rebalance;
mod workspace_scaffold;
