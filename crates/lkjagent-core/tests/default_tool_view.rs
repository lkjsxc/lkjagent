use lkjagent_core::runtime_decision::OutputEnvelope;
use lkjagent_core::runtime_operation::{RuntimePolicy, Selection};
use lkjagent_core::runtime_selector::select;
use lkjagent_core::runtime_state::{
    CurrentTime, RuntimeSnapshot, RuntimeState, StateCell, StateKey,
};
use lkjagent_core::runtime_tool_catalog::direct_tool_view_for_state;

#[test]
fn direct_views_are_exact_for_every_phase() {
    assert_eq!(
        names("orient", None),
        ["list_directory", "read_file", "search_text", "write_record"]
    );
    assert_eq!(
        names("modify", None),
        ["create_file", "edit_file", "read_file", "write_record"]
    );
    assert_eq!(names("recovery", Some("edit_file")), ["edit_file"]);
    for state in ["review", "respond", "wait", "idle"] {
        assert!(names(state, None).is_empty(), "{state} exposed tools");
    }
}

#[test]
fn direct_views_hide_other_states_retired_names_and_unintended_recovery_tools() {
    let orient = names("orient", None);
    assert!(!orient
        .iter()
        .any(|name| name.starts_with("fs.") || name == "edit_file"));
    let recovery = names("recovery", Some("read_file"));
    assert_eq!(recovery, ["read_file"]);
    assert!(names("recovery", Some("fs.read")).is_empty());
    assert!(names("recovery", None).is_empty());
}

#[test]
fn direct_view_fingerprints_are_stable_and_phase_specific() -> Result<(), String> {
    let orient = direct_tool_view_for_state("orient", None);
    let repeated = direct_tool_view_for_state("orient", None);
    let modify = direct_tool_view_for_state("modify", None);
    let orient_fingerprint = orient.fingerprint().map_err(|error| error.message)?;
    assert_eq!(
        orient_fingerprint,
        repeated.fingerprint().map_err(|error| error.message)?
    );
    assert_ne!(
        orient_fingerprint,
        modify.fingerprint().map_err(|error| error.message)?
    );
    assert_ne!(
        orient_fingerprint,
        direct_tool_view_for_state("review", None)
            .fingerprint()
            .map_err(|error| error.message)?
    );
    Ok(())
}

#[test]
fn fresh_matter_decision_uses_action_and_only_orient_tools() -> Result<(), String> {
    let selected = select(
        state_with("matter", "opened")?,
        RuntimePolicy::default(),
        now(),
    );
    assert!(matches!(selected, Selection::Decision(spec)
        if spec.expected_envelope == OutputEnvelope::Action
        && spec.tool_view == direct_tool_view_for_state("orient", None)));
    Ok(())
}

#[test]
fn selector_decisions_use_modify_review_and_respond_projections() -> Result<(), String> {
    for (namespace, name, phase) in [
        ("source", "current", "modify"),
        ("report", "pending", "modify"),
        ("edit", "committed", "review"),
        ("check", "current-passed", "respond"),
    ] {
        let selected = select(
            state_with(namespace, name)?,
            RuntimePolicy::default(),
            now(),
        );
        assert!(matches!(selected, Selection::Decision(spec)
            if spec.tool_view == direct_tool_view_for_state(phase, None)));
    }
    Ok(())
}

#[test]
fn recovery_decision_persists_only_the_intended_tool() -> Result<(), String> {
    let policy = RuntimePolicy {
        intended_recovery_tool: Some("edit_file".into()),
        ..RuntimePolicy::default()
    };
    let selected = select(state_with("fault", "protocol")?, policy, now());
    assert!(matches!(selected, Selection::Decision(spec)
        if spec.tool_view == direct_tool_view_for_state("recovery", Some("edit_file"))));
    Ok(())
}

fn names(state: &str, intended: Option<&str>) -> Vec<String> {
    direct_tool_view_for_state(state, intended).tool_names()
}
fn state_with(namespace: &str, name: &str) -> Result<RuntimeState, String> {
    let key = StateKey::new(namespace, name).map_err(|error| error.message)?;
    let mut snapshot = RuntimeSnapshot::empty("matter-1");
    snapshot
        .cells
        .insert(key.clone(), StateCell::active(key, "event-1"));
    Ok(RuntimeState::from_snapshot(snapshot))
}
fn now() -> CurrentTime {
    CurrentTime::new("2026-07-12T10:00:00Z")
}
