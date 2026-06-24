mod support;

use lkjagent_store::runtime_authority::{
    latest_decision_for_case, latest_observation_for_decision, latest_prompt_frame_for_decision,
    latest_snapshot_for_case, record_effect, record_prompt_frame, record_runtime_observation,
    record_tool_admission, PromptFrameInput, RuntimeEffectInput, RuntimeObservationInput,
    ToolAdmissionInput,
};
use lkjagent_store::schema::setup;
use rusqlite::Connection;
use support::runtime_kernel::{remove_temp_store, seed_decision, temp_store_path};
use support::{memory_store, TestResult};

#[test]
fn kernel_prompt_and_observation_rows_round_trip() -> TestResult<()> {
    let conn = memory_store()?;
    let seeded = seed_decision(&conn)?;
    let packages = vec!["pkg:artifact".to_string(), "pkg:recovery".to_string()];
    let frame_id = record_prompt_frame(
        &conn,
        &PromptFrameInput {
            decision_id: seeded.decision_id,
            case_scope: "case",
            case_id: Some(17),
            frame_kind: "model-call",
            prompt_fingerprint: "prompt-fp-1",
            context_package_ids: &packages,
            rendered_summary: "mission=owner_execution admitted=artifact.next",
            created_at: "2026-01-01T00:00:03Z",
        },
    )?;
    let admission_id = record_tool_admission(&conn, &admission_input(seeded.decision_id))?;
    let effect_id = record_effect(
        &conn,
        &RuntimeEffectInput {
            decision_id: seeded.decision_id,
            admission_id: Some(admission_id),
            effect_kind: "tool",
            effect_summary: "artifact.next selected weak path",
            observation_event_id: None,
            created_at: "2026-01-01T00:00:05Z",
        },
    )?;
    let observation_id = record_runtime_observation(
        &conn,
        &observation_input(seeded.decision_id, admission_id, effect_id, seeded.event_id),
    )?;

    let frame = latest_prompt_frame_for_decision(&conn, seeded.decision_id)?
        .ok_or("missing prompt frame")?;
    assert_eq!(frame.id, frame_id);
    assert_eq!(frame.context_package_ids, "pkg:artifact,pkg:recovery");
    let observation =
        latest_observation_for_decision(&conn, seeded.decision_id)?.ok_or("missing observation")?;
    assert_eq!(observation.id, observation_id);
    assert_eq!(observation.admission_id, Some(admission_id));
    assert_eq!(observation.effect_id, Some(effect_id));
    assert_eq!(observation.status, "ok");
    Ok(())
}

#[test]
fn latest_snapshot_and_decision_survive_store_reopen() -> TestResult<()> {
    let path = temp_store_path()?;
    {
        let conn = Connection::open(&path)?;
        setup(&conn)?;
        let seeded = seed_decision(&conn)?;
        assert!(seeded.snapshot_id > 0);
    }
    let conn = Connection::open(&path)?;
    setup(&conn)?;
    let snapshot = latest_snapshot_for_case(&conn, 17)?.ok_or("missing snapshot")?;
    let decision = latest_decision_for_case(&conn, 17)?.ok_or("missing decision")?;
    assert_eq!(snapshot.case_id, Some(17));
    assert_eq!(decision.snapshot_id, Some(snapshot.id));
    remove_temp_store(path)?;
    Ok(())
}

#[test]
fn admission_cannot_exist_without_decision_id() -> TestResult<()> {
    let conn = memory_store()?;
    let mut input = admission_input(9_999);
    input.refusal_reason = "missing decision";
    let result = record_tool_admission(&conn, &input);
    assert!(result.is_err());
    Ok(())
}

fn admission_input(decision_id: i64) -> ToolAdmissionInput<'static> {
    ToolAdmissionInput {
        decision_id,
        case_scope: "case",
        case_id: Some(17),
        requested_tool: "artifact.next",
        admitted: true,
        refusal_reason: "",
        exact_valid_example: None,
        created_at: "2026-01-01T00:00:04Z",
    }
}

fn observation_input(
    decision_id: i64,
    admission_id: i64,
    effect_id: i64,
    event_id: i64,
) -> RuntimeObservationInput<'static> {
    RuntimeObservationInput {
        decision_id,
        admission_id: Some(admission_id),
        effect_id: Some(effect_id),
        observation_event_id: Some(event_id),
        observation_kind: "tool",
        status: "ok",
        summary: "weak path selected",
        created_at: "2026-01-01T00:00:06Z",
    }
}
