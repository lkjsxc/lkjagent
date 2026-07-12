mod admission_bridge;
pub use admission_bridge::persist_tool_admissions;
pub mod args;
mod artifact_effects;
mod artifact_plan;
pub mod automatic_checks;
pub mod cli;
pub mod clock;
#[allow(dead_code)]
pub mod config;
mod config_registry;
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
mod context_bridge;
#[allow(dead_code)]
mod context_resolution_bridge;
pub mod daemon;
mod daemon_intake;
#[allow(dead_code)]
mod daemon_lock;
#[allow(dead_code)]
mod daemon_owner_routes;
mod daemon_route_effects;
#[allow(dead_code)]
mod diagnostics;
pub mod effect_dispatch;
mod effect_files;
mod endpoint_recovery;
mod exchange_bridge;
mod exchange_record;
mod explore;
#[allow(dead_code)]
mod inspect;
mod lease_status;
#[allow(dead_code)]
mod log_view;
mod model_call;
mod model_io;
mod observation_bridge;
pub mod progress_bridge;
pub mod public_loop;
mod record_archive;
#[allow(dead_code)]
mod record_files;
mod record_identity;
mod record_state;
mod recovery_bridge;
mod runtime_bridge;
pub mod runtime_budget;
mod runtime_cell;
mod runtime_projection;
mod snapshot_state;
pub mod status;
#[allow(dead_code)]
mod task_view;
pub mod tui_event {
    pub use crate::tui_state::TuiEvent;
}

pub mod endpoint {
    pub use crate::model_io::{CompletionRecord, Endpoint, LlmEndpoint, ScriptedEndpoint};
}
mod tui_keys;
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
pub mod turn_effects;
pub mod workbench;
mod workbench_line;
mod workbench_render;
pub mod workbench_state;
mod workspace_index;
#[allow(dead_code)]
mod workspace_rebalance;
#[allow(dead_code)]
mod workspace_rebalance_apply;
#[allow(dead_code)]
mod workspace_rebalance_group;
pub mod workspace_root;
mod workspace_scan;
mod workspace_search;
