use lkjagent_core::runtime_context::{CONTEXT_LANES, CONTEXT_REGIONS};
use lkjagent_core::runtime_decision::{DECISION_SETTLEMENTS, DECISION_SPEC_FIELDS, MODEL_GRAMMARS};
use lkjagent_core::runtime_event::{RUNTIME_EVENT_KINDS, STATE_TRANSITION_EVENTS};
use lkjagent_core::runtime_recovery::RECOVERY_KINDS;
use lkjagent_core::runtime_selector::{EXIT_GUARDS, FILE_CHECK_KINDS};
#[rustfmt::skip]
use lkjagent_core::runtime_state::{StateKey, FAULT_KINDS, MATTER_STATES, NEED_KINDS, RUNTIME_PHASES, STATE_STATUSES, WAKE_KINDS};
use lkjagent_core::runtime_tool_call::{FINAL_FIELDS, MODEL_ENVELOPES, TOOL_CALL_FIELDS};
#[rustfmt::skip]
use lkjagent_core::runtime_tool_catalog::{direct_catalog, direct_tool_view, TOOL_DESCRIPTOR_FIELDS};
#[test]
fn contract_tables_are_exact_and_closed() {
    assert_eq!(
        STATE_STATUSES.join(","),
        "active,inactive,suppressed,resolved,blocked"
    );
    assert_eq!(MATTER_STATES, ["open", "waiting", "blocked", "closed"]);
    assert_eq!(
        RUNTIME_PHASES,
        ["orient", "modify", "review", "respond", "idle"]
    );
    assert_eq!(
        NEED_KINDS,
        [
            "target",
            "source-revision",
            "edit",
            "check",
            "response",
            "owner-fact"
        ]
    );
    assert_eq!(
        FAULT_KINDS,
        [
            "protocol",
            "admission",
            "stale-file",
            "effect",
            "endpoint",
            "check",
            "stasis"
        ]
    );
    assert_eq!(
        WAKE_KINDS,
        [
            "immediate",
            "time",
            "owner-input",
            "file-change",
            "config-change"
        ]
    );
    assert_eq!(
        RUNTIME_EVENT_KINDS,
        [
            "owner-turn",
            "wake",
            "provider-outcome",
            "effect-outcome",
            "file-change"
        ]
    );
    assert_eq!(
        STATE_TRANSITION_EVENTS,
        [
            "matter-opened",
            "source-need-met",
            "revision-observed",
            "measured-difference",
            "obligations-met",
            "close-eligible",
            "fault-recorded",
            "question-persisted"
        ]
    );
    assert_eq!(
        DECISION_SETTLEMENTS,
        [
            "selected",
            "compilation-complete",
            "provider-intent",
            "effect-prepared",
            "settled",
            "blocked"
        ]
    );
    assert_eq!(MODEL_GRAMMARS, ["tool-call", "final", "none"]);
    assert_eq!(MODEL_ENVELOPES, ["tool_call", "final"]);
    assert_eq!(TOOL_CALL_FIELDS, ["tool", "input"]);
    assert_eq!(FINAL_FIELDS, ["message"]);
}

#[test]
fn contract_tables_decision_context_check_recovery_and_exit_are_exact() {
    assert_eq!(
        DECISION_SPEC_FIELDS,
        [
            "selected-state",
            "operation",
            "tool-descriptors",
            "grammar",
            "information-needs",
            "context-caps",
            "model-budget",
            "recovery-policy",
            "check-requirements",
            "exit-policy"
        ]
    );
    assert_eq!(
        CONTEXT_REGIONS,
        [
            "identity-honesty",
            "phase-fault",
            "workspace-operation",
            "tools-grammar-example",
            "evidence",
            "owner-message"
        ]
    );
    assert_eq!(
        CONTEXT_LANES,
        [
            "objective-constraints",
            "file-evidence",
            "memory-history",
            "recovery-diagnosis",
            "output-reserve"
        ]
    );
    assert_eq!(
        FILE_CHECK_KINDS,
        [
            "regular-utf8",
            "intended-sha256",
            "occurrence-counts",
            "admitted-diff",
            "preserved-mode",
            "allowed-changed-paths",
            "effects-settled"
        ]
    );
    assert_eq!(
        RECOVERY_KINDS,
        [
            "protocol",
            "hidden-tool",
            "premature-final",
            "missing-read",
            "stale-or-ambiguous-edit",
            "output-limit",
            "endpoint",
            "check",
            "equal-progress"
        ]
    );
    assert_eq!(
        EXIT_GUARDS,
        [
            "required-current-checks-passed",
            "no-blocking-operation",
            "effects-settled",
            "final-message-persisted"
        ]
    );
}
#[test]
fn contract_tables_tool_catalog_and_open_keys_preserve_authority() {
    assert_eq!(
        TOOL_DESCRIPTOR_FIELDS.join(","),
        "name,purpose,field-order,required-flags,value-classes,byte-count-bounds,safe-example,state-affordances,admission-rules,effect-key,result-bound,denial-code"
    );
    let catalog = direct_catalog();
    let names = catalog.iter().map(|tool| tool.name).collect::<Vec<_>>();
    assert_eq!(
        names.join(","),
        "list_directory,search_text,read_file,edit_file,create_file"
    );
    #[rustfmt::skip]
    let field_names = |index: usize| catalog[index].fields.iter()
        .map(|field| field.name).collect::<Vec<_>>();
    assert_eq!(field_names(0), ["path", "offset", "count", "complete"]);
    assert_eq!(field_names(1), ["path", "query", "offset", "count"]);
    assert_eq!(field_names(2), ["path", "offset", "count", "complete"]);
    assert_eq!(field_names(3), ["path", "old_text", "new_text"]);
    assert_eq!(field_names(4), ["path", "content"]);
    assert!(catalog.iter().all(|tool| !tool.effect_key.is_empty()
        && tool.result_max_bytes > 0
        && !tool.denial_code.is_empty()));
    let mut sorted_names = names;
    sorted_names.sort();
    assert_eq!(direct_tool_view().tool_names(), sorted_names);
    let unknown = StateKey::from_label("future-capability:unrecognized/value");
    assert_eq!(
        unknown.map(|key| key.as_label()),
        Ok("future-capability:unrecognized/value".to_string())
    );
}
