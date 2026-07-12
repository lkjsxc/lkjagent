use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::transactions::{
    Decision, Effect, Intake, NativeStore, Obligation, Settlement, Target,
};
use rusqlite::Connection;

static OBLIGATIONS: [Obligation<'static>; 1] = [Obligation("o", "exact", b"p", true)];

fn path() -> Result<PathBuf, Box<dyn Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("native-settlement-{nonce}.db")))
}

fn intake() -> Intake<'static> {
    Intake {
        matter: "m",
        objective: b"objective",
        turn: "t",
        queue_sequence: 1,
        raw_text: b"owner",
        message: "owner-msg",
        message_sequence: 1,
        message_fingerprint: b"owner-fp",
        event: "e1",
        event_sequence: 1,
        event_payload: b"intake",
        monotonic_ms: 1,
        wall_time: "now",
        obligations: &OBLIGATIONS,
        cells: &[],
    }
}

fn decision() -> Decision<'static> {
    Decision {
        id: "d",
        matter: "m",
        event: "e2",
        event_sequence: 2,
        event_payload: b"selection",
        operation: b"op",
        idempotency: b"decision-idem",
        monotonic_ms: 2,
        wall_time: "now",
        specs: [
            b"state",
            b"context",
            b"tool",
            b"grammar",
            b"budget",
            b"recovery",
            b"check",
            b"exit",
        ],
    }
}

#[test]
fn durable_boundaries_settlement_links_revision_and_close_guards() -> Result<(), Box<dyn Error>> {
    let db = path()?;
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&intake())?;
    store.select_decision(&decision())?;
    store.attach_compilation("d", b"attachments", b"frame", b"context", b"tool", &[])?;
    let target = Target {
        path: b"notes/a",
        prior: None,
        intended: Some(b"new"),
        operation: "create",
        prior_mode: None,
        intended_mode: Some(0o644),
        stage_identity: b"stage",
    };
    store.prepare_effect(&Effect {
        admission: "a",
        journal: "j",
        decision: "d",
        action_ordinal: 0,
        action_fingerprint: b"action",
        reason: b"ok",
        parsed_call: b"call",
        tool_spec: b"tool",
        idempotency: b"effect-idem",
        intended_fingerprint: b"new-fp",
        prior_fingerprint: None,
        targets: &[target],
    })?;
    assert!(store
        .close_matter(
            "m",
            "final",
            2,
            b"done",
            b"final-fp",
            "close",
            4,
            4,
            "now",
            b"close"
        )
        .is_err());
    for (old, new) in [
        ("prepared", "staging"),
        ("staging", "exchange-ready"),
        ("exchange-ready", "exchanging"),
        ("exchanging", "exchanged"),
        ("exchanged", "observing"),
    ] {
        store.effect_phase("j", old, new)?;
    }
    store.settle_effect(&Settlement {
        journal: "j",
        observation: "obs",
        event: "e3",
        matter: "m",
        event_sequence: 3,
        monotonic_ms: 3,
        wall_time: "now",
        event_payload: b"observed",
        status: "succeeded",
        outcome: b"written",
        content_ref: b"rev",
        fingerprint: b"new-fp",
        document: "doc",
        path: b"notes/a",
        revision: "rev",
        parent: None,
        sha256: &[7; 32],
        content: b"new",
    })?;
    let connection = Connection::open(&db)?;
    connection.execute("INSERT INTO checks(id,matter_id,obligation_id,decision_id,kind,parameters,current,passed,measured,evidence_fingerprint,source_revision,checked_event_id) VALUES('failed','m','o','d','exact',x'01',1,0,x'00',x'01',x'01','e3')", [])?;
    connection.execute(
        "UPDATE obligations SET current_check_id='failed' WHERE id='o'",
        [],
    )?;
    assert!(store
        .close_matter(
            "m",
            "final",
            2,
            b"done",
            b"final-fp",
            "close",
            4,
            4,
            "now",
            b"close"
        )
        .is_err());
    connection.execute("UPDATE checks SET current=0 WHERE id='failed'", [])?;
    connection.execute("INSERT INTO checks(id,matter_id,obligation_id,decision_id,kind,parameters,current,passed,measured,evidence_fingerprint,source_revision,checked_event_id) VALUES('passed','m','o','d','exact',x'01',1,1,x'01',x'02',x'02','e3')", [])?;
    connection.execute(
        "UPDATE obligations SET status='passed',current_check_id='passed' WHERE id='o'",
        [],
    )?;
    connection.execute_batch("CREATE TRIGGER fail_final BEFORE INSERT ON conversation_messages WHEN NEW.role='agent' BEGIN SELECT RAISE(ABORT,'injected final'); END;")?;
    assert!(store
        .close_matter(
            "m",
            "final",
            2,
            b"done",
            b"final-fp",
            "close",
            4,
            4,
            "now",
            b"close"
        )
        .is_err());
    let state: (String, i64) = connection.query_row("SELECT lifecycle,(SELECT count(*) FROM runtime_events WHERE id='close') FROM matters WHERE id='m'", [], |r| Ok((r.get(0)?, r.get(1)?)))?;
    assert_eq!(state, ("open".into(), 0));
    connection.execute_batch("DROP TRIGGER fail_final")?;
    store.close_matter(
        "m",
        "final",
        2,
        b"done",
        b"final-fp",
        "close",
        4,
        4,
        "now",
        b"close",
    )?;
    let linked: (String, String, String, String) = connection.query_row(
        "SELECT j.status,o.event_id,r.effect_id,d.status FROM effect_journal j JOIN observations o ON o.id=j.observation_id JOIN workspace_revisions r ON r.effect_id=j.id JOIN runtime_decisions d ON d.id=j.decision_id",
        [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
    assert_eq!(
        linked,
        ("settled".into(), "e3".into(), "j".into(), "settled".into())
    );
    let closed: (String, i64, i64) = connection.query_row("SELECT lifecycle,(SELECT count(*) FROM conversation_messages WHERE id='final'),(SELECT count(*) FROM runtime_events WHERE id='close') FROM matters WHERE id='m'", [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    assert_eq!(closed, ("closed".into(), 1, 1));
    Ok(())
}
