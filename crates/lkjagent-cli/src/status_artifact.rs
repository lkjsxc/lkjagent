use rusqlite::Connection;

use super::{fact, StatusFact};
use crate::error::CliError;

pub fn push_progress(conn: &Connection, out: &mut Vec<StatusFact>) -> Result<(), CliError> {
    let Some(case) = lkjagent_store::graph::active_case(conn)? else {
        return push_empty(out);
    };
    let Some(row) = lkjagent_store::artifact_graph::readiness_for_case(conn, case.id)? else {
        return push_empty(out);
    };
    for (key, value) in [
        ("artifact.profile", row.profile),
        ("artifact.plan_status", row.plan_status),
        ("artifact.atom_total", row.atom_total.to_string()),
        ("artifact.atom_ready", row.atom_ready.to_string()),
        ("artifact.atom_missing", row.atom_missing.to_string()),
        ("artifact.next_atom", row.next_atom_id),
        ("artifact.next_path", row.next_path),
        ("artifact.active_contract", row.active_contract_id),
        ("artifact.measured_total", row.measured_total.to_string()),
        ("artifact.accepted_floor", row.accepted_floor.to_string()),
        ("artifact.assembly_pending", row.assembly_pending),
        ("artifact.readiness", row.status),
        (
            "artifact.completion_blockers",
            row.completion_blockers.replace('\n', ";"),
        ),
    ] {
        out.push(fact(key, value));
    }
    Ok(())
}

fn push_empty(out: &mut Vec<StatusFact>) -> Result<(), CliError> {
    for key in [
        "artifact.profile",
        "artifact.plan_status",
        "artifact.atom_total",
        "artifact.atom_ready",
        "artifact.atom_missing",
        "artifact.next_atom",
        "artifact.next_path",
        "artifact.active_contract",
        "artifact.measured_total",
        "artifact.accepted_floor",
        "artifact.assembly_pending",
        "artifact.readiness",
        "artifact.completion_blockers",
    ] {
        out.push(fact(key, "none"));
    }
    Ok(())
}
