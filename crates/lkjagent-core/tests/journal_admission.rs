use std::collections::BTreeMap;

use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_catalog::{direct_tool_view, direct_tool_view_for_state};

#[test]
fn record_descriptor_and_admission_are_exact() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "matter-1",
        OperationKey("orient.matter".into()),
        direct_tool_view(),
        OutputEnvelope::Action,
    );
    let entry = decision
        .tool_view
        .entry("write_record")
        .ok_or("missing record tool")?;
    assert_eq!(entry.required_params, ["family", "title", "body"]);
    assert_eq!(
        entry.optional_params,
        ["slug", "unit", "children", "minimum_words"]
    );
    assert_eq!(entry.effect_key.0, "workspace.record");
    for family in ["journal", "memory", "report"] {
        let admitted = admit_action(&decision, &action(family)).map_err(|error| error.message)?;
        assert_eq!(admitted.status, AdmissionStatus::Admitted);
    }
    let map = admit_action(&decision, &map_action()).map_err(|error| error.message)?;
    assert_eq!(map.status, AdmissionStatus::Admitted);
    let denied = admit_action(&decision, &action("calendar")).map_err(|error| error.message)?;
    assert_eq!(
        (denied.status, denied.reason.as_str()),
        (AdmissionStatus::Rejected, "record family is not admitted")
    );
    for state in ["review", "respond"] {
        assert!(direct_tool_view_for_state(state, None)
            .entry("write_record")
            .is_none());
    }
    Ok(())
}

fn action(family: &str) -> ModelAction {
    ModelAction {
        tool: "write_record".into(),
        params: [
            ("family".into(), family.into()),
            ("title".into(), "Known day".into()),
            ("body".into(), "Owner-grounded reflection".into()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>(),
    }
}

fn map_action() -> ModelAction {
    ModelAction {
        tool: "write_record".into(),
        params: [
            ("family".into(), "report".into()),
            ("title".into(), "Known report".into()),
            ("body".into(), "Owner-grounded reflection".into()),
            ("slug".into(), "known-report".into()),
            ("unit".into(), "index".into()),
            ("children".into(), "summary,risks".into()),
            ("minimum_words".into(), "20".into()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>(),
    }
}
