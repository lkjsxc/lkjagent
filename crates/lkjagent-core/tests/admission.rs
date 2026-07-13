use std::collections::BTreeMap;

use lkjagent_core::runtime_admission::{
    admit_action, dispatch_effect_key, AdmissionStatus, ModelAction,
};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolValueClass, ToolViewEntry,
};
use lkjagent_core::runtime_tool_catalog::{direct_tool_view, direct_tool_view_for_state};
use lkjagent_core::runtime_tool_view::EffectKey;

#[test]
fn rejects_placeholder_values_before_effects() -> Result<(), String> {
    let decision = direct_decision();
    let result = admit_action(
        &decision,
        &action_params(
            "read_file",
            vec![("path", "FIELD_VALUE"), ("complete", "false")],
        ),
    )
    .map_err(|error| error.message)?;
    assert_eq!(result.status, AdmissionStatus::Rejected);
    assert_eq!(result.reason, "placeholder value for path");
    Ok(())
}

#[test]
fn incomplete_persisted_projection_is_not_admitted() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".into()),
        ToolSetView::new(vec![
            ToolViewEntry::new("finish", "removed action").with_params(vec!["summary"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    let result = admit_action(&decision, &action("finish", "summary", "done"))
        .map_err(|error| error.message)?;
    assert_eq!(result.status, AdmissionStatus::Rejected);
    assert_eq!(result.reason, "incomplete persisted tool projection");
    Ok(())
}

#[test]
fn rejects_empty_required_values_before_effects() -> Result<(), String> {
    let result = admit_action(
        &direct_decision(),
        &action_params("read_file", vec![("path", "   "), ("complete", "false")]),
    )
    .map_err(|error| error.message)?;
    assert_eq!(result.status, AdmissionStatus::Rejected);
    assert_eq!(result.reason, "empty value for path");
    Ok(())
}

#[test]
fn tool_field_specs_drive_value_class_admission() -> Result<(), String> {
    let decision = direct_decision();
    let entry = decision
        .tool_view
        .entry("read_file")
        .ok_or("missing tool")?;
    let path = entry.field_spec("path").ok_or("missing path field")?;
    assert_eq!(path.value_class, ToolValueClass::WorkspacePath);
    assert_eq!((path.min_bytes, path.max_bytes), (1, 1024));
    let count = entry.field_spec("count").ok_or("missing count field")?;
    assert_eq!((count.minimum, count.maximum), (Some(1), Some(120)));
    for value in ["many", "01", "0", "121"] {
        let bad = admit_action(
            &decision,
            &action_params(
                "read_file",
                vec![
                    ("path", "README.md"),
                    ("count", value),
                    ("complete", "false"),
                ],
            ),
        )
        .map_err(|error| error.message)?;
        assert_eq!(bad.status, AdmissionStatus::Rejected, "value={value}");
        let expected = if value == "many" {
            "value size out of bounds for count"
        } else {
            "invalid count for count"
        };
        assert_eq!(bad.reason, expected);
    }
    Ok(())
}

#[test]
fn hidden_tool_is_rejected_from_persisted_view() -> Result<(), String> {
    let result = admit_action(&direct_decision(), &action("shell.run", "command", "date"))
        .map_err(|error| error.message)?;
    assert_eq!(result.status, AdmissionStatus::Rejected);
    assert_eq!(result.reason, "hidden-tool");
    Ok(())
}

#[test]
fn state_views_and_effect_keys_are_closed() -> Result<(), String> {
    assert_eq!(
        direct_tool_view_for_state("orient", None).tool_names(),
        ["list_directory", "read_file", "search_text", "write_record"]
    );
    assert_eq!(
        direct_tool_view_for_state("modify", None).tool_names(),
        ["create_file", "edit_file", "write_record"]
    );
    for state in ["review", "respond", "wait", "idle"] {
        assert!(direct_tool_view_for_state(state, None).entries.is_empty());
    }
    assert_eq!(
        direct_tool_view_for_state("recovery", Some("edit_file")).tool_names(),
        ["edit_file"]
    );
    let decision = direct_decision();
    let admission = admit_action(
        &decision,
        &action_params(
            "read_file",
            vec![("path", "README.md"), ("complete", "false")],
        ),
    )
    .map_err(|error| error.message)?;
    assert_eq!(
        dispatch_effect_key(&decision, &admission, &EffectKey("workspace.read".into())),
        Ok(EffectKey("workspace.read".into()))
    );
    assert_eq!(
        dispatch_effect_key(&decision, &admission, &EffectKey("workspace.edit".into())),
        Err("stale-effect-key")
    );
    Ok(())
}

fn direct_decision() -> RuntimeDecision {
    RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".into()),
        direct_tool_view(),
        OutputEnvelope::Action,
    )
}

fn action(tool: &str, field: &str, value: &str) -> ModelAction {
    action_params(tool, vec![(field, value)])
}

fn action_params(tool: &str, params: Vec<(&str, &str)>) -> ModelAction {
    ModelAction {
        tool: tool.into(),
        params: params
            .into_iter()
            .map(|(field, value)| (field.into(), value.into()))
            .collect::<BTreeMap<_, _>>(),
    }
}
