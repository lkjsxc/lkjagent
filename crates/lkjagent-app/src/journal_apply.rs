use lkjagent_effects::{
    workspace::OpenedWorkspace,
    workspace_edit::{DurablePhase, ObservedTarget, PreparedEdit, Revision, VerifiedOutcome},
};
use lkjagent_store::transactions::{Effect, NativeStore, Settlement, Target};
use rusqlite::{Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use std::path::Path;
type R<T> = Result<T, String>;
type ManagedRow = (i64, Option<String>, Option<Vec<u8>>, Option<Vec<u8>>);

pub(crate) struct JournalEdit {
    edit: PreparedEdit,
    parents: Vec<String>,
    parent_revision: Option<String>,
}

pub(crate) fn prepare(
    db: &Path,
    workspace: &OpenedWorkspace,
    path: &str,
    rendered: &str,
) -> R<JournalEdit> {
    prepare_mode(db, workspace, path, rendered, 0o644)
}

#[rustfmt::skip]
pub(crate) fn prepare_mode(db:&Path,workspace:&OpenedWorkspace,path:&str,rendered:&str,mode:u32)->R<JournalEdit>{
 let connection=Connection::open(db).map_err(err)?;let managed:Option<ManagedRow>=connection.query_row("SELECT d.managed,d.current_revision_id,r.sha256,r.content FROM workspace_documents d LEFT JOIN workspace_revisions r ON r.id=d.current_revision_id WHERE d.current_path=?1",[path.as_bytes()],|row|Ok((row.get(0)?,row.get(1)?,row.get(2)?,row.get(3)?))).optional().map_err(err)?;
 match managed{None=>{let value=workspace.prepare_absent_edit(path.into(),rendered,mode).map_err(err)?;Ok(JournalEdit{edit:value.edit,parents:value.missing_parents,parent_revision:None})},Some((1,Some(parent),Some(hash),Some(content)))=>{let revision=hex(&hash);let observed=workspace.observe_edit_target(path).map_err(err)?;if !matches!(&observed,ObservedTarget::Present(value) if value.revision==revision&&value.bytes==content){return Err("managed record revision is stale".into())}let old=std::str::from_utf8(&content).map_err(err)?;let edit=workspace.prepare_exact_edit(path.into(),Revision::Sha256(revision),old,rendered,mode).map_err(err)?;Ok(JournalEdit{edit,parents:vec![],parent_revision:Some(parent)})},_=>Err("unmanaged existing record collision".into())}
}

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
pub(crate) fn apply(db:&Path,store:&mut NativeStore,workspace:&OpenedWorkspace,matter:&str,decision:&str,entry:&lkjagent_core::runtime_decision::ToolViewEntry,raw:&str,path:&str,prepared:JournalEdit)->R<String>{
 let intended=sha(&prepared.edit.intended_bytes);let prior=prepared.edit.prior_bytes.as_deref().map(sha);let action=sha(raw.as_bytes());let admission=id("admission",decision.as_bytes());let journal=id("journal",decision.as_bytes());let spec=serde_json::to_vec(entry).map_err(err)?;
 let stages=prepared.parents.iter().map(|path|format!("mkdir:{path}")).collect::<Vec<_>>();let mut targets=vec![Target{path:path.as_bytes(),prior:prepared.edit.prior_bytes.as_deref(),intended:Some(&prepared.edit.intended_bytes),operation:if prepared.edit.prior_bytes.is_some(){"replace"}else{"create"},prior_mode:prepared.edit.expected_mode.map(i64::from),intended_mode:Some(i64::from(prepared.edit.intended_mode)),stage_identity:prepared.edit.stage_identity.as_bytes()}];for (parent,stage) in prepared.parents.iter().zip(&stages){targets.push(Target{path:parent.as_bytes(),prior:None,intended:None,operation:"mkdir",prior_mode:None,intended_mode:Some(0o755),stage_identity:stage.as_bytes()});}
 store.prepare_exact_effect(&Effect{admission:&admission,journal:&journal,decision,action_ordinal:0,action_fingerprint:&action,reason:entry.effect_key.0.as_bytes(),parsed_call:raw.as_bytes(),tool_spec:&spec,idempotency:journal.as_bytes(),intended_fingerprint:&intended,prior_fingerprint:prior.as_deref(),targets:&targets}).map_err(err)?;
 workspace.create_declared_directories(&prepared.parents).map_err(err)?;for (old,new,phase) in [("prepared","staging",Some(DurablePhase::Staged)),("staging","exchange-ready",None),("exchange-ready","exchanging",Some(DurablePhase::Exchanged)),("exchanging","exchanged",Some(DurablePhase::Settled)),("exchanged","observing",None)]{store.effect_phase(&journal,old,new).map_err(err)?;if let Some(phase)=phase{workspace.advance_exact_edit(&prepared.edit,phase).map_err(err)?;}}
 let sequence=store.next_event_sequence(matter).map_err(err)?;let event=id("effect-event",decision.as_bytes());let observation=id("observation",decision.as_bytes());let document=id("document",path.as_bytes());let revision=id("revision",[path.as_bytes(),&intended].concat().as_slice());let wall=crate::clock::utc_now();store.settle_effect(&Settlement{journal:&journal,observation:&observation,event:&event,matter,event_sequence:sequence,monotonic_ms:crate::public_loop::millis(),wall_time:&wall,event_payload:raw.as_bytes(),status:"succeeded",outcome:b"managed record bytes committed",content_ref:revision.as_bytes(),fingerprint:&intended,document:&document,path:path.as_bytes(),revision:&revision,parent:prepared.parent_revision.as_deref(),sha256:&intended,content:&prepared.edit.intended_bytes}).map_err(err)?;workspace.cleanup_exact_edit(&prepared.edit,VerifiedOutcome::Settled).map_err(err)?;
 let mut connection=Connection::open(db).map_err(err)?;let checks=crate::automatic_checks::reduce_committed_edit(&mut connection,workspace,&journal,crate::public_loop::millis(),&crate::clock::utc_now()).map_err(err)?;Ok(format!("settled: decision={decision} journal={journal} path={path} checks={}/{}",checks.passed,checks.scheduled))
}

fn sha(value: &[u8]) -> Vec<u8> {
    Sha256::digest(value).to_vec()
}
fn id(prefix: &str, value: &[u8]) -> String {
    format!("{prefix}-{}", &hex(&sha(value))[..24])
}
fn hex(value: &[u8]) -> String {
    value.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn err(value: impl std::fmt::Debug) -> String {
    format!("{value:?}")
}
