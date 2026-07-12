use rusqlite::{params, OptionalExtension, Transaction};

use crate::error::{StoreError, StoreResult};
use crate::transactions::{Effect, NativeStore};

#[rustfmt::skip]
pub enum DirectTool { List, Search, Read }
#[rustfmt::skip]
impl DirectTool { fn name(&self) -> &'static str { match self { Self::List => "list", Self::Search => "search", Self::Read => "read" } } }
#[rustfmt::skip]
pub struct DirectSettlement<'a> {
    pub decision: &'a str, pub matter: &'a str, pub admission: &'a str,
    pub action_ordinal: i64, pub action_fingerprint: &'a [u8], pub parsed_call: &'a [u8],
    pub tool_spec: &'a [u8], pub tool: DirectTool, pub observation: &'a str,
    pub outcome: &'a [u8], pub content_ref: &'a [u8], pub fingerprint: &'a [u8],
    pub event: &'a str, pub event_sequence: i64, pub monotonic_ms: i64,
    pub wall_time: &'a str, pub event_payload: &'a [u8], pub namespace: &'a [u8],
    pub cell_key: &'a [u8], pub source_revision: &'a [u8], pub bytes_ref: &'a [u8],
}
#[rustfmt::skip]
pub enum ModelFaultKind { Malformed, Hidden, Stale }
#[rustfmt::skip]
impl ModelFaultKind { fn key(&self) -> &'static [u8] { match self { Self::Malformed => b"malformed", Self::Hidden => b"hidden", Self::Stale => b"stale" } } }
#[rustfmt::skip]
pub struct ModelFault<'a> {
    pub decision: &'a str, pub matter: &'a str, pub event: &'a str,
    pub event_sequence: i64, pub monotonic_ms: i64, pub wall_time: &'a str,
    pub event_payload: &'a [u8], pub fault_kind: ModelFaultKind, pub recovery_ref: &'a [u8],
    pub fingerprint: &'a [u8],
}
#[rustfmt::skip] #[derive(Debug, PartialEq, Eq)]
pub struct MatterRow { pub id: String, pub lifecycle: String }
#[rustfmt::skip] #[derive(Debug, PartialEq, Eq)]
pub struct CellRow { pub namespace: Vec<u8>, pub key: Vec<u8>, pub payload: Vec<u8>, pub fingerprint: Vec<u8> }
#[rustfmt::skip] #[derive(Debug, PartialEq, Eq)]
pub struct WorkRow { pub id: String, pub status: String }
#[rustfmt::skip] #[derive(Debug, PartialEq, Eq)]
pub struct RestartProjection {
    pub matter: Option<MatterRow>, pub cells: Vec<CellRow>, pub decisions: Vec<WorkRow>,
    pub exchanges: Vec<WorkRow>, pub effects: Vec<WorkRow>, pub checks_ready: bool,
}

