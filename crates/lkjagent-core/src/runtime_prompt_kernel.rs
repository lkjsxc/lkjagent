use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prompt_policy::{envelope_tag, protocol_for_envelope};
use crate::render::Prompt;
use crate::runtime_context::{
    normalized_body, ContextFramePlan, ContextItem, ContextPlanEntry, TrustClass,
};
use crate::runtime_decision::RuntimeDecision;
use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};
use crate::runtime_state::{RuntimeSnapshot, StateCell};
use crate::runtime_tool_cards::protocol_card;

pub const PROMPT_PROFILE: &str = "kernel-v2";
pub const CONTEXT_PROFILE: &str = "decision-context-v1";

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptCard { pub id: String, pub kind: String, pub reason: String, pub fingerprint: String }
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptCardPlan { pub prompt_profile: String, pub context_profile: String,
    pub cards: Vec<PromptCard>, pub fingerprint: String }

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptBudgets { pub total_tokens: u32, pub source_tokens: u32,
    pub observation_tokens: u32, pub memory_tokens: u32, pub agent_file_tokens: u32 }
#[rustfmt::skip]
impl Default for PromptBudgets {
    fn default() -> Self { Self { total_tokens: 8_000, source_tokens: 2_500,
        observation_tokens: 512, memory_tokens: 512, agent_file_tokens: 512 } }
}
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledPrompt { pub prompt: Prompt, pub context_plan: ContextFramePlan }

#[rustfmt::skip]
pub fn compile_prompt(decision: &RuntimeDecision, snapshot: &RuntimeSnapshot, objective: ContextItem,
    sources: &[ContextItem], budgets: &PromptBudgets) -> Result<CompiledPrompt, String> {
    validate_binding(decision, snapshot, &objective, budgets)?;
    let conflicts = crate::runtime_context::detect_contradictions(sources);
    let mut all = vec![objective.clone()]; all.extend_from_slice(sources);
    let all_conflicts = crate::runtime_context::detect_contradictions(&all);
    let mut plan = crate::runtime_context::select_context_plan(&all, &all_conflicts);
    if !plan.included.iter().any(|entry| entry.item_id == objective.id) {
        return Err("current owner objective was suppressed".into());
    }
    let by_id = all.iter().map(|item| (item.id.as_str(), item)).collect::<BTreeMap<_, _>>();
    apply_budgets(&mut plan, &by_id, &objective.id, budgets)?;
    let selected = plan.included.iter().filter_map(|entry| by_id.get(entry.item_id.as_str()).copied()).collect::<Vec<_>>();
    let bodies = selected.iter().map(|item| normalized_body(&item.body)).collect::<BTreeSet<_>>();
    let state = selected_cell(decision, snapshot)?;
    let system = format!("{}\n{}\n{}\n{}", kernel_card(decision), state_card(state, &bodies),
        tools_card(decision)?, output_card(decision));
    let user = selected.iter().map(|item| source_card(item)).collect::<Vec<_>>().join("\n");
    let max_tokens = decision.model_budget_tokens.ok_or("decision has no model output budget")?;
    if estimate_tokens(&format!("{system}\n{user}")) > budgets.total_tokens {
        return Err("compiled prompt exceeds total token budget".into());
    }
    let stop = envelope_tag(decision.expected_envelope).map_or(String::new(), |tag| format!("</{tag}>"));
    let fingerprint = stable_fingerprint(&(&system, &user, max_tokens, &stop)).map_err(|error| error.message)?;
    let prompt = Prompt { system, user, fingerprint, max_tokens, stop };
    let _ = conflicts;
    Ok(CompiledPrompt { prompt, context_plan: plan })
}

#[rustfmt::skip]
fn validate_binding(decision: &RuntimeDecision, snapshot: &RuntimeSnapshot, objective: &ContextItem,
    budgets: &PromptBudgets) -> Result<(), String> {
    if decision.id.is_empty() || decision.case_id != snapshot.case_id { return Err("decision is not bound to snapshot case".into()); }
    let fingerprint = snapshot.fingerprint().map_err(|error| error.message)?;
    if decision.snapshot_fingerprint != fingerprint || decision.state_vector_fingerprint != fingerprint {
        return Err("decision state view is stale".into()); }
    if objective.trust != TrustClass::Owner || objective.semantic_key != "objective" || !objective.is_normal_prompt_candidate() {
        return Err("one clean current owner objective is required".into());
    }
    if budgets.total_tokens == 0 || budgets.source_tokens == 0 { return Err("prompt budgets must be positive".into()); }
    Ok(())
}

