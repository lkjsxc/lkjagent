use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

struct Fact {
    id: &'static str,
    path: &'static str,
    needles: &'static [&'static str],
}

#[rustfmt::skip]
const FACTS: &[Fact] = &[
    fact("F03", "crates/lkjagent-effects/src/workspace_capability.rs", &["pub fn read_file(", "OFlags::NOFOLLOW", "require_type(&fd, FileType::RegularFile)"]),
    fact("F03", "crates/lkjagent-effects/tests/workspace_safety_paths.rs", &["workspace_safety_rejects_traversal_reserved_and_symlinks", "workspace_safety_special_files_return_promptly"]),
    fact("F04", "crates/lkjagent-effects/src/workspace_edit.rs", &["pub fn prepare_exact_edit(", "old_text must match exactly once", "Revision::Absent"]),
    fact("F04", "crates/lkjagent-effects/tests/workspace_edit.rs", &["preparation_rejects_unbound_or_unsafe_edits", "Revision::Sha256(\"stale\".into())"]),
    fact("R01", "crates/lkjagent-store/src/transactions.rs", &["pub fn owner_intake(", "self.atomic(|tx|", "INSERT INTO conversation_messages"]),
    fact("R01", "crates/lkjagent-store/tests/native_transactions.rs", &["durable_boundaries_intake_is_atomic_and_idempotency_is_typed", "durable_boundaries_injected_statement_failure_rolls_back_intake"]),
    fact("R02", "crates/lkjagent-store/src/transactions.rs", &["pub fn provider_intent(", "pub fn prepare_effect(", "decision compilation is incomplete"]),
    fact("R02", "crates/lkjagent-store/tests/native_transactions.rs", &["durable_boundaries_compiles_before_provider_and_sent_is_not_replayed", "durable_boundaries_effect_prep_is_complete_and_phases_do_not_skip"]),
    fact("R03", "crates/lkjagent-store/src/native-schema.sql", &["CREATE TABLE runtime_decisions", "selected_state BLOB NOT NULL", "exit_spec BLOB NOT NULL"]),
    fact("R03", "crates/lkjagent-core/tests/contract_tables.rs", &["contract_tables_decision_context_check_recovery_and_exit_are_exact", "DECISION_SPEC_FIELDS"]),
    fact("R04", "crates/lkjagent-store/src/transactions.rs", &["pub fn attach_compilation(", "compiler_status='complete'", "pub fn provider_intent("]),
    fact("R04", "crates/lkjagent-store/tests/native_transactions.rs", &["durable_boundaries_compiles_before_provider_and_sent_is_not_replayed"]),
    fact("R06", "crates/lkjagent-core/src/runtime_event.rs", &["pub fn reduce(", "ReduceFault::CausalSequence"]),
    fact("R06", "crates/lkjagent-core/src/runtime_selector.rs", &["pub fn select(", "Selection::Idle"]),
    fact("R06", "crates/lkjagent-core/tests/runtime_reducer_selector.rs", &["reducer_preserves_unknown_and_invalidates_old_revision", "direct_transitions_do_not_add_model_review"]),
    fact("R10", "crates/lkjagent-store/src/direct_transactions.rs", &["ORDER BY CASE WHEN EXISTS", "d.status NOT IN ('settled','failed')"]),
    fact("R10", "crates/lkjagent-store/tests/native_direct_loop.rs", &["restart_projection_skips_blocked_matter_for_runnable_owner_turn", "Some(\"m2\")"]),
    fact("S04", "crates/lkjagent-app/src/config_registry.rs", &["endpoint_api_key_env", "endpoint_timeout_seconds", "prompt_context_tokens", "workspace_root"]),
    fact("S04", "crates/lkjagent-app/src/config.rs", &["load_client", "api_key", "prompt_max_context_tokens", "workspace_root"]),
    fact("S04", "crates/lkjagent-app/src/public_loop.rs", &["fn prompt_budgets(", "prompt_max_context_tokens"]),
    fact("S04", "crates/lkjagent-app/tests/configuration_contract.rs", &["tracked_example_matches_the_registry", "object.len(), 6", "rejects_unknown_composite_and_wrong_type_values"]),
    fact("P01", "crates/lkjagent-core/src/runtime_tool_call.rs", &["pub fn parse_model_value", "MODEL_ENVELOPES", "exact_children"]),
    fact("P01", "crates/lkjagent-core/tests/direct_action_grammar.rs", &["contract_tables_reject_roots_prose_and_legacy_actions", "\\\"tool\\\":\\\"read_file\\\""]),
    fact("P02", "crates/lkjagent-core/tests/direct_action_grammar.rs", &["<decision_id>x</decision_id>", "ToolCallError::UnknownTag"]),
    fact("R05", "crates/lkjagent-core/src/runtime_tool_catalog.rs", &["const DIRECT_CATALOG", "direct_tool_view_for_state", "descriptor_entry"]),
    fact("R05", "crates/lkjagent-core/src/runtime_prompt_kernel.rs", &["fn tools_card", "decision.tool_view.entries"]),
    fact("R05", "crates/lkjagent-core/src/runtime_admission.rs", &["decision.tool_view_fingerprint()", "decision.tool_view.entry(&action.tool)"]),
    fact("R05", "crates/lkjagent-core/tests/admission.rs", &["tool_field_specs_drive_value_class_admission", "state_views_and_effect_keys_are_closed"]),
    fact("R05", "crates/lkjagent-core/tests/tool_call.rs", &["contract_tables_accept_descriptor_order_and_text", "contract_tables_reject_hidden_missing_unknown_and_order"]),
    fact("P03", "crates/lkjagent-core/src/runtime_tool_catalog.rs", &["const DIRECT_CATALOG", "direct_tool_view_for_state"]),
    fact("P03", "crates/lkjagent-core/src/runtime_admission.rs", &["decision.tool_view_fingerprint()", "decision.tool_view.entry(&action.tool)"]),
    fact("P03", "crates/lkjagent-core/tests/admission.rs", &["state_views_and_effect_keys_are_closed"]),
    fact("P03", "crates/lkjagent-core/tests/tool_call.rs", &["contract_tables_reject_hidden_missing_unknown_and_order"]),
    fact("P04", "crates/lkjagent-app/src/public_loop.rs", &["final_claims_allowed", "unsupported future or command claim in final wording"]),
    fact("P04", "crates/lkjagent-app/tests/public_loop.rs", &["I will update the exact phrase", "Ready to report the checked phrase"]),
    fact("P05", "crates/lkjagent-app/src/automatic_checks.rs", &["pub fn reduce_committed_edit(", "workspace-byte", "workspace-content", "workspace-collateral"]),
    fact("P05", "crates/lkjagent-app/src/public_loop.rs", &["automatic_checks::reduce_committed_edit"]),
    fact("P05", "crates/lkjagent-app/tests/public_loop.rs", &["SELECT count(*) FROM checks WHERE current=1 AND passed=1"]),
    fact("P06", "crates/lkjagent-store/src/native_schema.rs", &["fn completion_receipt(", "receipt_fingerprint"]),
    fact("P06", "crates/lkjagent-app/src/public_loop.rs", &["Completed with current harness checks.", "final_claims_allowed"]),
    fact("P06", "crates/lkjagent-app/tests/public_loop.rs", &["role='agent' AND receipt IS NOT NULL", "<final>"]),
    fact("C01", "crates/lkjagent-core/src/runtime_prompt_kernel.rs", &["current owner objective was suppressed", "select_context_plan"]),
    fact("C01", "crates/lkjagent-core/tests/prompt_kernel.rs", &["compiler_binds_selected_state_and_escapes_sources_once", "matches(\"Edit &lt;file&gt;\").count(), 1"]),
    fact("C02", "crates/lkjagent-core/src/runtime_prompt_kernel.rs", &["normalized_body", "select_context_plan"]),
    fact("C02", "crates/lkjagent-core/tests/prompt_kernel.rs", &["compiler_binds_selected_state_and_escapes_sources_once", "Observed &amp; current"]),
    fact("P07", "crates/lkjagent-llm/src/wire/response.rs", &["ProviderAnomalyKind::ToolCallOnlyResponse", "fn content_and_anomaly("]),
    fact("P07", "crates/lkjagent-llm/tests/wire_contract.rs", &["response_anomalies_remain_distinct", "ProviderAnomalyKind::ToolCallOnlyResponse"]),
    fact("X03", "crates/lkjagent-llm/src/error.rs", &["ResponseTooLarge", "Timeout", "Connect"]),
    fact("X03", "crates/lkjagent-llm/tests/wire_contract.rs", &["timeout_connect_and_status_are_distinct", "length_is_not_repaired_and_ambiguous_send_is_not_retried"]),
];

