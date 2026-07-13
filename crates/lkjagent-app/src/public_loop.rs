use crate::model_io::Endpoint;
use lkjagent_core::{
    parse::ParsedOutput,
    runtime_context::{ContaminationClass, ContextItem, StalenessClass, TrustClass},
    runtime_decision::{OperationKey, RuntimeDecision},
    runtime_fingerprint::stable_fingerprint,
    runtime_operation::{RuntimePhase, RuntimePolicy, RuntimeState, Selection},
    runtime_prompt_kernel::{compile_prompt, PromptBudgets},
    runtime_selector::{select, EXIT_GUARDS, FILE_CHECK_KINDS},
    runtime_state::{CurrentTime, RuntimeSnapshot, StateCell, StateKey, StateStatus},
};
use lkjagent_effects::{workspace::OpenedWorkspace, workspace_edit::Revision};
use lkjagent_store::{
    direct_transactions::{DirectSettlement, DirectTool, ModelFault, ModelFaultKind},
    native_schema,
    transactions::{Cell, ContextRef, Decision, FinalClose, Intake, NativeStore},
};
use rusqlite::Connection;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type R<T> = Result<T, String>;
const MODEL_CALL_LIMIT: i64 = 64;
const TOKEN_BUDGET_LIMIT: i64 = 1_048_576;
const EFFECT_BUDGET_LIMIT: i64 = 16;
const RECOVERY_COST_LIMIT: i64 = 16;
const ACTIVE_MILLISECONDS_LIMIT: i64 = 900_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendReceipt {
    pub message_id: String,
    pub output: String,
}

pub fn send(data: &Path, text: &str, force_new: bool) -> R<String> {
    send_message(data, text, force_new).map(|receipt| receipt.output)
}

#[rustfmt::skip]
pub fn send_message(data:&Path,text:&str,force_new:bool)->R<SendReceipt>{
 fs::create_dir_all(data).map_err(e)?;let db=data.join("lkjagent.sqlite3");let mut s=NativeStore::open(&db).map_err(e)?;let q=s.next_queue_sequence().map_err(e)?;let seed=format!("{q}:{text}");let blocked=if force_new{None}else{s.latest_blocked_matter().map_err(e)?};let resumed=blocked.is_some();let matter=blocked.unwrap_or_else(||id("matter",seed.as_bytes()));let turn=id("turn",seed.as_bytes());let event=id(if resumed{"resume"}else{"intake"},turn.as_bytes());let payload=json!({"kind":if resumed{"matter/resumed"}else{"matter/opened"},"objective":text}).to_string();let fp=sha(text.as_bytes());let cfp=sha(payload.as_bytes());let cell=Cell(b"matter",b"opened",payload.as_bytes(),&cfp);let sequence=if resumed{s.next_event_sequence(&matter).map_err(e)?}else{1};let value=Intake{matter:&matter,objective:text.as_bytes(),turn:&turn,queue_sequence:q,raw_text:text.as_bytes(),message_fingerprint:&fp,event:&event,event_sequence:sequence,event_payload:payload.as_bytes(),monotonic_ms:millis(),wall_time:&crate::clock::utc_now(),obligations:&[],cells:&[cell]};let msg=if resumed{s.resume_blocked(&value)}else{s.owner_intake(&value)}.map_err(e)?;let output=format!("send: matter={matter} turn={turn} message={} sequence={} new={} resumed={resumed}",msg.id,msg.sequence,force_new||!resumed);
 Ok(SendReceipt{message_id:msg.id,output}) }

pub fn run(data: &Path, endpoint: &mut dyn Endpoint) -> R<()> {
    loop {
        let delay = if run_once(data, endpoint).is_ok() {
            100
        } else {
            250
        };
        std::thread::sleep(Duration::from_millis(delay));
    }
}