#[rustfmt::skip]
fn apply_budgets(plan: &mut ContextFramePlan, items: &BTreeMap<&str, &ContextItem>, objective: &str,
    budgets: &PromptBudgets) -> Result<(), String> {
    let mut spent = BTreeMap::<&str, u32>::new(); let mut keep = Vec::new(); let mut excluded = Vec::new();
    for entry in plan.included.drain(..) {
        let item = items.get(entry.item_id.as_str()).ok_or("context plan lost an item")?;
        let lane = lane(item, objective); let cap = match lane { "memory" => budgets.memory_tokens,
            "observation" => budgets.observation_tokens, _ => budgets.source_tokens };
        let item_cap = if is_agent_file(item) { cap.min(budgets.agent_file_tokens).min(512) } else { cap };
        let cost = estimate_tokens(&item.body);
        if cost <= item_cap && spent.get(lane).copied().unwrap_or(0).saturating_add(cost) <= cap {
            spent.insert(lane, spent.get(lane).copied().unwrap_or(0) + cost); keep.push(entry);
        } else if entry.item_id == objective { return Err("owner objective exceeds source budget".into()); }
        else { excluded.push(ContextPlanEntry { reason: "lane-budget-exhausted".into(), ..entry }); }
    }
    plan.included = keep; plan.excluded.extend(excluded); Ok(())
}
#[rustfmt::skip]
fn lane<'a>(item: &ContextItem, objective: &str) -> &'a str {
    if item.id == objective { "source" } else if item.trust == TrustClass::Memory { "memory" }
    else if item.source_type == "observation" { "observation" } else { "source" }
}
fn is_agent_file(item: &ContextItem) -> bool {
    item.source_type == "agent-file" || item.source_id.rsplit('/').next() == Some("AGENTS.md")
}
#[rustfmt::skip]
fn selected_cell<'a>(decision: &RuntimeDecision, snapshot: &'a RuntimeSnapshot) -> Result<&'a StateCell, String> {
    let label = decision.selected_state_key.as_deref().ok_or("decision has no selected state")?;
    snapshot.cells.values().find(|cell| cell.key.as_label() == label)
        .ok_or_else(|| "selected state is absent".to_string())
}
#[rustfmt::skip]
fn kernel_card(decision: &RuntimeDecision) -> String { format!(
    "<kernel>\n<case>{}</case>\n<operation>{}</operation>\n<rule>Use only decision-selected state, tools, and output grammar. Completion belongs to durable checks.</rule>\n</kernel>",
    escape(&decision.case_id), escape(&decision.operation.0)) }
#[rustfmt::skip]
fn state_card(cell: &StateCell, source_bodies: &BTreeSet<String>) -> String {
    let payload = serde_json::from_str::<Value>(&cell.payload_json).map(|value| payload_lines(&value, "", source_bodies)).unwrap_or_else(|_| "<payload_error>invalid durable payload</payload_error>".into());
    format!("<state>\n<key>{}</key>\n<status>{:?}</status>\n<schema>{}</schema>\n{}\n<event>{}</event>\n</state>",
        escape(&cell.key.as_label()), cell.status, escape(&cell.payload_schema), payload, escape(&cell.source_event_id))
}
#[rustfmt::skip]
fn payload_lines(value: &Value, path: &str, bodies: &BTreeSet<String>) -> String {
    match value {
        Value::Object(map) => map.iter().map(|(key, value)| payload_lines(value, &join_path(path, key), bodies)).collect::<Vec<_>>().join("\n"),
        Value::Array(values) => values.iter().enumerate().map(|(index, value)| payload_lines(value, &join_path(path, &index.to_string()), bodies)).collect::<Vec<_>>().join("\n"),
        Value::Null => field(path, "null", bodies), Value::Bool(v) => field(path, &v.to_string(), bodies),
        Value::Number(v) => field(path, &v.to_string(), bodies), Value::String(v) => field(path, v, bodies),
    }
}
#[rustfmt::skip]
fn join_path(path: &str, key: &str) -> String { if path.is_empty() { key.into() } else { format!("{path}.{key}") } }
#[rustfmt::skip]
fn field(path: &str, value: &str, bodies: &BTreeSet<String>) -> String { let value = if bodies.contains(&normalized_body(value)) { "source-linked" } else { value };
    format!("<field>\n<name>{}</name>\n<value>{}</value>\n</field>", escape(path), escape(value)) }
