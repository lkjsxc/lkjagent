use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};

#[test]
fn persisted_view_discards_catalog_excluded_entries() -> Result<(), String> {
    let key = StateKey::new("model", "42").map_err(|error| error.message)?;
    let mut cell = StateCell::active(key.clone(), "event-1");
    cell.payload_schema = "state.model".to_string();
    cell.payload_json = serde_json::json!({
        "expected_envelope": "Action",
        "tool_budget_remaining": 1,
        "tool_view": [
            {"name": "finish", "purpose": "removed", "required_params": ["summary"]},
            {"name": "read_file", "purpose": "read", "required_params": ["path"]}
        ]
    })
    .to_string();
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    snapshot.cells.insert(key, cell);

    let decision = select_runtime_decision(&snapshot, "decision-1", "ctx-1", &[])
        .map_err(|error| error.message)?;

    assert_eq!(decision.tool_view.tool_names(), vec!["read_file"]);
    Ok(())
}
