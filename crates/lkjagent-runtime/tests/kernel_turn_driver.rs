use lkjagent_runtime::kernel::{RuntimeEvent, SnapshotAdapterInput};
use lkjagent_runtime::kernel_driver::{run_kernel_turn, KernelTurnInput, KernelTurnStage};
use lkjagent_store::runtime_authority::{latest_complete_chain_for_case, latest_decision_for_case};
use lkjagent_store::schema::setup;
use rusqlite::Connection;

#[test]
fn kernel_driver_records_decision_before_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let input = KernelTurnInput::new(owner_snapshot(), RuntimeEvent::OwnerMessageReceived);
    let record = run_kernel_turn(&conn, input).map_err(format_error)?;

    assert_eq!(record.stage, KernelTurnStage::PromptFrame);
    assert!(record.prompt_frame_id.is_some());
    let chain = latest_complete_chain_for_case(&conn, 17)?.ok_or("missing chain")?;
    assert_eq!(chain.snapshot_id, Some(record.snapshot_id));
    assert_eq!(chain.event_id, record.event_id);
    assert_eq!(chain.decision_id, record.decision_id);
    assert_eq!(chain.prompt_frame_id, record.prompt_frame_id);
    let decision = latest_decision_for_case(&conn, 17)?.ok_or("missing decision")?;
    assert_eq!(decision.id, record.decision_id);
    assert_eq!(decision.mission, "owner_execution");
    Ok(())
}

#[test]
fn kernel_driver_records_decision_before_runtime_effect() -> Result<(), Box<dyn std::error::Error>>
{
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let input = KernelTurnInput::new(
        SnapshotAdapterInput::default(),
        RuntimeEvent::MaintenanceTick,
    );
    let record = run_kernel_turn(&conn, input).map_err(format_error)?;

    assert_eq!(record.stage, KernelTurnStage::RuntimeEffect);
    let effect_id = record.effect_id.ok_or("missing effect")?;
    let decision_id: i64 = conn.query_row(
        "SELECT decision_id FROM runtime_effects WHERE id = ?1",
        [effect_id],
        |row| row.get(0),
    )?;
    assert_eq!(decision_id, record.decision_id);
    assert!(record.prompt_frame_id.is_none());
    Ok(())
}

fn owner_snapshot() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 44,
        case_id: Some("17".to_string()),
        graph_node: Some("document".to_string()),
        graph_phase: Some("execution".to_string()),
        owner_objective: Some("create long novel".to_string()),
        queue_head: Some("1".to_string()),
        pending_owner_count: 1,
        missing_evidence: vec!["artifact-readiness".to_string()],
        artifact_root: Some("stories/long-novel-with-detailed-settings".to_string()),
        ..SnapshotAdapterInput::default()
    }
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
