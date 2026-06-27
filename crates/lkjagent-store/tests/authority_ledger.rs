mod support;

use lkjagent_store::runtime_authority::{
    latest_admission_for_decision, latest_complete_chain_for_case, latest_decision_detail_for_case,
    latest_observation_for_decision, latest_prompt_frame_for_case, record_decision_detail,
    record_effect, record_prompt_frame, record_runtime_observation, record_snapshot_detail,
    record_tool_admission, DecisionDetailInput, PromptFrameInput, RuntimeEffectInput,
    RuntimeObservationInput, SnapshotDetailInput, ToolAdmissionInput,
};
use lkjagent_store::schema::setup;
use rusqlite::Connection;
use support::runtime_kernel::{remove_temp_store, seed_decision, temp_store_path};
use support::{memory_store, TestResult};

#[test]
fn latest_complete_chain_survives_store_reopen() -> TestResult<()> {
    let path = temp_store_path()?;
    let ids = {
        let conn = Connection::open(&path)?;
        setup(&conn)?;
        let seeded = seed_decision(&conn)?;
        record_snapshot_detail(&conn, &snapshot_detail(seeded.snapshot_id))?;
        record_decision_detail(&conn, &decision_detail(seeded.decision_id))?;
        let frame_id = record_prompt_frame(&conn, &prompt_frame(seeded.decision_id))?;
        let admission_id = record_tool_admission(&conn, &admission(seeded.decision_id))?;
        let effect_id = record_effect(&conn, &effect(seeded.decision_id, admission_id))?;
        let observation_id = record_runtime_observation(
            &conn,
            &observation(seeded.decision_id, admission_id, effect_id, seeded.event_id),
        )?;
        (
            seeded.snapshot_id,
            seeded.event_id,
            seeded.decision_id,
            frame_id,
            admission_id,
            observation_id,
        )
    };

    let conn = Connection::open(&path)?;
    setup(&conn)?;
    let chain = latest_complete_chain_for_case(&conn, 17)?.ok_or("missing chain")?;
    assert_eq!(chain.snapshot_id, Some(ids.0));
    assert_eq!(chain.event_id, ids.1);
    assert_eq!(chain.decision_id, ids.2);
    assert_eq!(chain.prompt_frame_id, Some(ids.3));
    assert_eq!(chain.admission_id, Some(ids.4));
    assert_eq!(chain.observation_id, Some(ids.5));
    let detail = latest_decision_detail_for_case(&conn, 17)?.ok_or("missing detail")?;
    assert_eq!(detail.decision_kind, "model_call");
    assert_eq!(detail.graph_phase, "execution");
    assert_eq!(detail.exact_next_action_class, "artifact.next");
    assert_eq!(detail.artifact_root.as_deref(), Some("stories/long-novel"));
    let frame = latest_prompt_frame_for_case(&conn, 17)?.ok_or("missing frame")?;
    assert_eq!(frame.id, ids.3);
    let admission_id = latest_admission_for_decision(&conn, ids.2)?.ok_or("missing admission")?;
    assert_eq!(admission_id, ids.4);
    let observed = latest_observation_for_decision(&conn, ids.2)?.ok_or("missing observation")?;
    assert_eq!(observed.id, ids.5);
    remove_temp_store(path)?;
    Ok(())
}

#[test]
fn orphan_authority_children_are_refused() -> TestResult<()> {
    let conn = memory_store()?;
    assert!(record_prompt_frame(&conn, &prompt_frame(9_999)).is_err());
    assert!(record_tool_admission(&conn, &admission(9_999)).is_err());
    assert!(record_effect(&conn, &effect(9_999, 9_998)).is_err());
    assert!(record_runtime_observation(&conn, &observation(9_999, 9_998, 9_997, 9_996)).is_err());

    let seeded = seed_decision(&conn)?;
    assert!(record_effect(&conn, &effect(seeded.decision_id, 9_998)).is_err());
    assert!(record_runtime_observation(
        &conn,
        &observation(seeded.decision_id, 9_998, 9_997, seeded.event_id),
    )
    .is_err());
    Ok(())
}

fn snapshot_detail(snapshot_id: i64) -> SnapshotDetailInput<'static> {
    SnapshotDetailInput {
        snapshot_id,
        graph_phase: "execution",
        artifact_root: Some("stories/long-novel"),
        weak_cursor: Some(3),
        latest_observation: Some("weak path selected"),
        prompt_frame_head: Some("frame-1"),
        authority_fingerprint: "authority-fp-1",
    }
}

fn decision_detail(decision_id: i64) -> DecisionDetailInput<'static> {
    DecisionDetailInput {
        decision_id,
        decision_kind: "model_call",
        graph_phase: "execution",
        exact_next_action_class: "artifact.next",
        runtime_effect_kind: None,
        artifact_root: Some("stories/long-novel"),
        weak_cursor: Some(3),
        latest_observation: Some("weak path selected"),
        prompt_frame_head: Some("frame-1"),
        authority_fingerprint: "authority-fp-1",
        staleness_fingerprint: "stale-fp-1",
    }
}

fn prompt_frame(decision_id: i64) -> PromptFrameInput<'static> {
    PromptFrameInput {
        decision_id,
        case_scope: "case",
        case_id: Some(17),
        frame_kind: "model-call",
        prompt_fingerprint: "prompt-fp-1",
        context_package_ids: &[],
        rendered_summary: "mission=owner_execution",
        created_at: "2026-01-01T00:00:03Z",
    }
}

fn admission(decision_id: i64) -> ToolAdmissionInput<'static> {
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

fn effect(decision_id: i64, admission_id: i64) -> RuntimeEffectInput<'static> {
    RuntimeEffectInput {
        decision_id,
        admission_id: Some(admission_id),
        effect_kind: "tool",
        effect_summary: "artifact.next selected weak path",
        observation_event_id: None,
        created_at: "2026-01-01T00:00:05Z",
    }
}

fn observation(
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
