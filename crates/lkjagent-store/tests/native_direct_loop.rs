use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::direct_transactions::{
    DirectSettlement, DirectTool, ModelFault, ModelFaultKind,
};
use lkjagent_store::transactions::{Decision, Effect, Intake, NativeStore, Target};
use rusqlite::Connection;

fn path(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let n = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("native-direct-{label}-{n}.db")))
}
#[rustfmt::skip]
fn intake() -> Intake<'static> {
    Intake { matter:"m", objective:b"objective", turn:"t", queue_sequence:1,
        raw_text:b"owner", message_fingerprint:b"owner-fp", event:"e1", event_sequence:1,
        event_payload:b"intake", monotonic_ms:1, wall_time:"now", obligations:&[], cells:&[] }
}
#[rustfmt::skip]
fn decision(id: &'static str, event: &'static str, sequence: i64) -> Decision<'static> {
    Decision { id, matter:"m", event, event_sequence:sequence, event_payload:b"selected",
        operation:b"repeatable-read", idempotency:id.as_bytes(), monotonic_ms:sequence,
        wall_time:"now", specs:[b"state",b"context",b"tool",b"grammar",b"budget",b"recovery",b"check",b"exit"] }
}
fn compile(store: &mut NativeStore, id: &str) -> Result<(), Box<dyn Error>> {
    store.attach_compilation(id, b"attachments", b"frame", b"context", b"tool", &[])?;
    Ok(())
}
#[rustfmt::skip]
fn direct() -> DirectSettlement<'static> {
    DirectSettlement { decision:"d1", matter:"m", admission:"a1", action_ordinal:0,
        action_fingerprint:b"action", parsed_call:b"read notes/a", tool_spec:b"read-v1",
        tool:DirectTool::Read, observation:"o1", outcome:b"bytes", content_ref:b"cache/1",
        fingerprint:b"observed-fp", event:"e3", event_sequence:3, monotonic_ms:3,
        wall_time:"now", event_payload:b"direct", namespace:b"source", cell_key:b"notes/a",
        source_revision:b"rev-1", bytes_ref:b"cache/1" }
}

#[test]
fn direct_settlement_is_atomic_idempotent_and_reopens() -> Result<(), Box<dyn Error>> {
    let db = path("settle")?;
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&intake())?;
    store.select_decision(&decision("d1", "e2", 2))?;
    compile(&mut store, "d1")?;
    store.settle_direct(&direct())?;
    store.settle_direct(&direct())?;
    let mut conflict = direct();
    conflict.outcome = b"changed";
    assert!(store.settle_direct(&conflict).is_err());
    drop(store);
    let reopened = NativeStore::open(&db)?;
    let projection = reopened.restart_projection()?;
    assert_eq!(projection.matter.expect("matter").id, "m");
    assert_eq!(projection.cells.len(), 1);
    assert!(projection.decisions.is_empty());
    let connection = Connection::open(&db)?;
    let counts: (i64, i64, i64) = connection.query_row(
        "SELECT (SELECT count(*) FROM observations),(SELECT count(*) FROM tool_admissions),(SELECT count(*) FROM effect_journal)",
        [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    assert_eq!(counts, (1, 1, 0));
    Ok(())
}

#[test]
fn model_fault_has_no_admission_and_repeated_operation_is_allowed() -> Result<(), Box<dyn Error>> {
    let db = path("fault")?;
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&intake())?;
    store.select_decision(&decision("d1", "e2", 2))?;
    store.reject_model_output(&ModelFault {
        decision: "d1",
        matter: "m",
        event: "e3",
        event_sequence: 3,
        monotonic_ms: 3,
        wall_time: "now",
        event_payload: b"malformed",
        fault_kind: ModelFaultKind::Malformed,
        recovery_ref: b"retry-bounded",
        fingerprint: b"fault-fp",
    })?;
    store.select_decision(&decision("d2", "e4", 4))?;
    let connection = Connection::open(&db)?;
    let rows: (i64, i64, String) = connection.query_row(
        "SELECT (SELECT count(*) FROM tool_admissions),(SELECT count(*) FROM effect_journal),status FROM runtime_decisions WHERE id='d1'",
        [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    assert_eq!(rows, (0, 0, "failed".into()));
    Ok(())
}

#[test]
fn exact_prepare_derives_open_checks_without_passing_them() -> Result<(), Box<dyn Error>> {
    let db = path("prepare")?;
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&intake())?;
    store.select_decision(&decision("d1", "e2", 2))?;
    compile(&mut store, "d1")?;
    let target = Target {
        path: b"notes/a",
        prior: None,
        intended: Some(b"new"),
        operation: "create",
        prior_mode: None,
        intended_mode: Some(0o644),
        stage_identity: b"stage",
    };
    let effect = Effect {
        admission: "a",
        journal: "j",
        decision: "d1",
        action_ordinal: 0,
        action_fingerprint: b"action",
        reason: b"accepted",
        parsed_call: b"create notes/a",
        tool_spec: b"write-v1",
        idempotency: b"effect-idem",
        intended_fingerprint: b"new-revision",
        prior_fingerprint: None,
        targets: &[target],
    };
    store.prepare_exact_effect(&effect)?;
    store.prepare_exact_effect(&effect)?;
    let projection = store.restart_projection()?;
    assert!(!projection.checks_ready);
    assert_eq!(projection.effects[0].status, "prepared");
    let connection = Connection::open(&db)?;
    let rows: (i64, i64) = connection.query_row(
        "SELECT count(*),sum(status='passed') FROM obligations WHERE matter_id='m'",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    assert_eq!(rows, (3, 0));
    Ok(())
}
