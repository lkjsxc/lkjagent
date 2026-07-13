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
    fact("R10", "crates/lkjagent-store/src/direct_transactions.rs", &["ORDER BY CASE WHEN m.lifecycle='blocked'", "d.status NOT IN ('settled','failed')", "max(d.selected_monotonic_ms)"]),
    fact("R10", "crates/lkjagent-store/tests/native_direct_loop.rs", &["restart_projection_skips_blocked_matter_for_runnable_owner_turn", "Some(\"m2\")"]),
    fact("R10", "crates/lkjagent-store/tests/budget_epoch.rs", &["settled_turn_rotates_to_never_selected_owner_matter_across_reopen", "Some(\"m2\".into())"]),
    fact("S04", "crates/lkjagent-app/src/config_registry.rs", &["endpoint_api_key_env", "endpoint_timeout_seconds", "prompt_context_tokens", "workspace_root", "workspace_timezone"]),
    fact("S04", "crates/lkjagent-app/src/config.rs", &["load_client", "api_key", "prompt_max_context_tokens", "workspace_root", "workspace_timezone"]),
    fact("S04", "crates/lkjagent-app/src/public_loop.rs", &["fn prompt_budgets(", "workspace_timezone(data)"]),
    fact("S04", "crates/lkjagent-app/tests/configuration_contract.rs", &["tracked_example_matches_the_registry", "object.len(), 7", "workspace_timezone_is_consumed_and_strictly_bounded"]),
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
    fact("C06", "crates/lkjagent-core/src/runtime_context_plan.rs", &["item.contamination != ContaminationClass::Clean", "unresolved-conflict"]),
    fact("C06", "crates/lkjagent-core/tests/context_completion.rs", &["contradictions_render_as_conflicts_and_contamination_is_excluded", "FailedModelOutput"]),
    fact("C06", "crates/lkjagent-app/src/public_loop.rs", &["c.namespace==b\"source\"||c.namespace==b\"observation\"", "context_body", "format!(\"{}: {}\",line.number,line.text)"]),
    fact("C06", "crates/lkjagent-app/tests/public_loop.rs", &["instr(CAST(rendered_frame AS TEXT),'{&quot;')>0", "assert_eq!"]),
    fact("P07", "crates/lkjagent-llm/src/wire/response.rs", &["ProviderAnomalyKind::ToolCallOnlyResponse", "fn content_and_anomaly("]),
    fact("P07", "crates/lkjagent-llm/tests/wire_contract.rs", &["response_anomalies_remain_distinct", "ProviderAnomalyKind::ToolCallOnlyResponse"]),
    fact("X03", "crates/lkjagent-llm/src/error.rs", &["ResponseTooLarge", "Timeout", "Connect"]),
    fact("X03", "crates/lkjagent-llm/tests/wire_contract.rs", &["timeout_connect_and_status_are_distinct", "length_is_not_repaired_and_ambiguous_send_is_not_retried"]),
    fact("X04", "crates/lkjagent-store/src/matter_control.rs", &["pub fn block_budget(", "'matter-blocked'", "'block','budget'", "lifecycle='blocked'", "pub fn resume_blocked(", "'owner-resume'", "accounted_tokens_in_budget_epoch", "effects_in_budget_epoch", "recovery_cost_in_budget_epoch", "active_milliseconds_in_budget_epoch", "settlement_event_id=?1"]),
    fact("X04", "crates/lkjagent-app/src/public_loop.rs", &["MODEL_CALL_LIMIT", "TOKEN_BUDGET_LIMIT", "EFFECT_BUDGET_LIMIT", "RECOVERY_COST_LIMIT", "ACTIVE_MILLISECONDS_LIMIT", "exhausted {label} budget"]),
    fact("X04", "crates/lkjagent-store/tests/native_direct_loop.rs", &["exhausted_budget_is_visible_idempotent_and_not_idle", "effects_in_budget_epoch", "recovery_cost_in_budget_epoch", "active_milliseconds_in_budget_epoch", "Some(\"blocked\")"]),
    fact("X04", "crates/lkjagent-app/tests/cli.rs", &["blocked_matter_resumes_on_owner_send_unless_new_is_forced", "resumed=true"]),
    fact("X04", "crates/lkjagent-store/tests/budget_epoch.rs", &["owner_resume_starts_a_fresh_model_budget_epoch", "missing_provider_usage_remains_unknown", "accounted_tokens_in_budget_epoch"]),
    fact("K01", "crates/lkjagent-app/src/automatic_checks.rs", &["source_revision", "workspace-byte", "managed-journal"]),
    fact("K01", "crates/lkjagent-app/tests/journal_flow.rs", &["predicate_kind='managed-journal'", "checks WHERE current=1 AND passed=1"]),
    fact("K02", "crates/lkjagent-app/tests/automatic_checks.rs", &["later_bytes_invalidate_checks_and_block_final", "current=0", "assert!(close(&db).is_err())"]),
    fact("K03", "crates/lkjagent-store/src/native_schema.rs", &["fn completion_receipt(", "matter has blocking operation, effect, or check", "\"revision\":hex(&revision)"]),
    fact("K03", "crates/lkjagent-app/tests/journal_flow.rs", &["receipt.contains(&path)", "effect_journal WHERE status='settled'"]),
    fact("W04", "crates/lkjagent-effects/src/workspace_parents.rs", &["prepare_absent_edit", "create_declared_directories", "mkdirat"]),
    fact("W04", "crates/lkjagent-app/src/journal_checks.rs", &["max_token_units", "token_units", "known_placeholder"]),
    fact("W04", "crates/lkjagent-app/tests/journal_flow.rs", &["journal_path(&date)", "SELECT count(*) FROM effect_targets WHERE operation='mkdir'", "assert!(!second.join(\"workspace\").exists())"]),
    fact("W05-deterministic", "crates/lkjagent-app/src/journal_dispatch.rs", &["selected_wall_time", "workspace_timezone", "canonical_path", "source_fingerprints"]),
    fact("W05-deterministic", "crates/lkjagent-app/tests/journal_flow.rs", &["assert_ne!(date", "source_revision AS TEXT", "write_record"]),
    fact("T01", "crates/lkjagent-app/src/tui_screen.rs", &["pub struct ConversationItem", "let mut conversation = snapshot", "sequence: Some(row.sequence)", "id: row.id.clone()"]),
    fact("T01", "crates/lkjagent-app/tests/tui_contract.rs", &["identity_is_exact_once_and_never_body_based", "message(\"eventual\", 3"]),
    fact("T01", "crates/lkjagent-app/src/tui_render.rs", &[".rows(width.saturating_sub", "role(&row.role)", "tui_viewport::visible"]),
    fact("T02", "crates/lkjagent-store/src/tui_snapshot.rs", &["transaction_with_behavior(TransactionBehavior::Deferred)", "let conversation = conversation(", "let activity = activity(", "let status = status("]),
    fact("T02", "crates/lkjagent-store/src/tui_snapshot_tests.rs", &["wal_writer_commit_between_queries_does_not_split_frame", "snapshot_with"]),
    fact("T03", "crates/lkjagent-app/src/tui_screen.rs", &["pub conversation: Vec<ConversationItem>", "pub activity: ActivityPanel", "expanded: false"]),
    fact("T03", "crates/lkjagent-store/src/tui_snapshot_tests.rs", &["canonical_order_and_conversation_activity_separation", "secret-prompt", "secret-payload"]),
    fact("T03", "crates/lkjagent-app/tests/tui_native.rs", &["renderer_clips_and_keeps_activity_out_of_conversation", "!line.contains(\"selected\")"]),
    fact("T04", "crates/lkjagent-store/src/tui_snapshot_tests.rs", &["activity_ids_are_unique_and_stable_across_polls", "first_ids.len()"]),
    fact("T04", "crates/lkjagent-app/src/tui_screen.rs", &["items: snapshot.activity.iter().map(activity_item).collect()", "let expanded = self.activity.expanded"]),
    fact("T05", "crates/lkjagent-app/src/tui_viewport.rs", &["Viewport::Follow => maximum", "if target == maximum", "Viewport::Follow"]),
    fact("T05", "crates/lkjagent-app/tests/tui_contract.rs", &["follow_and_manual_append_have_durable_anchors", "scroll(&mut viewport, &appended, 2, 99)"]),
    fact("T06", "crates/lkjagent-app/src/tui_viewport.rs", &["pub struct Anchor", "message_id", "wrapped_row", "pub fn reconcile"]),
    fact("T06", "crates/lkjagent-app/tests/tui_contract.rs", &["resize_search_shrink_and_overscroll_clamp_without_blank_windows", "assert!(!visible"]),
    fact("T07", "crates/lkjagent-app/src/tui_runtime.rs", &["send_message(data_dir, &body, false)", "SubmitCommitted", "worker.wake()"]),
    fact("T07", "crates/lkjagent-app/tests/tui_native.rs", &["typed_intake_uses_durable_id_and_preserves_failed_text", "message_id: receipt.message_id.clone()"]),
    fact("T07", "crates/lkjagent-app/tests/tui_responsive.rs", &["input_remains_reducible_while_worker_endpoint_is_blocked", "Event::Resize", "KeyCode::F(2)"]),
    fact("T08", "crates/lkjagent-app/src/tui_worker.rs", &["thread::Builder::new()", "public_loop::run_once", "commands.recv()"]),
    fact("T08", "crates/lkjagent-app/tests/tui_responsive.rs", &["input_remains_reducible_while_worker_endpoint_is_blocked", "Event::Paste(\"日本語\"", "Event::Paste(\"検索\""]),
    fact("T08", "crates/lkjagent-app/tests/tui_pty.rs", &["native_binary_enters_and_restores_a_unix_pty", "b\"\\x1b[?1049l\""]),
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
 const PATHS:&[&str]=&["crates/lkjagent-core/src/owner_turn.rs","crates/lkjagent-core/src/plan.rs","crates/lkjagent-core/src/templates.rs","crates/lkjagent-core/src/artifact_manifest.rs","crates/lkjagent-core/src/runtime_artifact.rs","crates/lkjagent-core/src/workspace_manifest.rs","crates/lkjagent-core/src/workspace_record.rs","crates/lkjagent-app/src/daemon_route_effects.rs","crates/lkjagent-app/src/runtime_bridge.rs","crates/lkjagent-app/src/workspace_search/inventory.rs","crates/lkjagent-store/src/plan_schema.rs"];
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
