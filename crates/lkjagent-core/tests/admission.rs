use std::collections::BTreeMap;

use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolValueClass, ToolViewEntry,
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

#[test]
fn rejects_empty_required_values_before_effects() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );

    let result = admit_action(&decision, &action("fs.read", "path", "   "))
        .map_err(|error| error.message)?;

    assert_eq!(result.status, AdmissionStatus::Rejected);
    assert_eq!(result.reason, "empty value for path");
    Ok(())
}

#[test]
fn tool_field_specs_drive_value_class_admission() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], vec!["count"])
        ]),
        OutputEnvelope::Action,
    );
    let entry = decision.tool_view.entry("fs.read").ok_or("missing tool")?;
    assert_eq!(
        entry.field_spec("path").map(|spec| spec.value_class),
        Some(ToolValueClass::WorkspacePath)
    );
    assert_eq!(
        entry.field_spec("count").map(|spec| spec.value_class),
        Some(ToolValueClass::Count)
    );

    let bad = admit_action(
        &decision,
        &action_params("fs.read", vec![("path", "README.md"), ("count", "many")]),
    )
    .map_err(|error| error.message)?;

    assert_eq!(bad.status, AdmissionStatus::Rejected);
    assert_eq!(bad.reason, "invalid count for count");
    Ok(())
}

#[test]
fn missing_tool_records_prompt_admission_mismatch() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );

    let result = admit_action(&decision, &action("shell.run", "cmd", "date"))
        .map_err(|error| error.message)?;

    assert_eq!(result.status, AdmissionStatus::Rejected);
    assert_eq!(
        result.reason,
        "tool-view mismatch: shell.run absent from decision view"
    );
    Ok(())
}

fn action(tool: &str, field: &str, value: &str) -> ModelAction {
    action_params(tool, vec![(field, value)])
}

fn action_params(tool: &str, params: Vec<(&str, &str)>) -> ModelAction {
    ModelAction {
        tool: tool.to_string(),
        params: params
            .into_iter()
            .map(|(field, value)| (field.to_string(), value.to_string()))
            .collect::<BTreeMap<_, _>>(),
    }
}