#[rustfmt::skip]
pub fn run_once(data:&Path,endpoint:&mut dyn Endpoint)->R<String>{
 let cycle=Instant::now();fs::create_dir_all(data).map_err(e)?; let db=data.join("lkjagent.sqlite3"); let mut store=NativeStore::open(&db).map_err(e)?;
 for exchange in store.ambiguous_providers().map_err(e)?{store.provider_phase(&exchange,"sent","ambiguous").map_err(e)?;}
 let p=store.restart_projection().map_err(e)?; let Some(m)=p.matter else{return Ok("idle: no open matter".into())};
 if m.lifecycle=="blocked"{return Err(format!("blocked: matter {} requires owner or config change",m.id))}
 let calls=store.provider_exchanges_in_budget_epoch(&m.id).map_err(e)?;if calls>=MODEL_CALL_LIMIT{return exhaust(&mut store,&m.id,None,"model-calls",calls,MODEL_CALL_LIMIT,0)}
 let (tokens,unknown)=store.accounted_tokens_in_budget_epoch(&m.id).map_err(e)?;if tokens>=TOKEN_BUDGET_LIMIT{return exhaust(&mut store,&m.id,None,"tokens",tokens,TOKEN_BUDGET_LIMIT,unknown)}
 let recovery=store.recovery_cost_in_budget_epoch(&m.id).map_err(e)?;if recovery>=RECOVERY_COST_LIMIT{return exhaust(&mut store,&m.id,None,"recovery-cost",recovery,RECOVERY_COST_LIMIT,0)}
 let active=store.active_milliseconds_in_budget_epoch(&m.id).map_err(e)?;if active>=ACTIVE_MILLISECONDS_LIMIT{return exhaust(&mut store,&m.id,None,"active-milliseconds",active,ACTIVE_MILLISECONDS_LIMIT,0)}
 if !p.effects.is_empty(){return Err(format!("blocked: unfinished effect {}:{}",p.effects[0].id,p.effects[0].status))}
 if !p.decisions.is_empty(){return Err(format!("blocked: unfinished decision {}:{}",p.decisions[0].id,p.decisions[0].status))}
 let snap=hydrate(&m.id,&p.cells)?; let now=crate::clock::utc_now(); let spec=match select(RuntimeState::from_snapshot(snap.clone()),RuntimePolicy::default(),CurrentTime(now.clone())){Selection::Decision(v)=>v,Selection::Idle=>return Ok("idle: no eligible decision".into()),other=>return Ok(format!("blocked: {other:?}"))};
 let timezone=crate::config::workspace_timezone(data)?;let bound=crate::journal_dispatch::bind_context(spec.phase,&now,&timezone)?;
 let seq=store.next_event_sequence(&m.id).map_err(e)?; let did=id("decision",format!("{}:{seq}:{}",m.id,spec.operation_key).as_bytes()); let devent=id("selected",did.as_bytes()); let mut decision=runtime_decision(&did,&m.id,&snap,&spec)?;crate::intent_tools::narrow(&mut decision,&m.objective,spec.phase,&snap); let specs=specs(&decision,&spec,&bound)?;
 store.select_decision(&Decision{id:&did,matter:&m.id,event:&devent,event_sequence:seq,event_payload:&specs[0],operation:spec.operation_key.as_bytes(),idempotency:did.as_bytes(),monotonic_ms:millis(),wall_time:&now,specs:[&specs[0],&specs[1],&specs[2],&specs[3],&specs[4],&specs[5],&specs[6],&specs[7]]}).map_err(e)?;
 let objective=owner_context(&db,&m.id,&m.objective)?; let mut sources=source_context(&p.cells);sources.extend(crate::history_context::load(&db,&m.id,&m.objective)?);sources.extend(crate::memory_context::candidates(&db,&objective.body)?);let items=std::iter::once(objective.clone()).chain(sources.iter().cloned()).collect::<Vec<_>>();let compiled=compile_prompt(&decision,&snap,objective,&sources,&prompt_budgets(data)?).map_err(|x|fault(&mut store,&did,&m.id,ModelFaultKind::Malformed,&x).unwrap_or(x))?; decision.context_frame_fingerprint=compiled.prompt.fingerprint.clone();
 let selected=compiled.context_plan.included.iter().map(|x|x.item_id.as_str()).collect::<BTreeSet<_>>();let owned=items.into_iter().filter(|x|selected.contains(x.id.as_str())).enumerate().map(|(n,x)|(format!("context-{did}-{n}"),x.source_type,x.source_id.into_bytes(),x.source_fingerprint.into_bytes(),x.semantic_key.into_bytes(),trust(x.trust).to_string(),x.body.into_bytes())).collect::<Vec<_>>();let refs=owned.iter().map(|x|ContextRef{id:&x.0,source_kind:&x.1,source_id:&x.2,revision:&x.3,semantic_key:&x.4,trust:&x.5,body_ref:&x.6}).collect::<Vec<_>>();
 let attachments=serde_json::to_vec(&compiled.context_plan).map_err(e)?; let frame=format!("{}\n{}",compiled.prompt.system,compiled.prompt.user); let toolfp=decision.tool_view_fingerprint().map_err(e)?.into_bytes(); store.attach_compilation(&did,&attachments,frame.as_bytes(),compiled.prompt.fingerprint.as_bytes(),&toolfp,&refs).map_err(e)?;
 let xid=id("exchange",did.as_bytes()); store.provider_intent(&xid,&did,frame.as_bytes(),millis()).map_err(e)?; store.provider_phase(&xid,"intended","sent").map_err(e)?;
 let answer=match endpoint.complete(&compiled.prompt,0){Ok(v)=>v,Err(x)=>{let limited=x=="endpoint completion hit max tokens"||x.starts_with("endpoint failure: response exceeds ");let detail=if limited{"output-limit"}else{x.as_str()};store.provider_outcome(&xid,"failed",detail.as_bytes(),(None,None),millis(),if limited{b"Length".as_slice()}else{b"error".as_slice()},b"endpoint",if limited{b"output-limit".as_slice()}else{b"not-parsed".as_slice()}).map_err(e)?;return fault(&mut store,&did,&m.id,if limited{ModelFaultKind::OutputLimit}else{ModelFaultKind::Stale},detail)}};
 let output_limit=answer.finish_reason=="Length";let raw=if output_limit{b"output-limit".as_slice()}else{bounded(answer.content.as_bytes(),16_384)};store.provider_outcome(&xid,"succeeded",raw,(answer.prompt_tokens.map(i64::from),answer.completion_tokens.map(i64::from)),millis(),answer.finish_reason.as_bytes(),answer.anomaly.as_deref().unwrap_or("").as_bytes(),if output_limit{b"output-limit".as_slice()}else{b"strict-parse-follows".as_slice()}).map_err(e)?;
 if output_limit{return fault(&mut store,&did,&m.id,ModelFaultKind::OutputLimit,"output-limit")}
 let (tokens,unknown)=store.accounted_tokens_in_budget_epoch(&m.id).map_err(e)?;if tokens>=TOKEN_BUDGET_LIMIT{return exhaust(&mut store,&m.id,Some(&did),"tokens",tokens,TOKEN_BUDGET_LIMIT,unknown)}
 let active=store.active_milliseconds_in_budget_epoch(&m.id).map_err(e)?.saturating_add(i64::try_from(cycle.elapsed().as_millis()).unwrap_or(i64::MAX));if active>=ACTIVE_MILLISECONDS_LIMIT{return exhaust(&mut store,&m.id,Some(&did),"active-milliseconds",active,ACTIVE_MILLISECONDS_LIMIT,0)}
 let parsed=match lkjagent_core::parse::parse_expected_for_decision(&decision,&answer.content){Ok(v)=>v,Err(x)=>{let text=format!("{x:?}");if decision.expected_envelope==lkjagent_core::runtime_decision::OutputEnvelope::Message&&output_faults(&db,&m.id)>0{return close(&mut store,&m.id,&did,"Completed with current harness checks.")}let outcome=fault(&mut store,&did,&m.id,if text.contains("UnknownTool"){ModelFaultKind::Hidden}else{ModelFaultKind::Malformed},&text)?;if answer.content.trim_start().starts_with("<final>"){if let Some((path,revision))=current_source(&store){let _=store.reuse_checked_revision(&m.id,&source_decision(&db,&m.id).unwrap_or_else(||did.to_string()),path.as_bytes(),&revision).map_err(e)?;}}return Ok(outcome)}};
 match parsed { ParsedOutput::Action(a)=>match dispatch(data,&db,&mut store,&m.id,&m.objective,&did,&decision,&a.tool,&a.params,&answer.content){Ok(v)=>Ok(v),Err(x)=>fault(&mut store,&did,&m.id,ModelFaultKind::Stale,&x)}, ParsedOutput::Message(body) if final_claims_allowed(&body)=>close(&mut store,&m.id,&did,&body), ParsedOutput::Message(_)=>fault(&mut store,&did,&m.id,ModelFaultKind::Malformed,"unsupported future or command claim in final wording") }
}

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
pub(crate) fn exhaust(store:&mut NativeStore,matter:&str,decision:Option<&str>,dimension:&str,used:i64,limit:i64,unknown:i64)->R<String>{let seq=store.next_event_sequence(matter).map_err(e)?;let event=id("budget-block",format!("{matter}:{seq}:{dimension}").as_bytes());let payload=json!({"schema":"matter-budget-block.v1","dimension":dimension,"used":used,"limit":limit,"unknown_usage_exchanges":unknown}).to_string();store.block_budget(matter,decision,&event,seq,millis(),&crate::clock::utc_now(),payload.as_bytes(),&sha(payload.as_bytes())).map_err(e)?;let label=if dimension=="model-calls"{"model-call"}else{dimension};Err(format!("blocked: matter {matter} exhausted {label} budget {used}/{limit}"))}

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn dispatch(data:&Path,db:&Path,store:&mut NativeStore,matter:&str,objective:&[u8],did:&str,d:&RuntimeDecision,tool:&str,args:&[(String,String)],raw:&str)->R<String>{
 let action=lkjagent_core::runtime_admission::ModelAction{tool:tool.into(),params:args.iter().cloned().collect()};
 let admission=lkjagent_core::runtime_admission::admit_action(d,&action).map_err(e)?;
 if admission.status!=lkjagent_core::runtime_admission::AdmissionStatus::Admitted{return Err(format!("admission rejected: {}",admission.reason))}
 let effect=lkjagent_core::runtime_admission::admitted_effect_key(d,&admission).map_err(str::to_string)?;
 let entry=d.tool_view.entry(tool).ok_or("persisted tool entry missing")?;let get=|n:&str|args.iter().find(|x|x.0==n).map(|x|x.1.as_str());
 if effect.0=="workspace.record"{return crate::journal_dispatch::dispatch(data,db,store,matter,did,entry,args,raw)}let root=crate::config::workspace_root(data)?;let root=crate::workspace_root::open(&root)?;let ws=OpenedWorkspace::open(&root).map_err(e)?;let path=get("path").ok_or("path missing")?;
 match effect.0.as_str(){
  "workspace.read"=>{let page=match ws.read_file(path,get("offset").and_then(|v|v.parse().ok()).unwrap_or(1),get("count").and_then(|v|v.parse().ok()).unwrap_or(200)){Ok(page)=>page,Err(error)=>{if let Some(absent)=crate::absent_read::observe(&ws,objective,path){return direct(store,matter,did,raw,entry,DirectTool::Read,b"source",b"current",path.as_bytes(),absent.body.as_bytes(),absent.context.as_bytes(),b"absent")}return Err(e(error))}}; let lines=page.lines.iter().map(|line|json!({"number":line.number,"text":&line.text})).collect::<Vec<_>>();let context=page.lines.iter().map(|line|format!("{}: {}",line.number,line.text)).collect::<Vec<_>>().join("\n"); let body=json!({"path":&page.path,"revision":&page.revision,"lines":lines,"total_lines":page.total_lines,"truncated":page.truncated,"next_line":page.next_line,"final_newline":page.final_newline}).to_string(); let outcome=direct(store,matter,did,raw,entry,DirectTool::Read,b"source",b"current",page.path.as_bytes(),body.as_bytes(),context.as_bytes(),page.revision.as_bytes())?; if get("complete")==Some("true")&&crate::continuity::allowed(objective){let _=store.reuse_checked_revision(matter,did,path.as_bytes(),&page.revision).map_err(e)?;} Ok(outcome)},
  "workspace.list"=>{let listing=ws.list_directory(path,get("offset").and_then(|v|v.parse().ok()).unwrap_or(0),get("count").and_then(|v|v.parse().ok()).unwrap_or(20)).map_err(e)?;if get("complete")==Some("true")&&crate::continuity::allowed(objective){if let Some(target)=crate::continuity::managed_path(db,objective){if let Ok(page)=ws.read_file(&target,1,200){let lines=page.lines.iter().map(|line|json!({"number":line.number,"text":&line.text})).collect::<Vec<_>>();let context=page.lines.iter().map(|line|format!("{}: {}",line.number,line.text)).collect::<Vec<_>>().join("\n");let out=json!({"path":&target,"revision":&page.revision,"lines":lines,"total_lines":page.total_lines,"truncated":page.truncated,"next_line":page.next_line,"final_newline":page.final_newline,"listing_path":path,"listing":format!("{listing:?}")}).to_string();let outcome=direct(store,matter,did,raw,entry,DirectTool::List,b"source",b"current",target.as_bytes(),out.as_bytes(),context.as_bytes(),page.revision.as_bytes())?;if get("complete")==Some("true"){let _=store.reuse_checked_revision(matter,did,target.as_bytes(),&page.revision).map_err(e)?;}return Ok(outcome)}}}let out=json!({"path":path,"listing":format!("{listing:?}")}).to_string();let fp=sha(out.as_bytes());let context=format!("directory {path}\n{listing:?}");direct(store,matter,did,raw,entry,DirectTool::List,b"observation",b"current",path.as_bytes(),out.as_bytes(),context.as_bytes(),&fp)},
  "workspace.search"=>{let query=get("query").ok_or("query missing")?;let results=ws.search_text(path,query).map_err(e)?;let out=json!({"path":path,"query":query,"results":format!("{results:?}")}).to_string(); let fp=sha(out.as_bytes());let context=format!("search {query} in {path}\n{results:?}"); direct(store,matter,did,raw,entry,DirectTool::Search,b"observation",b"current",path.as_bytes(),out.as_bytes(),context.as_bytes(),&fp)},
  "workspace.edit"|"workspace.create"=>modify(db,store,&ws,matter,did,entry,path,get("old_text").unwrap_or(""),get("new_text").or_else(||get("content")).unwrap_or(""),raw),
  key=>Err(format!("blocked: persisted effect key {key}")) }
}

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn direct(store:&mut NativeStore,matter:&str,did:&str,raw:&str,entry:&lkjagent_core::runtime_decision::ToolViewEntry,tool:DirectTool,ns:&[u8],key:&[u8],source_path:&[u8],body:&[u8],context_body:&[u8],revision:&[u8])->R<String>{
 let seq=store.next_event_sequence(matter).map_err(e)?; let admission=id("admission",did.as_bytes()); let event=id("observed",did.as_bytes()); let observation=id("observation",did.as_bytes()); let fp=sha(body); let spec=serde_json::to_vec(entry).map_err(e)?;
 store.settle_direct(&DirectSettlement{decision:did,matter,admission:&admission,action_ordinal:0,action_fingerprint:&sha(raw.as_bytes()),parsed_call:raw.as_bytes(),tool_spec:&spec,tool,observation:&observation,outcome:body,content_ref:body,fingerprint:&fp,event:&event,event_sequence:seq,monotonic_ms:millis(),wall_time:&crate::clock::utc_now(),event_payload:body,namespace:ns,cell_key:key,source_path,source_revision:revision,bytes_ref:context_body}).map_err(e)?; Ok(format!("settled: decision={did} observation={observation}")) }

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn modify(db:&Path,store:&mut NativeStore,ws:&OpenedWorkspace,matter:&str,did:&str,entry:&lkjagent_core::runtime_decision::ToolViewEntry,path:&str,old:&str,new:&str,raw:&str)->R<String>{
 let effects=store.effects_in_budget_epoch(matter).map_err(e)?;if effects>=EFFECT_BUDGET_LIMIT{return exhaust(store,matter,Some(did),"effects",effects,EFFECT_BUDGET_LIMIT,0)}
 let revision=if entry.effect_key.0=="workspace.create"{Revision::Absent}else{Revision::Sha256(source_revision(store,path)?)};let prepared=match crate::journal_apply::prepare_direct(ws,path,revision,old,new){Ok(value)=>value,Err(error)=>return fault(store,did,matter,ModelFaultKind::Stale,&error)};
 crate::journal_apply::apply(db,store,ws,matter,did,entry,raw,path,prepared)
}

