use lkjagent_store::transactions::{
    Cell, ContextRef, Decision, Effect, Intake, NativeStore, Settlement, Target,
};
use std::{
    error::Error,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

type T<T = ()> = Result<T, Box<dyn Error>>;
static CELLS: [Cell<'static>; 1] = [Cell(b"matter", b"opened", b"{}", b"fp")];

#[test]
#[rustfmt::skip]
fn exact_effect_accepts_declared_mkdir_and_settled_retry_is_honest()->T{
 let db=path()?;let mut store=NativeStore::open(&db)?;store.owner_intake(&Intake{matter:"m",objective:b"journal",turn:"t",queue_sequence:1,raw_text:b"owner",message_fingerprint:b"owner-fp",event:"e1",event_sequence:1,event_payload:b"intake",monotonic_ms:1,wall_time:"2026-07-13T00:00:00Z",obligations:&[],cells:&CELLS})?;
 store.select_decision(&Decision{id:"d",matter:"m",event:"e2",event_sequence:2,event_payload:b"selected",operation:b"orient",idempotency:b"did",monotonic_ms:2,wall_time:"2026-07-13T00:00:00Z",specs:[b"state",b"context",b"tool",b"grammar",b"budget",b"recovery",b"checks",b"exit"]})?;
 store.attach_compilation("d",b"attachments",b"frame",b"context-fp",b"tool-fp",&[ContextRef{id:"ctx",source_kind:"owner",source_id:b"owner-turn/t",revision:b"owner-fp",semantic_key:b"objective",trust:"owner",body_ref:b"owner"}])?;
 let targets=[Target{path:b"life/journal/2026/07/13/entry.md",prior:None,intended:Some(b"new"),operation:"create",prior_mode:None,intended_mode:Some(0o644),stage_identity:b"stage"},Target{path:b"life",prior:None,intended:None,operation:"mkdir",prior_mode:None,intended_mode:Some(0o755),stage_identity:b"mkdir:life"}];
 let invalid=[Target{path:b"life",prior:None,intended:None,operation:"mkdir",prior_mode:None,intended_mode:Some(0o755),stage_identity:b"mkdir:life"},Target{path:b"life/entry.md",prior:None,intended:Some(b"new"),operation:"create",prior_mode:None,intended_mode:Some(0o644),stage_identity:b"stage"}];let rejected=Effect{admission:"bad-a",journal:"bad-j",decision:"d",action_ordinal:0,action_fingerprint:b"bad",reason:b"workspace.create",parsed_call:b"bad",tool_spec:b"tool",idempotency:b"bad-idem",intended_fingerprint:b"intended",prior_fingerprint:None,targets:&invalid};assert!(store.prepare_exact_effect(&rejected).is_err());
 let effect=Effect{admission:"a",journal:"j",decision:"d",action_ordinal:0,action_fingerprint:b"action",reason:b"workspace.create",parsed_call:b"call",tool_spec:b"tool",idempotency:b"idem",intended_fingerprint:b"intended",prior_fingerprint:None,targets:&targets};store.prepare_exact_effect(&effect)?;
 for (old,new) in [("prepared","staging"),("staging","exchange-ready"),("exchange-ready","exchanging"),("exchanging","exchanged"),("exchanged","observing")]{store.effect_phase("j",old,new)?;}
 store.settle_effect(&Settlement{journal:"j",observation:"o",event:"e3",matter:"m",event_sequence:3,monotonic_ms:3,wall_time:"2026-07-13T00:00:01Z",event_payload:b"effect",status:"succeeded",outcome:b"done",content_ref:b"r",fingerprint:b"intended",document:"doc",path:targets[0].path,revision:"r",parent:None,sha256:&[1;32],content:b"new"})?;
 store.prepare_exact_effect(&effect)?;let changed=Effect{intended_fingerprint:b"changed",..effect};assert!(store.prepare_exact_effect(&changed).is_err());let connection=rusqlite::Connection::open(&db)?;let row:(i64,i64,String)=connection.query_row("SELECT (SELECT count(*) FROM effect_journal),(SELECT count(*) FROM effect_targets),status FROM effect_journal WHERE id='j'",[],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?)))?;assert_eq!(row,(1,2,"settled".into()));Ok(())
}

fn path() -> T<PathBuf> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("journal-effect-{nonce}.db")))
}
