mod support;

use lkjagent_cli::run_cli;
use lkjagent_store::graph::snapshots::record_compaction_snapshot;
use lkjagent_store::token_usage::{record, TokenUsageEvent};
use support::{open_store, temp_data, TestResult};

#[test]
fn status_prints_ranked_active_state_tracks() -> TestResult<()> {
    let data = temp_data("status-tracks")?;
    let conn = open_store(&data)?;
    lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Create structured documentation for lkjagent.",
        "2026-06-20T00:00:00Z",
    )?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status.stdout.contains("active_states=1."));
    assert!(status.stdout.contains("document-structure"));
    assert!(status.stdout.contains("phase=planning"));
    assert!(status.stdout.contains("model_log="));
    Ok(())
}

#[test]
fn status_prints_compact_context_and_token_usage() -> TestResult<()> {
    let data = temp_data("status-accounting")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "context used tokens", "1234")?;
    record(
        &conn,
        &TokenUsageEvent {
            task_id: None,
            turn: 1,
            input_tokens: Some(8_120),
            output_tokens: Some(1_040),
            cached_input_tokens: Some(6_880),
            total_tokens: Some(9_160),
            context_window: Some(24_576),
            context_used_estimate: Some(1_234),
            source: "endpoint".to_string(),
        },
        "2026-06-20T00:00:00Z",
    )?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status.stdout.contains("ctx=1.23K/24.58K 5.02%"));
    assert!(status
        .stdout
        .contains("in=8.12K out=1.04K cache=6.88K total=9.16K"));
    Ok(())
}

#[test]
fn status_prints_unknown_token_usage_as_unknown() -> TestResult<()> {
    let data = temp_data("status-unknown-usage")?;
    open_store(&data)?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status
        .stdout
        .contains("in=unknown out=unknown cache=unknown total=unknown"));
    Ok(())
}

#[test]
fn status_prints_authority_snapshot_fields() -> TestResult<()> {
    let data = temp_data("status-authority")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "authority active mode", "Recovery")?;
    lkjagent_store::state::set(&conn, "authority phase", "recovery")?;
    lkjagent_store::state::set(&conn, "authority node", "recover-repeat")?;
    lkjagent_store::state::set(&conn, "authority evidence gaps", "artifact-readiness")?;
    lkjagent_store::state::set(
        &conn,
        "authority next action",
        "<action><tool>artifact.next</tool></action>",
    )?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status.stdout.contains("active_mode=Recovery"));
    assert!(status.stdout.contains("authority_phase=recovery"));
    assert!(status.stdout.contains("authority_node=recover-repeat"));
    assert!(status.stdout.contains("evidence_gaps=artifact-readiness"));
    assert!(status
        .stdout
        .contains("next_executable_action=<action><tool>artifact.next</tool></action>"));
    Ok(())
}

#[test]
fn status_prints_latest_compaction_snapshot_fields() -> TestResult<()> {
    let data = temp_data("status-compaction-snapshot")?;
    let conn = open_store(&data)?;
    let case = lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Create structured documentation.",
        "2026-06-20T00:00:00Z",
    )?;
    record_compaction_snapshot(
        &conn,
        case.case_id.ok_or("missing case id")?,
        "recovery",
        "recover-by-artifact-plan",
        "Create structured documentation.",
        &[
            "stage=post".to_string(),
            "write_batch_cursor=docs/a.md".to_string(),
        ],
        "2026-06-20T00:00:01Z",
    )?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status
        .stdout
        .contains("last_compaction=snapshot:2026-06-20T00:00:01Z"));
    assert!(status.stdout.contains("phase=recovery"));
    assert!(status.stdout.contains("node=recover-by-artifact-plan"));
    assert!(status.stdout.contains("write_batch_cursor=docs/a.md"));
    Ok(())
}

#[test]
fn status_prints_continuation_checkpoint_state() -> TestResult<()> {
    let data = temp_data("status-continuation")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "continuation epoch", "2")?;
    lkjagent_store::state::set(&conn, "continuation turns used", "5")?;
    lkjagent_store::state::set(&conn, "checkpoint turns", "8")?;
    lkjagent_store::state::set(&conn, "continuation decision", "continue-owner-execution")?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status.stdout.contains("continuation_epoch=2"));
    assert!(status.stdout.contains("continuation_turns=5"));
    assert!(status.stdout.contains("checkpoint_turns=8"));
    assert!(status
        .stdout
        .contains("continuation_decision=continue-owner-execution"));
    Ok(())
}