#[rustfmt::skip]
fn close(store:&mut NativeStore,matter:&str,did:&str,body:&str)->R<String>{let checked=store.checked_paths(matter).map_err(e)?;let facts=checked.iter().map(|(path,revision)|format!("{path}@{revision}")).collect::<Vec<_>>().join(", ");let body=format!("{}\nChecked: {facts}",body.trim());let seq=store.next_event_sequence(matter).map_err(e)?;let event=id("close",did.as_bytes());let fp=sha(body.as_bytes());let msg=store.close_matter(&FinalClose{matter,decision:did,body:body.as_bytes(),body_fingerprint:&fp,event:&event,event_sequence:seq,monotonic_ms:millis(),wall_time:&crate::clock::utc_now(),payload:b"harness checked close"}).map_err(e)?;Ok(format!("closed: matter={matter} decision={did} message={} sequence={}",msg.id,msg.sequence))}
#[rustfmt::skip]
fn fault(store:&mut NativeStore,did:&str,matter:&str,kind:ModelFaultKind,text:&str)->R<String>{let seq=store.next_event_sequence(matter).map_err(e)?;let event=id("fault",format!("{did}:{text}").as_bytes());let fp=sha(text.as_bytes());store.reject_model_output(&ModelFault{decision:did,matter,event:&event,event_sequence:seq,monotonic_ms:millis(),wall_time:&crate::clock::utc_now(),event_payload:text.as_bytes(),fault_kind:kind,recovery_ref:bounded(text.as_bytes(),1024),fingerprint:&fp}).map_err(e)?;let used=store.recovery_cost_in_budget_epoch(matter).map_err(e)?;if used>=RECOVERY_COST_LIMIT{return exhaust(store,matter,None,"recovery-cost",used,RECOVERY_COST_LIMIT,0)}Ok(format!("fault: decision={did} event={event}"))}

