use std::path::Path;

use rusqlite::{params, Connection, Transaction, TransactionBehavior};

use crate::error::{StoreError, StoreResult};
use crate::native_schema;

pub struct NativeStore {
    connection: Connection,
}

#[rustfmt::skip]
pub struct Intake<'a> {
    pub matter: &'a str, pub objective: &'a [u8], pub turn: &'a str,
    pub queue_sequence: i64, pub raw_text: &'a [u8], pub message: &'a str,
    pub message_sequence: i64, pub message_fingerprint: &'a [u8], pub event: &'a str,
    pub event_sequence: i64, pub event_payload: &'a [u8], pub monotonic_ms: i64,
    pub wall_time: &'a str, pub obligations: &'a [Obligation<'a>], pub cells: &'a [Cell<'a>],
}
pub struct Obligation<'a>(pub &'a str, pub &'a str, pub &'a [u8], pub bool);
pub struct Cell<'a>(pub &'a [u8], pub &'a [u8], pub &'a [u8], pub &'a [u8]);

#[rustfmt::skip]
pub struct Decision<'a> {
    pub id: &'a str, pub matter: &'a str, pub event: &'a str, pub event_sequence: i64,
    pub event_payload: &'a [u8], pub operation: &'a [u8], pub idempotency: &'a [u8],
    pub monotonic_ms: i64, pub wall_time: &'a str, pub specs: [&'a [u8]; 8],
}
#[rustfmt::skip]
pub struct ContextRef<'a> {
    pub id: &'a str, pub source_kind: &'a str, pub source_id: &'a [u8],
    pub revision: &'a [u8], pub semantic_key: &'a [u8], pub trust: &'a str, pub body_ref: &'a [u8],
}
#[rustfmt::skip]
pub struct Effect<'a> {
    pub admission: &'a str, pub journal: &'a str, pub decision: &'a str,
    pub action_ordinal: i64, pub action_fingerprint: &'a [u8], pub reason: &'a [u8],
    pub parsed_call: &'a [u8], pub tool_spec: &'a [u8], pub idempotency: &'a [u8],
    pub intended_fingerprint: &'a [u8], pub prior_fingerprint: Option<&'a [u8]>,
    pub targets: &'a [Target<'a>],
}
#[rustfmt::skip]
pub struct Target<'a> {
    pub path: &'a [u8], pub prior: Option<&'a [u8]>, pub intended: Option<&'a [u8]>,
    pub operation: &'a str, pub prior_mode: Option<i64>, pub intended_mode: Option<i64>,
    pub stage_identity: &'a [u8],
}
#[rustfmt::skip]
pub struct Settlement<'a> {
    pub journal: &'a str, pub observation: &'a str, pub event: &'a str, pub matter: &'a str,
    pub event_sequence: i64, pub monotonic_ms: i64, pub wall_time: &'a str,
    pub event_payload: &'a [u8], pub status: &'a str, pub outcome: &'a [u8],
    pub content_ref: &'a [u8], pub fingerprint: &'a [u8], pub document: &'a str,
    pub path: &'a [u8], pub revision: &'a str, pub parent: Option<&'a str>,
    pub sha256: &'a [u8], pub content: &'a [u8],
}

#[rustfmt::skip]
impl NativeStore {
    pub fn open(path: impl AsRef<Path>) -> StoreResult<Self> {
        Ok(Self {
            connection: native_schema::open(path)?,
        })
    }

