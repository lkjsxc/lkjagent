use std::collections::BTreeMap;

use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};

#[test]
fn rejects_placeholder_values_before_effects() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new()),
            ToolViewEntry::new("finish", "finish").with_params(vec!["summary"], Vec::new()),
        ]),
        OutputEnvelope::Action,
    );

    let path = admit_action(&decision, &action("fs.read", "path", "FIELD_VALUE"))
        .map_err(|error| error.message)?;
    let summary = admit_action(&decision, &action("finish", "summary", "TODO"))
        .map_err(|error| error.message)?;

    assert_eq!(path.status, AdmissionStatus::Rejected);
    assert_eq!(path.reason, "placeholder value for path");
    assert_eq!(summary.status, AdmissionStatus::Rejected);
    assert_eq!(summary.reason, "placeholder value for summary");
    Ok(())
}

fn action(tool: &str, field: &str, value: &str) -> ModelAction {
    ModelAction {
        tool: tool.to_string(),
        params: BTreeMap::from([(field.to_string(), value.to_string())]),
    }
}