#[rustfmt::skip]
fn hydrate(matter:&str,cells:&[lkjagent_store::direct_transactions::CellRow])->R<RuntimeSnapshot>{let mut out=RuntimeSnapshot::empty(matter);for row in cells{let ns=String::from_utf8(row.namespace.clone()).map_err(e)?;let name=String::from_utf8(row.key.clone()).map_err(e)?;let key=StateKey::new(&ns,&name).map_err(e)?;out.cells.insert(key.clone(),StateCell{key,status:StateStatus::Active,priority:0,confidence:100,payload_schema:format!("state.{ns}.v1"),payload_json:String::from_utf8(row.payload.clone()).map_err(e)?,evidence_refs:vec![],source_event_id:"event-1".into(),created_at:String::new(),updated_at:String::new(),expires_at:None,cooldown_until:None,conflict_group:None,parent_key:None});}Ok(out)}
#[rustfmt::skip]
fn runtime_decision(id:&str,matter:&str,s:&RuntimeSnapshot,spec:&lkjagent_core::runtime_operation::RuntimeDecisionSpec)->R<RuntimeDecision>{let mut d=RuntimeDecision::new(id,matter,OperationKey(spec.operation_key.clone()),spec.tool_view.clone(),spec.expected_envelope);let selected=match spec.phase{RuntimePhase::Orient=>"matter:opened".into(),RuntimePhase::Modify if spec.operation_key.starts_with("modify.report")=>"report:pending".into(),RuntimePhase::Modify if s.cells.keys().any(|k|k.namespace=="recovery")=>s.cells.keys().find(|k|k.namespace=="recovery").map(|k|format!("{}:{}",k.namespace,k.name)).unwrap_or_else(||"recovery:stale".into()),RuntimePhase::Modify=>"source:current".into(),RuntimePhase::Respond=>"check:current-passed".into(),_=>"edit:committed".into()};d.selected_state_key=Some(selected);let fp=s.fingerprint().map_err(e)?;d.snapshot_fingerprint=fp.clone();d.state_vector_fingerprint=fp.clone();d.context_frame_fingerprint=fp;d.model_budget_tokens=spec.model_budget_tokens;d.recovery_policy=spec.recovery_policy.clone();d.refresh_harness_state();Ok(d)}
#[rustfmt::skip]
fn specs(d:&RuntimeDecision,_s:&lkjagent_core::runtime_operation::RuntimeDecisionSpec,context:&crate::journal_dispatch::BoundDecisionContext)->R<[Vec<u8>;8]>{Ok([serde_json::to_vec(&(d.selected_state_key.clone(),&d.snapshot_fingerprint)).map_err(e)?,serde_json::to_vec(context).map_err(e)?,serde_json::to_vec(&d.tool_view).map_err(e)?,serde_json::to_vec(&d.expected_envelope).map_err(e)?,serde_json::to_vec(&d.model_budget_tokens).map_err(e)?,serde_json::to_vec(&d.recovery_policy).map_err(e)?,serde_json::to_vec(FILE_CHECK_KINDS).map_err(e)?,serde_json::to_vec(EXIT_GUARDS).map_err(e)?])}
#[rustfmt::skip]
fn source_context(cells:&[lkjagent_store::direct_transactions::CellRow])->Vec<ContextItem>{cells.iter().filter(|c|c.namespace==b"source"||c.namespace==b"observation").filter_map(|c|{let v=serde_json::from_slice::<Value>(&c.payload).ok()?;let ns=String::from_utf8_lossy(&c.namespace);let raw=v["body_ref"].as_str().unwrap_or("");let body=serde_json::from_str::<Value>(raw).ok().and_then(|inner|inner["context"].as_str().map(str::to_owned)).unwrap_or_else(||raw.to_owned());Some(context(&format!("state-{ns}"),if ns=="source"{"workspace-source"}else{"workspace-observation"},body,TrustClass::Measured,"workspace",v["revision"].as_str().unwrap_or("")))}).collect()}
#[rustfmt::skip]
fn owner_context(db:&Path,matter:&str,objective:&[u8])->R<ContextItem>{let c=Connection::open(db).map_err(e)?;let (id,body,fp):(String,Vec<u8>,Vec<u8>)=c.query_row("SELECT id,body,body_fingerprint FROM conversation_messages WHERE matter_id=?1 AND role='owner' AND lifecycle='active' ORDER BY sequence DESC LIMIT 1",[matter],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).map_err(e)?;if body!=objective{return Err("current owner objective lineage is stale".into())}Ok(ContextItem{id:format!("objective-{id}"),semantic_key:"objective".into(),body:String::from_utf8(body).map_err(e)?,source_type:"owner".into(),source_id:id,source_fingerprint:hex(&fp),trust:TrustClass::Owner,staleness:StalenessClass::Current,contamination:ContaminationClass::Clean,artifact_refs:vec![],decision_id:None,created_at:String::new()})}
#[rustfmt::skip]
fn current_source(store:&NativeStore)->Option<(String,String)>{store.restart_projection().ok()?.cells.into_iter().find(|c|c.namespace==b"source").and_then(|c|{let outer=serde_json::from_slice::<Value>(&c.payload).ok()?;Some((outer["path"].as_str()?.into(),outer["revision"].as_str()?.into()))})}
#[rustfmt::skip]
fn output_faults(db:&Path,matter:&str)->i64{Connection::open(db).ok().and_then(|c|c.query_row("SELECT count(*) FROM runtime_events WHERE matter_id=?1 AND kind='model-output-rejected'",[matter],|r|r.get(0)).ok()).unwrap_or(0)}
#[rustfmt::skip]
fn source_decision(db:&Path,matter:&str)->Option<String>{Connection::open(db).ok()?.query_row("SELECT o.decision_id FROM state_cells s JOIN observations o ON o.event_id=s.source_event_id WHERE s.matter_id=?1 AND CAST(s.namespace AS TEXT)='source' AND CAST(s.cell_key AS TEXT)='current' AND s.status='active'",[matter],|r|r.get(0)).ok()}
#[rustfmt::skip]
fn source_revision(store:&NativeStore,path:&str)->R<String>{current_source(store).filter(|(current,_)|current==path).map(|(_,revision)|revision).ok_or_else(||"no current revision-bound source for edit".into())}
#[rustfmt::skip]
fn context(id:&str,key:&str,body:String,trust:TrustClass,kind:&str,source:&str)->ContextItem{ContextItem{id:id.into(),semantic_key:key.into(),body,source_type:kind.into(),source_id:source.into(),source_fingerprint:stable_fingerprint(&source).unwrap_or_default(),trust,staleness:StalenessClass::Current,contamination:ContaminationClass::Clean,artifact_refs:vec![],decision_id:None,created_at:String::new()}}
#[rustfmt::skip]
fn trust(value:TrustClass)->&'static str{match value{TrustClass::Owner|TrustClass::Memory=>"owner",TrustClass::Measured=>"workspace",TrustClass::External|TrustClass::Model=>"provider",_=>"tool"}}
#[rustfmt::skip]
fn final_claims_allowed(body:&str)->bool{let lower=body.to_ascii_lowercase();!["i will ","will update","will create","going to ","ready to ","command ran","command passed","tests passed","test suite passed"].iter().any(|claim|lower.contains(claim))}
#[rustfmt::skip]
fn prompt_budgets(data:&Path)->R<PromptBudgets>{let total=crate::config::prompt_max_context_tokens(data)?.unwrap_or(16_384).min(u64::from(u32::MAX)) as u32;let mut value=PromptBudgets::default();value.total_tokens=total;value.source_tokens=value.source_tokens.min(total/2);value.observation_tokens=value.observation_tokens.min(total/8);value.memory_tokens=value.memory_tokens.min(total/8);value.agent_file_tokens=value.agent_file_tokens.min(total/8);Ok(value)}