    pub fn owner_intake(&mut self, value: &Intake<'_>) -> StoreResult<()> {
        self.atomic(|tx| {
            tx.execute("INSERT INTO matters(id,objective,lifecycle,priority,created_sequence,updated_sequence) VALUES(?1,?2,'open',0,?3,?3)", params![value.matter,value.objective,value.event_sequence])?;
            tx.execute("INSERT INTO owner_turns(id,queue_sequence,raw_text,delivery,matter_id,created_at) VALUES(?1,?2,?3,'delivered',?4,?5)", params![value.turn,value.queue_sequence,value.raw_text,value.matter,value.wall_time])?;
            tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'owner-intake',?4,?5,?6,'owner-turn',?7)", params![value.event,value.matter,value.event_sequence,value.monotonic_ms,value.wall_time,value.event_payload,value.turn])?;
            tx.execute("INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,lifecycle,matter_id,owner_turn_id,cause_event_id) VALUES(?1,?2,'owner',?3,?4,'active',?5,?6,?7)", params![value.message,value.message_sequence,value.raw_text,value.message_fingerprint,value.matter,value.turn,value.event])?;
            for row in value.obligations { tx.execute("INSERT INTO obligations(id,matter_id,predicate_kind,predicate_payload,required,status) VALUES(?1,?2,?3,?4,?5,'open')", params![row.0,value.matter,row.1,row.2,i64::from(row.3)])?; }
            for row in value.cells { tx.execute("INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,?2,?3,?4,'active',?5,?6)", params![value.matter,row.0,row.1,row.2,value.event,row.3])?; }
            Ok(())
        })
    }

    pub fn select_decision(&mut self, value: &Decision<'_>) -> StoreResult<()> {
        self.atomic(|tx| {
            tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'decision-selected',?4,?5,?6,'harness',?7)", params![value.event,value.matter,value.event_sequence,value.monotonic_ms,value.wall_time,value.event_payload,value.id])?;
            tx.execute("INSERT INTO runtime_decisions(id,matter_id,event_id,operation_key,idempotency_key,selected_monotonic_ms,selected_state,context_spec,tool_spec,grammar_spec,budget_spec,recovery_spec,check_spec,exit_spec,compiler_status,status) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,'compiling','selected')", params![value.id,value.matter,value.event,value.operation,value.idempotency,value.monotonic_ms,value.specs[0],value.specs[1],value.specs[2],value.specs[3],value.specs[4],value.specs[5],value.specs[6],value.specs[7]])?;
            Ok(())
        })
    }

    pub fn attach_compilation(&mut self, decision: &str, attachments: &[u8], frame: &[u8],
        context_fp: &[u8], tool_fp: &[u8], refs: &[ContextRef<'_>]) -> StoreResult<()> {
        self.atomic(|tx| {
            for row in refs { tx.execute("INSERT INTO context_items(id,decision_id,source_kind,source_id,source_revision,semantic_key,trust,body_ref) VALUES(?1,?2,?3,?4,?5,?6,?7,?8)", params![row.id,decision,row.source_kind,row.source_id,row.revision,row.semantic_key,row.trust,row.body_ref])?; }
            changed(tx.execute("UPDATE runtime_decisions SET compiler_status='complete',compiler_attachments=?1,rendered_frame=?2,context_fingerprint=?3,tool_fingerprint=?4 WHERE id=?5 AND compiler_status='compiling'", params![attachments,frame,context_fp,tool_fp,decision])?, "decision is not compiling")
        })
    }

    pub fn provider_intent(&mut self, id: &str, decision: &str, request: &[u8], at: i64) -> StoreResult<()> {
        self.atomic(|tx| changed(tx.execute("INSERT INTO provider_exchanges(id,decision_id,request_ref,started_monotonic_ms,status) SELECT ?1,?2,?3,?4,'intended' FROM runtime_decisions WHERE id=?2 AND compiler_status='complete'", params![id,decision,request,at])?, "decision compilation is incomplete"))
    }

    pub fn provider_phase(&mut self, id: &str, expected: &str, next: &str) -> StoreResult<()> {
        if !matches!((expected,next), ("intended","sent") | ("sent","ambiguous")) {
            return Err(StoreError::InvalidState("provider phase transition".into()));
        }
        self.atomic(|tx| changed(tx.execute("UPDATE provider_exchanges SET status=?1 WHERE id=?2 AND status=?3", params![next,id,expected])?, "provider phase conflict"))
    }

    pub fn ambiguous_providers(&self) -> StoreResult<Vec<String>> {
        let mut query = self.connection.prepare(
            "SELECT id FROM provider_exchanges WHERE status IN ('sent','ambiguous') ORDER BY id",
        )?;
        let rows = query.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub fn provider_outcome(&mut self, id: &str, status: &str, response: &[u8], usage: (i64, i64), finished: i64, finish: &[u8], anomaly: &[u8], parsed: &[u8]) -> StoreResult<()> {
        if !matches!(status, "succeeded" | "failed") {
            return Err(StoreError::InvalidState("provider outcome".into()));
        }
        self.atomic(|tx| changed(tx.execute("UPDATE provider_exchanges SET status=?1,response_ref=?2,input_tokens=?3,output_tokens=?4,finished_monotonic_ms=?5,finish_reason=?6,anomaly=?7,parse_result=?8 WHERE id=?9 AND status='sent'", params![status,response,usage.0,usage.1,finished,finish,anomaly,parsed,id])?, "provider outcome conflict"))
    }

    pub fn prepare_effect(&mut self, value: &Effect<'_>) -> StoreResult<()> {
        if value.targets.is_empty() {
            return Err(StoreError::InvalidState("effect has no targets".into()));
        }
        self.atomic(|tx| {
            let complete: i64 = tx.query_row("SELECT count(*) FROM runtime_decisions WHERE id=?1 AND compiler_status='complete'", [value.decision], |r| r.get(0))?;
            if complete != 1 { return Err(StoreError::InvalidState("decision compilation is incomplete".into())); }
            tx.execute("INSERT INTO tool_admissions(id,decision_id,action_ordinal,action_fingerprint,origin,effectful,status,reason,parsed_call,tool_spec,journal_id) VALUES(?1,?2,?3,?4,'model',1,'accepted',?5,?6,?7,?8)", params![value.admission,value.decision,value.action_ordinal,value.action_fingerprint,value.reason,value.parsed_call,value.tool_spec,value.journal])?;
            tx.execute("INSERT INTO effect_journal(id,admission_id,decision_id,command_ordinal,idempotency_key,status,intended_fingerprint,prior_fingerprint) VALUES(?1,?2,?3,?4,?5,'prepared',?6,?7)", params![value.journal,value.admission,value.decision,value.action_ordinal,value.idempotency,value.intended_fingerprint,value.prior_fingerprint])?;
            for (ordinal,row) in value.targets.iter().enumerate() { tx.execute("INSERT INTO effect_targets(journal_id,ordinal,normalized_path,prior_bytes,intended_bytes,operation,prior_mode,intended_mode,stage_identity) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)", params![value.journal,ordinal as i64,row.path,row.prior,row.intended,row.operation,row.prior_mode,row.intended_mode,row.stage_identity])?; }
            Ok(())
        })
    }

    pub fn effect_phase(&mut self, id: &str, expected: &str, next: &str) -> StoreResult<()> {
        if !effect_transition(expected,next) { return Err(StoreError::InvalidState("effect phase transition".into())); }
        self.atomic(|tx| changed(tx.execute("UPDATE effect_journal SET status=?1 WHERE id=?2 AND status=?3", params![next,id,expected])?, "effect phase conflict"))
    }

    pub fn settle_effect(&mut self, value: &Settlement<'_>) -> StoreResult<()> {
        self.atomic(|tx| {
            tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'effect-observed',?4,?5,?6,'effect',?7)", params![value.event,value.matter,value.event_sequence,value.monotonic_ms,value.wall_time,value.event_payload,value.journal])?;
            tx.execute("INSERT INTO observations(id,journal_id,status,attempt_outcome,content_ref,fingerprint,contamination,event_id) VALUES(?1,?2,?3,?4,?5,?6,'clean',?7)", params![value.observation,value.journal,value.status,value.outcome,value.content_ref,value.fingerprint,value.event])?;
            tx.execute("INSERT INTO workspace_documents(id,current_path,status,managed) VALUES(?1,?2,'active',1) ON CONFLICT(id) DO NOTHING", params![value.document,value.path])?;
            tx.execute("INSERT INTO workspace_revisions(id,document_id,parent_id,sha256,content,effect_id,created_event_id) VALUES(?1,?2,?3,?4,?5,?6,?7)", params![value.revision,value.document,value.parent,value.sha256,value.content,value.journal,value.event])?;
            tx.execute("UPDATE workspace_documents SET current_revision_id=?1,current_path=?2 WHERE id=?3", params![value.revision,value.path,value.document])?;
            changed(tx.execute("UPDATE effect_journal SET status=CASE WHEN ?1='succeeded' THEN 'settled' ELSE 'failed' END,observation_id=?2,outcome_fingerprint=?3 WHERE id=?4 AND status='observing'", params![value.status,value.observation,value.fingerprint,value.journal])?, "effect is not observing")?;
            changed(tx.execute("UPDATE runtime_decisions SET status=CASE WHEN ?1='succeeded' THEN 'settled' ELSE 'failed' END,settlement_event_id=?2 WHERE id=(SELECT decision_id FROM effect_journal WHERE id=?3) AND status IN ('selected','admitted','running')", params![value.status,value.event,value.journal])?, "decision cannot settle")
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn close_matter(&mut self, matter: &str, message: &str, sequence: i64, body: &[u8],
        fingerprint: &[u8], event: &str, event_sequence: i64, monotonic: i64, wall: &str,
        payload: &[u8]) -> StoreResult<()> {
        self.atomic(|tx| {
            let blockers: i64 = tx.query_row("SELECT (SELECT count(*) FROM effect_journal j JOIN runtime_decisions d ON d.id=j.decision_id WHERE d.matter_id=?1 AND j.status NOT IN ('settled','compensated'))+(SELECT count(*) FROM runtime_decisions WHERE matter_id=?1 AND status NOT IN ('settled','failed'))+(SELECT count(*) FROM obligations o LEFT JOIN checks c ON c.id=o.current_check_id WHERE o.matter_id=?1 AND o.required=1 AND (o.status!='passed' OR c.current!=1 OR c.passed!=1))", [matter], |r| r.get(0))?;
            if blockers != 0 { return Err(StoreError::InvalidState("matter has blocking operation, effect, or check".into())); }
            tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'matter-completed',?4,?5,?6,'harness',?7)", params![event,matter,event_sequence,monotonic,wall,payload,message])?;
            tx.execute("INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,lifecycle,matter_id,cause_event_id) VALUES(?1,?2,'agent',?3,?4,'active',?5,?6)", params![message,sequence,body,fingerprint,matter,event])?;
            changed(tx.execute("UPDATE matters SET lifecycle='closed',closure_event_id=?1,closure_checks_passed=1,unsettled_effects=0,updated_sequence=?2 WHERE id=?3 AND lifecycle='open'", params![event,event_sequence,matter])?, "matter cannot close")
        })
    }

    fn atomic(&mut self, operation: impl FnOnce(&Transaction<'_>) -> StoreResult<()>) -> StoreResult<()> {
        let tx = self.connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        match operation(&tx).and_then(|()| tx.commit().map_err(Into::into)) {
            Ok(()) => Ok(()), Err(error) => Err(classify(error)),
        }
    }
}

fn changed(count: usize, message: &str) -> StoreResult<()> {
    if count == 1 {
        Ok(())
    } else {
        Err(StoreError::InvalidState(message.into()))
    }
}
#[rustfmt::skip]
fn effect_transition(old: &str, new: &str) -> bool {
    matches!((old,new), ("prepared","staging") | ("staging","exchange-ready") |
        ("exchange-ready","exchanging") | ("exchanging","exchanged") |
        ("exchanged","observing") | ("exchanged","compensating") |
        ("compensating","compensated") | ("compensated","observing"))
}
fn classify(error: StoreError) -> StoreError {
    match error {
        StoreError::Sql(ref text) if text.contains("FOREIGN KEY") => {
            StoreError::NotFound(text.clone())
        }
        StoreError::Sql(ref text) if text.contains("UNIQUE") => {
            StoreError::InvalidState(text.clone())
        }
        other => other,
    }
}