impl NativeStore {
    pub fn settle_direct(&mut self, value: &DirectSettlement<'_>) -> StoreResult<()> {
        if value.outcome.len() > 65_536 {
            return Err(StoreError::InvalidState("observation exceeds bound".into()));
        }
        self.atomic(|tx| {
            if direct_retry(tx, value)? { return Ok(()); }
            tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'direct-observed',?4,?5,?6,'tool',?7)", params![value.event,value.matter,value.event_sequence,value.monotonic_ms,value.wall_time,value.event_payload,value.admission])?;
            tx.execute("INSERT INTO tool_admissions(id,decision_id,action_ordinal,action_fingerprint,origin,effectful,status,reason,parsed_call,tool_spec) VALUES(?1,?2,?3,?4,'model',0,'accepted',?5,?6,?7)", params![value.admission,value.decision,value.action_ordinal,value.action_fingerprint,value.tool.name().as_bytes(),value.parsed_call,value.tool_spec])?;
            tx.execute("INSERT INTO observations(id,decision_id,status,attempt_outcome,content_ref,fingerprint,contamination,event_id) VALUES(?1,?2,'succeeded',?3,?4,?5,'clean',?6)", params![value.observation,value.decision,value.outcome,value.content_ref,value.fingerprint,value.event])?;
            tx.execute("INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,?2,?3,?4,'active',?5,?6) ON CONFLICT(matter_id,namespace,cell_key) DO UPDATE SET payload=excluded.payload,status='active',source_event_id=excluded.source_event_id,fingerprint=excluded.fingerprint", params![value.matter,value.namespace,value.cell_key,cell_payload(value.source_revision,value.bytes_ref),value.event,value.fingerprint])?;
            one(tx.execute("UPDATE runtime_decisions SET status='settled',settlement_event_id=?1 WHERE id=?2 AND matter_id=?3 AND compiler_status='complete' AND status IN ('selected','admitted','running')", params![value.event,value.decision,value.matter])?, "decision cannot settle")
        })
    }

    pub fn reject_model_output(&mut self, value: &ModelFault<'_>) -> StoreResult<()> {
        self.atomic(|tx| {
            let old: Option<(String, Vec<u8>)> = tx.query_row("SELECT d.status,e.payload FROM runtime_decisions d JOIN runtime_events e ON e.id=d.settlement_event_id JOIN state_cells s ON s.source_event_id=e.id WHERE d.id=?1 AND e.id=?2 AND s.namespace='recovery' AND s.cell_key=?3 AND s.payload=?4 AND s.fingerprint=?5", params![value.decision,value.event,value.fault_kind.key(),value.recovery_ref,value.fingerprint], |r| Ok((r.get(0)?,r.get(1)?))).optional()?;
            if let Some((status, payload)) = old { return if status == "failed" && payload == value.event_payload { Ok(()) } else { Err(StoreError::InvalidState("fault retry conflict".into())) }; }
            tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'model-output-rejected',?4,?5,?6,'harness',?7)", params![value.event,value.matter,value.event_sequence,value.monotonic_ms,value.wall_time,value.event_payload,value.decision])?;
            tx.execute("INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,'recovery',?2,?3,'active',?4,?5) ON CONFLICT(matter_id,namespace,cell_key) DO UPDATE SET payload=excluded.payload,status='active',source_event_id=excluded.source_event_id,fingerprint=excluded.fingerprint", params![value.matter,value.fault_kind.key(),value.recovery_ref,value.event,value.fingerprint])?;
            one(tx.execute("UPDATE runtime_decisions SET status='failed',settlement_event_id=?1 WHERE id=?2 AND matter_id=?3 AND status IN ('selected','admitted','running')", params![value.event,value.decision,value.matter])?, "decision cannot fail")
        })
    }

    pub fn prepare_effect(&mut self, value: &Effect<'_>) -> StoreResult<()> {
        self.prepare(value, false)
    }
    pub fn prepare_exact_effect(&mut self, value: &Effect<'_>) -> StoreResult<()> {
        self.prepare(value, true)
    }
    fn prepare(&mut self, value: &Effect<'_>, obligations: bool) -> StoreResult<()> {
        if value.targets.is_empty() {
            return Err(StoreError::InvalidState("effect has no targets".into()));
        }
        if obligations
            && value.targets.iter().any(|t| {
                !matches!(t.operation, "create" | "replace")
                    || t.intended.is_none()
                    || (t.operation == "replace" && t.prior.is_none())
            })
        {
            return Err(StoreError::InvalidState(
                "exact effect requires create/edit bytes".into(),
            ));
        }
        self.atomic(|tx| {
            if effect_retry(tx, value)? { return Ok(()); }
            let complete: i64 = tx.query_row("SELECT count(*) FROM runtime_decisions WHERE id=?1 AND compiler_status='complete'", [value.decision], |r| r.get(0))?;
            if complete != 1 { return Err(StoreError::InvalidState("decision compilation is incomplete".into())); }
            tx.execute("INSERT INTO tool_admissions(id,decision_id,action_ordinal,action_fingerprint,origin,effectful,status,reason,parsed_call,tool_spec,journal_id) VALUES(?1,?2,?3,?4,'model',1,'accepted',?5,?6,?7,?8)", params![value.admission,value.decision,value.action_ordinal,value.action_fingerprint,value.reason,value.parsed_call,value.tool_spec,value.journal])?;
            tx.execute("INSERT INTO effect_journal(id,admission_id,decision_id,command_ordinal,idempotency_key,status,intended_fingerprint,prior_fingerprint) VALUES(?1,?2,?3,?4,?5,'prepared',?6,?7)", params![value.journal,value.admission,value.decision,value.action_ordinal,value.idempotency,value.intended_fingerprint,value.prior_fingerprint])?;
            for (ordinal, target) in value.targets.iter().enumerate() {
                tx.execute("INSERT INTO effect_targets(journal_id,ordinal,normalized_path,prior_bytes,intended_bytes,operation,prior_mode,intended_mode,stage_identity) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)", params![value.journal,ordinal as i64,target.path,target.prior,target.intended,target.operation,target.prior_mode,target.intended_mode,target.stage_identity])?;
                if obligations { insert_obligations(tx, value, ordinal, target.path, target.prior, target.intended)?; }
            }
            Ok(())
        })
    }

    pub fn restart_projection(&self) -> StoreResult<RestartProjection> {
        let matter = self.connection.query_row("SELECT id,lifecycle FROM matters WHERE lifecycle!='closed' ORDER BY priority DESC,created_sequence LIMIT 1", [], |r| Ok(MatterRow{id:r.get(0)?,lifecycle:r.get(1)?})).optional()?;
        let Some(ref current) = matter else {
            return Ok(RestartProjection {
                matter: None,
                cells: vec![],
                decisions: vec![],
                exchanges: vec![],
                effects: vec![],
                checks_ready: false,
            });
        };
        let cells = query_cells(&self.connection, &current.id)?;
        let decisions = query_work(&self.connection, "SELECT id,status FROM runtime_decisions WHERE matter_id=?1 AND status NOT IN ('settled','failed') ORDER BY selected_monotonic_ms,id", &current.id)?;
        let exchanges = query_work(&self.connection, "SELECT p.id,p.status FROM provider_exchanges p JOIN runtime_decisions d ON d.id=p.decision_id WHERE d.matter_id=?1 AND p.status NOT IN ('succeeded','failed') ORDER BY p.id", &current.id)?;
        let effects = query_work(&self.connection, "SELECT j.id,j.status FROM effect_journal j JOIN runtime_decisions d ON d.id=j.decision_id WHERE d.matter_id=?1 AND j.status NOT IN ('settled','failed','compensated') ORDER BY j.id", &current.id)?;
        let checks_ready = readiness(&self.connection, &current.id)?;
        Ok(RestartProjection {
            matter,
            cells,
            decisions,
            exchanges,
            effects,
            checks_ready,
        })
    }
}