const fn fact(id: &'static str, path: &'static str, needles: &'static [&'static str]) -> Fact {
    Fact { id, path, needles }
}

pub fn contract_files() -> Vec<&'static str> {
    FACTS
        .iter()
        .map(|fact| fact.path)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn contract_derivations(root: &Path) -> BTreeSet<String> {
    let mut complete = BTreeMap::<&str, bool>::new();
    for fact in FACTS {
        let text = fs::read_to_string(root.join(fact.path)).ok();
        let present = text
            .as_ref()
            .is_some_and(|value| fact.needles.iter().all(|needle| value.contains(needle)));
        complete
            .entry(fact.id)
            .and_modify(|value| *value &= present)
            .or_insert(present);
    }
    let mut derived = complete
        .into_iter()
        .filter(|(_, complete)| *complete)
        .map(|(id, _)| id.to_string())
        .collect::<BTreeSet<_>>();
    if retired_authority_absent(root) {
        derived.insert("S02".into());
    }
    derived
}

#[rustfmt::skip]
fn retired_authority_absent(root:&Path)->bool{
 const PATHS:&[&str]=&["crates/lkjagent-core/src/owner_turn.rs","crates/lkjagent-core/src/plan.rs","crates/lkjagent-core/src/templates.rs","crates/lkjagent-app/src/daemon_route_effects.rs","crates/lkjagent-app/src/runtime_bridge.rs","crates/lkjagent-store/src/plan_schema.rs"];
 if PATHS.iter().any(|path|root.join(path).exists()){return false}
 const TOKENS:&[&str]=&["CREATE TABLE tasks","CREATE TABLE steps","Command::Workbench","Command::Record","pub mod plan_schema","pub mod runtime_bridge"];
 ["crates/lkjagent-core/src","crates/lkjagent-app/src","crates/lkjagent-store/src"].iter().flat_map(|dir|fs::read_dir(root.join(dir)).into_iter().flatten().flatten()).filter(|row|row.path().extension().and_then(|x|x.to_str())==Some("rs")).all(|row|fs::read_to_string(row.path()).is_ok_and(|text|TOKENS.iter().all(|token|!text.contains(token))))
}