#[rustfmt::skip]
fn tools_card(decision: &RuntimeDecision) -> Result<String, String> { let fingerprint = decision.tool_view_fingerprint().map_err(|error| error.message)?;
    let schemas = decision.tool_view.entries.iter().map(|entry| format!("<tool>\n<name>{}</name>\n<purpose>{}</purpose>\n<required>{}</required>\n<optional>{}</optional>\n</tool>",
        escape(&entry.name), escape(&entry.purpose), escape(&entry.required_params.join(",")), escape(&entry.optional_params.join(",")))).collect::<Vec<_>>().join("\n");
    Ok(format!("<tools>\n<fingerprint>{fingerprint}</fingerprint>\n{schemas}\n</tools>")) }
#[rustfmt::skip]
fn output_card(decision: &RuntimeDecision) -> String { format!("<output>\n<protocol>{}</protocol>\n{}\n</output>",
    escape(protocol_for_envelope(decision.expected_envelope)), protocol_card(decision)) }
#[rustfmt::skip]
fn source_card(item: &ContextItem) -> String { format!("<source>\n<id>{}</id>\n<kind>{}</kind>\n<revision>{}</revision>\n<body>{}</body>\n</source>",
    escape(&item.id), escape(&item.source_type), escape(&item.source_fingerprint), escape(&item.body)) }
#[rustfmt::skip]
fn estimate_tokens(value: &str) -> u32 { let bytes = value.len().div_ceil(4); bytes.max(value.chars().count()).min(u32::MAX as usize) as u32 }
#[rustfmt::skip]
fn escape(value: &str) -> String { value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('\'', "&apos;").replace('"', "&quot;") }

#[rustfmt::skip]
pub fn build_prompt_card_plan(decision: &RuntimeDecision, prompt: &Prompt,
    context: &ContextFramePlan) -> Result<PromptCardPlan, FingerprintError> {
    let tool = decision.tool_view_fingerprint()?;
    let reasons = [("kernel",format!("case={} decision={} prompt-profile={} context-profile={}",decision.case_id,decision.id,PROMPT_PROFILE,CONTEXT_PROFILE)),
        ("objective",format!("operation={}",decision.operation.0)), ("state",format!("harness_state={} purpose={} snapshot={} state={}",decision.harness_state.as_str(),decision.harness_state.purpose(),decision.snapshot_fingerprint,decision.state_vector_fingerprint)),
        ("facts",facts_reason(decision,context)), ("conflicts",conflict_reason(context)), ("recovery",format!("policy-ref={}",stable_fingerprint(&decision.recovery_policy)?)),
        ("tools",format!("tool-view={} count={}",tool,decision.tool_view.entries.len())), ("output",format!("envelope={:?} stop={} max={}",decision.expected_envelope,prompt.stop,prompt.max_tokens))];
    let cards=reasons.into_iter().map(|(kind,reason)|card(kind,reason)).collect::<Result<Vec<_>,_>>()?;
    let fingerprint=stable_fingerprint(&(PROMPT_PROFILE,CONTEXT_PROFILE,&cards))?;
    Ok(PromptCardPlan{prompt_profile:PROMPT_PROFILE.into(),context_profile:CONTEXT_PROFILE.into(),cards,fingerprint})
}
#[rustfmt::skip]
fn facts_reason(decision: &RuntimeDecision, plan: &ContextFramePlan) -> String { format!("context={} included={} excluded={} lanes={} pipeline={}",decision.context_frame_fingerprint,entries(&plan.included),entries(&plan.excluded),plan.lanes.iter().map(|lane|format!("{}:{} refs={}",lane.name,lane.fingerprint,lane.source_refs.join("+"))).collect::<Vec<_>>().join(","),plan.pipeline.iter().map(|stage|format!("{}:{}",stage.name,stage.status)).collect::<Vec<_>>().join(",")) }
#[rustfmt::skip]
fn conflict_reason(plan: &ContextFramePlan) -> String { format!("unresolved={}",entries(&plan.excluded.iter().filter(|entry|entry.reason=="unresolved-conflict").cloned().collect::<Vec<_>>())) }
#[rustfmt::skip]
fn entries(values: &[ContextPlanEntry]) -> String { if values.is_empty(){return "none".into();} let mut out=values.iter().map(|entry|format!("<context_item><id>{}</id><reason>{}</reason><rank>{}</rank><source_ref>{}</source_ref></context_item>",escape(&entry.item_id),escape(&entry.reason),entry.rank,escape(&entry.source_ref))).collect::<Vec<_>>();out.sort();out.join(",") }
#[rustfmt::skip]
fn card(kind:&str,reason:String)->Result<PromptCard,FingerprintError>{let id=format!("card-{kind}");let fingerprint=stable_fingerprint(&(kind,&reason))?;Ok(PromptCard{id,kind:kind.into(),reason,fingerprint})}
