use lkjagent_store::transactions::{Decision, Intake, NativeStore};
use rusqlite::Connection;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

type TestResult = Result<(), Box<dyn Error>>;

fn path() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let n = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("native-budget-epoch-{n}.db")))
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

fn compile(store: &mut NativeStore, id: &str) -> TestResult {
    store.attach_compilation(id, b"attachments", b"frame", b"context", b"tool", &[])?;
    Ok(())
}

#[test]
#[rustfmt::skip]
fn owner_resume_starts_a_fresh_model_budget_epoch() -> TestResult {
    let mut store=NativeStore::open(path()?)?;store.owner_intake(&intake())?;
    store.select_decision(&decision("d1","e2",2))?;compile(&mut store,"d1")?;
    store.provider_intent("p1","d1",b"request",2)?;
    assert_eq!(store.provider_exchanges_in_budget_epoch("m")?,1);
    store.block_budget("m","blocked",3,3,"now",b"used=1 limit=1",b"block")?;
    let resumed=Intake{matter:"m",objective:b"corrected",turn:"t2",queue_sequence:2,
        raw_text:b"resume",message_fingerprint:b"resume-fp",event:"e4",event_sequence:4,
        event_payload:b"resume",monotonic_ms:4,wall_time:"now",obligations:&[],cells:&[]};
    store.resume_blocked(&resumed)?;
    assert_eq!(store.provider_exchanges_in_budget_epoch("m")?,0);
    store.select_decision(&decision("d2","e5",5))?;compile(&mut store,"d2")?;
    store.provider_intent("p2","d2",b"request",5)?;
    assert_eq!(store.provider_exchanges_in_budget_epoch("m")?,1);
    Ok(())
}

#[test]
fn missing_provider_usage_remains_unknown() -> TestResult {
    let db = path()?;
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&intake())?;
    store.select_decision(&decision("d1", "e2", 2))?;
    compile(&mut store, "d1")?;
    store.provider_intent("p1", "d1", b"request", 2)?;
    store.provider_phase("p1", "intended", "sent")?;
    store.provider_outcome(
        "p1",
        "failed",
        b"transport",
        (None, None),
        3,
        b"error",
        b"endpoint",
        b"not-parsed",
    )?;
    drop(store);
    let connection = Connection::open(db)?;
    let usage: (Option<i64>, Option<i64>) = connection.query_row(
        "SELECT input_tokens,output_tokens FROM provider_exchanges WHERE id='p1'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(usage, (None, None));
    Ok(())
}
