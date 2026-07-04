use std::collections::BTreeMap;

use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_core::runtime_event::{apply_patch, reduce_event, RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};

#[test]
fn arbitrary_unknown_state_cells_round_trip_deterministically() {
    let initial = RuntimeSnapshot::empty("case-1");
    let first = event_with_cell("event-1", cell("custom", "alpha"));
    let second = event_with_cell("event-2", cell("tool", "fs-read-needed"));

    let after_first = apply_patch(&initial, &reduce_event(&initial, &first));
    let after_second = apply_patch(&after_first, &reduce_event(&after_first, &second));
    let replay_first = apply_patch(&initial, &reduce_event(&initial, &first));
    let replay_second = apply_patch(&replay_first, &reduce_event(&replay_first, &second));

    assert_eq!(after_second, replay_second);
    assert_eq!(after_second.active_cells().len(), 2);
    assert!(after_second.cells.contains_key(&key("custom", "alpha")));
    assert_eq!(
        snapshot_fingerprint(&after_second),
        snapshot_fingerprint(&replay_second)
    );
}

#[test]
fn decision_fingerprint_is_stable_for_canonical_tool_view() {
    let left = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![finish_tool(), read_tool()]),
        OutputEnvelope::Action,
    );
    let right = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![read_tool(), finish_tool()]),
        OutputEnvelope::Action,
    );

    assert_eq!(decision_fingerprint(&left), decision_fingerprint(&right));
    assert_eq!(left.tool_view.tool_names(), vec!["finish", "fs.read"]);
}

#[test]
fn rendered_tool_view_and_admission_match() {
    let decision = decision_with_tools(vec![read_tool(), finish_tool()]);
    let read_action = action("fs.read", vec![("path", "docs/current-state.md")]);
    let admitted = admit(&decision, &read_action);
    let rejected = admit(&decision, &action("shell.run", vec![("command", "pwd")]));

    assert_eq!(admitted.status, AdmissionStatus::Admitted);
    assert_eq!(admitted.tool_view_fingerprint, tool_fingerprint(&decision));
    assert_eq!(rejected.status, AdmissionStatus::Rejected);
    assert_eq!(rejected.reason, "tool absent from decision view");
    assert!(!decision
        .tool_view
        .tool_names()
        .contains(&"shell.run".to_string()));
}

#[test]
fn workspace_policy_blocks_path_escapes() {
    let decision = decision_with_tools(vec![read_tool()]);
    let parent = admit(&decision, &action("fs.read", vec![("path", "../secret")]));
    let absolute = admit(&decision, &action("fs.read", vec![("path", "/tmp/secret")]));

    assert_eq!(parent.status, AdmissionStatus::Rejected);
    assert_eq!(absolute.status, AdmissionStatus::Rejected);
    assert_eq!(parent.reason, "path escapes workspace");
    assert_eq!(absolute.reason, "path escapes workspace");
}

fn key(namespace: &str, name: &str) -> StateKey {
    let result = StateKey::new(namespace, name);
    assert!(result.is_ok());
    match result {
        Ok(key) => key,
        Err(_) => StateKey {
            namespace: namespace.to_string(),
            name: name.to_string(),
        },
    }
}

fn cell(namespace: &str, name: &str) -> StateCell {
    StateCell::active(key(namespace, name), "source-event")
}

fn event_with_cell(id: &str, cell: StateCell) -> RuntimeEvent {
    RuntimeEvent {
        id: id.to_string(),
        case_id: "case-1".to_string(),
        kind: "state.cell.upsert".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "test".to_string(),
        created_at: "now".to_string(),
        decision_id: None,
    }
}

fn read_tool() -> ToolViewEntry {
    ToolViewEntry::new("fs.read", "read a workspace file").with_params(vec!["path"], vec!["count"])
}

fn finish_tool() -> ToolViewEntry {
    ToolViewEntry::new("finish", "finish exploration").with_params(vec!["summary"], Vec::new())
}

fn decision_with_tools(entries: Vec<ToolViewEntry>) -> RuntimeDecision {
    RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(entries),
        OutputEnvelope::Action,
    )
}

fn action(tool: &str, params: Vec<(&str, &str)>) -> ModelAction {
    ModelAction {
        tool: tool.to_string(),
        params: params
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<BTreeMap<_, _>>(),
    }
}

fn admit(
    decision: &RuntimeDecision,
    action: &ModelAction,
) -> lkjagent_core::runtime_admission::ToolAdmission {
    let result = admit_action(decision, action);
    assert!(result.is_ok());
    match result {
        Ok(admission) => admission,
        Err(_) => lkjagent_core::runtime_admission::ToolAdmission {
            decision_id: String::new(),
            tool_view_fingerprint: String::new(),
            action_tool: String::new(),
            status: AdmissionStatus::Rejected,
            reason: String::new(),
        },
    }
}

fn snapshot_fingerprint(snapshot: &RuntimeSnapshot) -> String {
    let result = snapshot.fingerprint();
    assert!(result.is_ok());
    result.unwrap_or_default()
}

fn decision_fingerprint(decision: &RuntimeDecision) -> String {
    let result = decision.fingerprint();
    assert!(result.is_ok());
    result.unwrap_or_default()
}

fn tool_fingerprint(decision: &RuntimeDecision) -> String {
    let result = decision.tool_view_fingerprint();
    assert!(result.is_ok());
    result.unwrap_or_default()
}
