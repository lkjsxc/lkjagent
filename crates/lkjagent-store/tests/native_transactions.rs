use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::error::StoreError;
use lkjagent_store::transactions::{
    Cell, ContextRef, Decision, Effect, Intake, NativeStore, Obligation, Target,
};
use rusqlite::Connection;

fn path(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("native-tx-{label}-{nonce}.db")))
}

static OBLIGATIONS: [Obligation<'static>; 1] = [Obligation("o", "exact", b"predicate", true)];
static CELLS: [Cell<'static>; 1] = [Cell(b"case", b"objective", b"value", b"fp")];

#[rustfmt::skip]
fn intake() -> Intake<'static> {
    Intake { matter: "m", objective: b"objective", turn: "t", queue_sequence: 1,
        raw_text: b"owner", message_fingerprint: b"mf", event: "e1",
        event_sequence: 1, event_payload: b"intake", monotonic_ms: 1,
        wall_time: "now", obligations: &OBLIGATIONS, cells: &CELLS }
}

#[rustfmt::skip]
fn decision<'a>() -> Decision<'a> {
    Decision { id: "d", matter: "m", event: "e2", event_sequence: 2,
        event_payload: b"selected", operation: b"op", idempotency: b"decision-idem",
        monotonic_ms: 2, wall_time: "now", specs: [b"state", b"context", b"tool",
        b"grammar", b"budget", b"recovery", b"check", b"exit"] }
}

#[rustfmt::skip]
fn compile(store: &mut NativeStore) -> Result<(), StoreError> {
    store.attach_compilation("d", b"attachments", b"frame", b"context-fp", b"tool-fp", &[
        ContextRef { id: "ctx", source_kind: "owner", source_id: b"msg", revision: b"mf",
            semantic_key: b"objective", trust: "owner", body_ref: b"msg" },
    ])
}

#[test]
#[rustfmt::skip]
fn durable_boundaries_intake_is_atomic_and_idempotency_is_typed() -> Result<(), Box<dyn Error>> {
    let db = path("intake")?;
    let mut store = NativeStore::open(&db)?;
    let first = store.owner_intake(&intake())?;
    assert_eq!(first.id, "owner-turn/t");
    assert_eq!(first.sequence, 1);
    assert_eq!(store.owner_intake(&intake())?, first);
    let mut orphan = decision();
    orphan.id = "orphan";
    orphan.event = "orphan-event";
    orphan.matter = "missing";
    assert!(matches!(
        store.select_decision(&orphan),
        Err(StoreError::NotFound(_))
    ));
    drop(store);
    let mut reopened = NativeStore::open(&db)?;
    assert_eq!(reopened.owner_intake(&intake())?, first);
    let messages = lkjagent_store::native_schema::conversation(&Connection::open(&db)?, None, 10)?;
    assert_eq!((messages[0].id.as_str(), messages[0].sequence), ("owner-turn/t", 1));
    drop(reopened);
    let connection = Connection::open(&db)?;
    for table in [
        "matters",
        "owner_turns",
        "conversation_messages",
        "obligations",
        "runtime_events",
        "state_cells",
    ] {
        let count: i64 =
            connection.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))?;
        assert_eq!(count, 1, "{table}");
    }
    Ok(())
}

#[test]
fn durable_boundaries_injected_statement_failure_rolls_back_intake() -> Result<(), Box<dyn Error>> {
    let db = path("rollback")?;
    let mut store = NativeStore::open(&db)?;
    let injector = Connection::open(&db)?;
    injector.execute_batch("CREATE TRIGGER fail_message BEFORE INSERT ON conversation_messages BEGIN SELECT RAISE(ABORT,'injected'); END;")?;
    assert!(store.owner_intake(&intake()).is_err());
    for table in [
        "matters",
        "owner_turns",
        "runtime_events",
        "obligations",
        "state_cells",
    ] {
        let count: i64 =
            injector.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))?;
        assert_eq!(count, 0, "{table}");
    }
    Ok(())
}

#[test]
fn durable_boundaries_compiles_before_provider_and_sent_is_not_replayed(
) -> Result<(), Box<dyn Error>> {
    let db = path("provider")?;
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&intake())?;
    store.select_decision(&decision())?;
    assert!(matches!(
        store.provider_intent("p", "d", b"request", 3),
        Err(StoreError::InvalidState(_))
    ));
    compile(&mut store)?;
    store.provider_intent("p", "d", b"request", 3)?;
    assert!(matches!(
        store.provider_intent("p2", "d", b"other", 3),
        Err(StoreError::InvalidState(_))
    ));
    store.provider_phase("p", "intended", "sent")?;
    assert_eq!(store.ambiguous_providers()?, vec!["p"]);
    assert!(store.provider_phase("p", "intended", "sent").is_err());
    store.provider_phase("p", "sent", "ambiguous")?;
    assert!(store
        .provider_outcome(
            "p",
            "succeeded",
            b"response",
            (1, 1),
            4,
            b"stop",
            b"",
            b"ok"
        )
        .is_err());
    assert!(store.ambiguous_providers()?.is_empty());
    Ok(())
}

#[test]
fn durable_boundaries_effect_prep_is_complete_and_phases_do_not_skip() -> Result<(), Box<dyn Error>>
{
    let db = path("effect")?;
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&intake())?;
    store.select_decision(&decision())?;
    compile(&mut store)?;
    let target = Target {
        path: b"notes/a",
        prior: None,
        intended: Some(b"new"),
        operation: "create",
        prior_mode: None,
        intended_mode: Some(0o644),
        stage_identity: b"stage",
    };
    let mut effect = Effect {
        admission: "a",
        journal: "j",
        decision: "d",
        action_ordinal: 0,
        action_fingerprint: b"action",
        reason: b"accepted",
        parsed_call: b"call",
        tool_spec: b"tool",
        idempotency: b"effect-idem",
        intended_fingerprint: b"intended",
        prior_fingerprint: None,
        targets: &[],
    };
    assert!(store.prepare_effect(&effect).is_err());
    effect.targets = std::slice::from_ref(&target);
    store.prepare_effect(&effect)?;
    assert!(store.effect_phase("j", "prepared", "exchanging").is_err());
    for (old, new) in [
        ("prepared", "staging"),
        ("staging", "exchange-ready"),
        ("exchange-ready", "exchanging"),
        ("exchanging", "exchanged"),
        ("exchanged", "observing"),
    ] {
        store.effect_phase("j", old, new)?;
    }
    drop(store);
    let connection = Connection::open(&db)?;
    let row: (String, i64, Vec<u8>, Option<i64>) = connection.query_row(
        "SELECT j.status,count(t.ordinal),t.intended_bytes,t.intended_mode FROM effect_journal j JOIN effect_targets t ON t.journal_id=j.id WHERE j.id='j'",
        [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
    assert_eq!(row, ("observing".into(), 1, b"new".to_vec(), Some(0o644)));
    Ok(())
}
