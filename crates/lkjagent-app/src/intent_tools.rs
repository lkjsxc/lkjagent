use lkjagent_core::runtime_decision::{RuntimeDecision, ToolExampleParam};
use lkjagent_core::runtime_operation::RuntimePhase;
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateStatus};

pub(crate) fn narrow(
    decision: &mut RuntimeDecision,
    objective: &[u8],
    phase: RuntimePhase,
    snapshot: &RuntimeSnapshot,
) {
    let text = String::from_utf8_lossy(objective).to_ascii_lowercase();
    let record = text.contains("write_record")
        || text.starts_with("remember ")
        || text.starts_with("correct ");
    if !record {
        return;
    }
    let needs_read = text.contains("read ");
    if phase == RuntimePhase::Modify || !needs_read {
        decision
            .tool_view
            .entries
            .retain(|entry| entry.name == "write_record");
    }
    let pending = snapshot.cells.iter().find(|(key, cell)| {
        key.namespace == "report" && key.name == "pending" && cell.status == StateStatus::Active
    });
    let Some((_, cell)) = pending else { return };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&cell.payload_json) else {
        return;
    };
    let (Some(slug), Some(unit)) = (
        value["slug"].as_str(),
        value["remaining_units"]
            .as_array()
            .and_then(|units| units.first())
            .and_then(|unit| unit.as_str()),
    ) else {
        return;
    };
    let reducing = snapshot.cells.iter().any(|(key, cell)| {
        key.namespace == "recovery"
            && key.name == "output-limit"
            && cell.status == StateStatus::Active
    });
    if reducing {
        decision.model_budget_tokens = Some(4_096);
    }
    let Some(entry) = decision
        .tool_view
        .entries
        .iter_mut()
        .find(|entry| entry.name == "write_record")
    else {
        return;
    };
    let names = ["family", "title", "body", "slug", "unit"];
    entry
        .field_specs
        .retain(|field| names.contains(&field.name.as_str()));
    for field in &mut entry.field_specs {
        field.required = true;
    }
    entry.required_params = names.iter().map(|name| (*name).into()).collect();
    entry.optional_params.clear();
    let length = if reducing {
        "between 190 and 220 words"
    } else {
        "at least 190 words"
    };
    entry.purpose = format!("write only report child slug={slug} unit={unit}; author a distinct source-linked body {length}; do not rewrite the map");
    entry.example_params = [
        ("family", "report".to_string()),
        ("title", format!("World History: {unit}")),
        (
            "body",
            format!("A distinct source-linked section about {unit}."),
        ),
        ("slug", slug.to_string()),
        ("unit", unit.to_string()),
    ]
    .into_iter()
    .map(|(name, value)| ToolExampleParam {
        name: name.into(),
        value,
    })
    .collect();
}

#[cfg(test)]
mod tests {
    use super::narrow;
    use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
    use lkjagent_core::runtime_operation::RuntimePhase;
    use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};
    use lkjagent_core::runtime_tool_catalog::direct_tool_view_for_state;

    #[test]
    fn explicit_record_intent_keeps_read_then_only_record() {
        let snapshot = RuntimeSnapshot::empty("m");
        let mut decision = RuntimeDecision::new(
            "d",
            "m",
            OperationKey("orient.matter".into()),
            direct_tool_view_for_state("orient", None),
            OutputEnvelope::Action,
        );
        narrow(
            &mut decision,
            b"Read source.md then use write_record family report",
            RuntimePhase::Orient,
            &snapshot,
        );
        assert!(decision.tool_view.entry("read_file").is_some());
        assert!(decision.tool_view.entry("write_record").is_some());
        narrow(
            &mut decision,
            b"Read source.md then use write_record family report",
            RuntimePhase::Modify,
            &snapshot,
        );
        assert_eq!(decision.tool_view.tool_names(), ["write_record"]);

        let mut memory = RuntimeDecision::new(
            "d2",
            "m",
            OperationKey("orient.matter".into()),
            direct_tool_view_for_state("orient", None),
            OutputEnvelope::Action,
        );
        narrow(
            &mut memory,
            b"Remember this fact",
            RuntimePhase::Orient,
            &snapshot,
        );
        assert_eq!(memory.tool_view.tool_names(), ["write_record"]);
    }

    #[test]
    fn pending_report_descriptor_names_exact_child() -> Result<(), String> {
        let key = StateKey::new("report", "pending").map_err(|error| error.message)?;
        let mut cell = StateCell::active(key.clone(), "event");
        cell.payload_json = r#"{"slug":"world-history","remaining_units":["origins"]}"#.into();
        let mut snapshot = RuntimeSnapshot::empty("m");
        snapshot.cells.insert(key, cell);
        let recovery = StateKey::new("recovery", "output-limit").map_err(|error| error.message)?;
        snapshot
            .cells
            .insert(recovery.clone(), StateCell::active(recovery, "fault"));
        let mut decision = RuntimeDecision::new(
            "d",
            "m",
            OperationKey("modify.report".into()),
            direct_tool_view_for_state("modify", None),
            OutputEnvelope::Action,
        );
        narrow(
            &mut decision,
            b"use write_record family report",
            RuntimePhase::Modify,
            &snapshot,
        );
        let entry = decision.tool_view.entry("write_record").ok_or("record")?;
        assert_eq!(
            entry.required_params,
            ["family", "title", "body", "slug", "unit"]
        );
        assert!(entry.purpose.contains("unit=origins"));
        assert!(entry.purpose.contains("between 190 and 220 words"));
        assert_eq!(decision.model_budget_tokens, Some(4_096));
        assert!(entry.field_spec("children").is_none());
        Ok(())
    }
}