#[rustfmt::skip]
fn one(count: usize, message: &str) -> StoreResult<()> {
    if count == 1 { Ok(()) } else { Err(StoreError::InvalidState(message.into())) }
}
fn cell_payload(revision: &[u8], body: &[u8]) -> Vec<u8> {
    let mut out = (revision.len() as u64).to_be_bytes().to_vec();
    out.extend(revision);
    out.extend(body);
    out
}
fn direct_retry(tx: &Transaction<'_>, v: &DirectSettlement<'_>) -> StoreResult<bool> {
    let found: Option<i64> = tx.query_row("SELECT count(*) FROM observations o JOIN runtime_decisions d ON d.id=o.decision_id JOIN tool_admissions a ON a.decision_id=d.id JOIN runtime_events e ON e.id=o.event_id JOIN state_cells s ON s.source_event_id=e.id WHERE o.id=?1 AND d.id=?2 AND d.status='settled' AND d.settlement_event_id=?3 AND a.id=?4 AND o.attempt_outcome=?5 AND o.content_ref=?6 AND o.fingerprint=?7 AND e.payload=?8 AND s.namespace=?9 AND s.cell_key=?10 AND s.payload=?11", params![v.observation,v.decision,v.event,v.admission,v.outcome,v.content_ref,v.fingerprint,v.event_payload,v.namespace,v.cell_key,cell_payload(v.source_revision,v.bytes_ref)], |r| r.get(0)).optional()?;
    Ok(found == Some(1))
}
#[rustfmt::skip]
fn effect_retry(tx: &Transaction<'_>, e: &Effect<'_>) -> StoreResult<bool> {
    let count:i64=tx.query_row("SELECT count(*) FROM effect_journal j JOIN tool_admissions a ON a.id=j.admission_id WHERE j.id=?1 AND a.id=?2 AND j.decision_id=?3 AND j.command_ordinal=?4 AND j.idempotency_key=?5 AND j.status='prepared' AND j.intended_fingerprint=?6 AND j.prior_fingerprint IS ?7 AND a.action_fingerprint=?8 AND a.reason=?9 AND a.parsed_call=?10 AND a.tool_spec=?11",params![e.journal,e.admission,e.decision,e.action_ordinal,e.idempotency,e.intended_fingerprint,e.prior_fingerprint,e.action_fingerprint,e.reason,e.parsed_call,e.tool_spec],|r|r.get(0))?;
    if count!=1 { return Ok(false); }
    for (n,t) in e.targets.iter().enumerate() { let found:i64=tx.query_row("SELECT count(*) FROM effect_targets WHERE journal_id=?1 AND ordinal=?2 AND normalized_path=?3 AND prior_bytes IS ?4 AND intended_bytes IS ?5 AND operation=?6 AND prior_mode IS ?7 AND intended_mode IS ?8 AND stage_identity=?9",params![e.journal,n as i64,t.path,t.prior,t.intended,t.operation,t.prior_mode,t.intended_mode,t.stage_identity],|r|r.get(0))?; if found!=1{return Ok(false);} }
    let total:i64=tx.query_row("SELECT count(*) FROM effect_targets WHERE journal_id=?1",[e.journal],|r|r.get(0))?;
    Ok(total==e.targets.len() as i64)
}
#[rustfmt::skip]
fn insert_obligations(tx: &Transaction<'_>, e: &Effect<'_>, n: usize, path: &[u8], prior: Option<&[u8]>, intended: Option<&[u8]>) -> StoreResult<()> {
    let matter:String=tx.query_row("SELECT matter_id FROM runtime_decisions WHERE id=?1",[e.decision],|r|r.get(0))?;
    let values=[
        ("workspace-bytes",e.prior_fingerprint.unwrap_or_default(),prior.unwrap_or_default()),
        ("content",e.intended_fingerprint,intended.unwrap_or_default()),
        ("collateral",e.intended_fingerprint,path),
    ];
    for (kind,revision,bytes) in values {
        let id=format!("{}/{n}/{kind}",e.journal); let payload=cell_payload(revision,bytes);
        tx.execute("INSERT INTO obligations(id,matter_id,predicate_kind,predicate_payload,required,status) VALUES(?1,?2,?3,?4,1,'open')",params![id,matter,kind,payload])?;
    }
    Ok(())
}
#[rustfmt::skip]
fn query_cells(c: &rusqlite::Connection, matter: &str) -> StoreResult<Vec<CellRow>> {
    let mut s=c.prepare("SELECT namespace,cell_key,payload,fingerprint FROM state_cells WHERE matter_id=?1 AND status='active' ORDER BY namespace,cell_key")?;
    let rows=s.query_map([matter],|r|Ok(CellRow{namespace:r.get(0)?,key:r.get(1)?,payload:r.get(2)?,fingerprint:r.get(3)?}))?.collect::<Result<Vec<_>,_>>()?;
    Ok(rows)
}
#[rustfmt::skip]
fn query_work(c: &rusqlite::Connection, sql: &str, matter: &str) -> StoreResult<Vec<WorkRow>> {
    let mut s=c.prepare(sql)?;
    let rows=s.query_map([matter],|r|Ok(WorkRow{id:r.get(0)?,status:r.get(1)?}))?.collect::<Result<Vec<_>,_>>()?;
    Ok(rows)
}
fn readiness(c: &rusqlite::Connection, matter: &str) -> StoreResult<bool> {
    let (required,blocked):(i64,i64)=c.query_row("SELECT count(*),sum(CASE WHEN o.status='passed' AND c.current=1 AND c.passed=1 THEN 0 ELSE 1 END) FROM obligations o LEFT JOIN checks c ON c.id=o.current_check_id WHERE o.matter_id=?1 AND o.required=1",[matter],|r|Ok((r.get(0)?,r.get::<_,Option<i64>>(1)?.unwrap_or(0))))?;
    Ok(required > 0 && blocked == 0)
}
