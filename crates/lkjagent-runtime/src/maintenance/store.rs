use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::maintenance::{idle_boundary, BoundaryDecision, DirectiveStamp, MaintenanceDirective};
use crate::task::RuntimeState;

const MIN_MAINTENANCE_INTERVAL_SECONDS: u64 = 60;

pub fn prepare_idle_cycle(
    conn: &Connection,
    state: &RuntimeState,
    now: &str,
) -> RuntimeResult<BoundaryDecision> {
    let pending_queue = lkjagent_store::queue::pending_count(conn)?;
    let stamps = load_directive_stamps(conn)?;
    let decision = idle_boundary(state, pending_queue, &stamps);
    if let BoundaryDecision::StartCycle { directive, .. } = &decision {
        if !cycle_due(*directive, &stamps, now) {
            return Ok(BoundaryDecision::NotIdle);
        }
        stamp_directive(conn, *directive, now)?;
    }
    Ok(decision)
}

pub fn maintenance_due(conn: &Connection, state: &RuntimeState, now: &str) -> RuntimeResult<bool> {
    let pending_queue = lkjagent_store::queue::pending_count(conn)?;
    let stamps = load_directive_stamps(conn)?;
    Ok(match idle_boundary(state, pending_queue, &stamps) {
        BoundaryDecision::StartCycle { directive, .. } => cycle_due(directive, &stamps, now),
        BoundaryDecision::ContinueCycle { .. } => true,
        BoundaryDecision::PreemptForQueue { .. } | BoundaryDecision::NotIdle => false,
    })
}

pub fn load_directive_stamps(conn: &Connection) -> RuntimeResult<Vec<DirectiveStamp>> {
    let mut stamps = Vec::new();
    for directive in MaintenanceDirective::all() {
        stamps.push(DirectiveStamp {
            directive: *directive,
            last_run: lkjagent_store::state::maintenance_stamp(conn, directive.as_str())?,
        });
    }
    Ok(stamps)
}

pub fn stamp_directive(
    conn: &Connection,
    directive: MaintenanceDirective,
    now: &str,
) -> RuntimeResult<()> {
    Ok(lkjagent_store::state::set_maintenance_stamp(
        conn,
        directive.as_str(),
        now,
    )?)
}

pub fn defer_all_directives(conn: &Connection, now: &str) -> RuntimeResult<()> {
    for directive in MaintenanceDirective::all() {
        stamp_directive(conn, *directive, now)?;
    }
    Ok(())
}

fn cycle_due(directive: MaintenanceDirective, stamps: &[DirectiveStamp], now: &str) -> bool {
    let Some(last_run) = stamps
        .iter()
        .find(|stamp| stamp.directive == directive)
        .and_then(|stamp| stamp.last_run.as_deref())
    else {
        return true;
    };
    match (last_run.parse::<u64>(), now.parse::<u64>()) {
        (Ok(last), Ok(current)) => current.saturating_sub(last) >= MIN_MAINTENANCE_INTERVAL_SECONDS,
        _ => last_run < now,
    }
}
