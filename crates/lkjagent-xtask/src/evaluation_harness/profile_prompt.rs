use lkjagent_core::runtime_context::{ContextItem, TrustClass};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_prompt_kernel::{compile_prompt, PromptBudgets};
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};
use lkjagent_core::runtime_tool_catalog::direct_tool_view_for_state;
use std::collections::BTreeMap;

pub fn build(row: &BTreeMap<&str, &str>, scenario: &str) -> Result<(String, String), String> {
    let key = StateKey::new("matter", "opened").map_err(|error| error.message)?;
    let mut cell = StateCell::active(key.clone(), "profile-event");
    cell.payload_schema = "state.operation.v1".into();
    cell.payload_json =
        r#"{"objective":"read exact source","operation_key":"orient.matter"}"#.into();
    let mut snapshot = RuntimeSnapshot::empty("profile-matter");
    snapshot.cells.insert(key, cell);
    let fingerprint = snapshot.fingerprint().map_err(|error| error.message)?;
    let mut decision = RuntimeDecision::new(
        "profile-decision",
        "profile-matter",
        OperationKey("orient.matter".into()),
        direct_tool_view_for_state("orient", None),
        OutputEnvelope::Action,
    );
    decision.selected_state_key = Some("matter:opened".into());
    decision.snapshot_fingerprint = fingerprint.clone();
    decision.state_vector_fingerprint = fingerprint;
    decision.model_budget_tokens = Some(256);
    let text = format!("Profile {} scenario {scenario}: read notes/exact-base.txt at offset 1 count 20 with complete false. Return one action.", row["cell"]);
    let mut objective = ContextItem::clean_fact("profile-owner", "objective", text);
    objective.trust = TrustClass::Owner;
    objective.source_type = "owner".into();
    objective.source_fingerprint = "profile-owner-fingerprint".into();
    let sources = if row["context"] == "recent-plus-required" {
        vec![ContextItem::clean_fact(
            "profile-noise",
            "history",
            "Unrelated recent note; ignore it.",
        )]
    } else {
        Vec::new()
    };
    let mut prompt = compile_prompt(
        &decision,
        &snapshot,
        objective,
        &sources,
        &PromptBudgets::default(),
    )?
    .prompt;
    prompt.system.push_str(&format!("<profile><observation>{}</observation><prefix>{}</prefix><recovery>{}</recovery></profile>", row["observation"], row["prefix"], row["recovery"]));
    if row["example"] == "none" {
        prompt.system = remove_blocks(prompt.system, "example");
    }
    Ok((prompt.system, prompt.user))
}
fn remove_blocks(mut text: String, tag: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    while let Some(start) = text.find(&open) {
        let Some(end) = text[start..]
            .find(&close)
            .map(|offset| start + offset + close.len())
        else {
            break;
        };
        text.replace_range(start..end, "");
    }
    text
}