#[rustfmt::skip]
pub fn doctor(data:&Path,json_output:bool)->R<String>{fs::create_dir_all(data).map_err(e)?;let db=data.join("lkjagent.sqlite3");let store=NativeStore::open(&db).map_err(e)?;let p=store.restart_projection().map_err(e)?;let c=Connection::open(&db).map_err(e)?;
 let matters:i64=c.query_row("SELECT count(*) FROM matters",[],|r|r.get(0)).map_err(e)?;let (workspace,present)=crate::config::workspace_state(data)?;let endpoint=crate::config::endpoint_state(data);let prompt=crate::config::prompt_max_context_tokens(data)?.unwrap_or(0);
 if json_output{return serde_json::to_string(&json!({"data_root":data,"workspace_root":workspace,"workspace_present":present,"endpoint":endpoint,"schema":"native-18","table_count":lkjagent_store::native_schema::NATIVE_TABLES.len(),"missing_tables":[],"matters":matters,"unfinished_decisions":p.decisions.len(),"prompt_max_context_tokens":prompt})).map_err(e)}
 Ok(format!("schema=native-18 tables={} missing=0\nroots: data={} workspace={} workspace_present={present}\nendpoint: {endpoint}\nmatters={matters} unfinished_decisions={} prompt_cap={prompt}",lkjagent_store::native_schema::NATIVE_TABLES.len(),data.display(),workspace.display(),p.decisions.len()))}