pub fn validate(root: &Path, source: &str) -> Result<(), String> {
    validate_shape(source)?;
    let revision = format!("{source}^{{commit}}");
    let resolved = git(root, &["rev-parse", "--verify", &revision])?;
    if !resolved.status.success() || String::from_utf8_lossy(&resolved.stdout).trim() != source {
        return Err("source is not an exact reachable commit".to_string());
    }
    let ancestor = git(root, &["merge-base", "--is-ancestor", source, "HEAD"])?;
    if !ancestor.status.success() {
        return Err("source is not an ancestor of Git HEAD".to_string());
    }
    validate_later_paths(root, source)
}

fn validate_shape(source: &str) -> Result<(), String> {
    if source.len() != 40
        || !source
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("source must be a full lowercase Git commit ID".to_string());
    }
    Ok(())
}

fn validate_later_paths(root: &Path, source: &str) -> Result<(), String> {
    let range = format!("{source}..HEAD");
    let output = git(root, &["diff", "--name-only", "-z", &range, "--"])?;
    if !output.status.success() {
        return Err("cannot compare source with Git HEAD".to_string());
    }
    let allowed = format!("evaluation/evidence/{source}/");
    let changed = output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path))
        .find(|path| !path.starts_with(&allowed));
    match changed {
        Some(path) => Err(format!(
            "Git HEAD changed outside source evidence after freeze: {path}"
        )),
        None => Ok(()),
    }
}

fn git(root: &Path, args: &[&str]) -> Result<Output, String> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))
}