#[rustfmt::skip]
pub fn status(data:&Path)->R<String>{fs::create_dir_all(data).map_err(e)?;let db=data.join("lkjagent.sqlite3");let store=NativeStore::open(&db).map_err(e)?;let p=store.restart_projection().map_err(e)?;let c=Connection::open(&db).map_err(e)?;let matters:i64=c.query_row("SELECT count(*) FROM matters",[],|r|r.get(0)).map_err(e)?;let closed:i64=c.query_row("SELECT count(*) FROM matters WHERE lifecycle='closed'",[],|r|r.get(0)).map_err(e)?;let messages=native_schema::conversation(&c,None,20).map_err(e)?.into_iter().map(|m|format!("{}:{}:{}",m.sequence,m.role,m.id)).collect::<Vec<_>>().join(",");let workspace=crate::config::workspace_root(data)?;Ok(format!("roots: data={} workspace={}\nmatters: total={matters} closed={closed} active={} lifecycle={}\nunfinished: decisions={} exchanges={} effects={} checks-ready={}\nconversation: {messages}",data.display(),workspace.display(),p.matter.as_ref().map(|m|m.id.as_str()).unwrap_or("none"),p.matter.as_ref().map(|m|m.lifecycle.as_str()).unwrap_or("none"),p.decisions.len(),p.exchanges.len(),p.effects.len(),p.checks_ready))}

fn sha(v: &[u8]) -> Vec<u8> {
    Sha256::digest(v).to_vec()
}
fn id(prefix: &str, v: &[u8]) -> String {
    format!("{prefix}-{}", &hex(&sha(v))[..24])
}
fn hex(v: &[u8]) -> String {
    v.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_millis().min(i64::MAX as u128) as i64)
        .unwrap_or(0)
}
fn bounded(v: &[u8], n: usize) -> &[u8] {
    &v[..v.len().min(n)]
}
fn e(x: impl std::fmt::Debug) -> String {
    format!("{x:?}")
}
