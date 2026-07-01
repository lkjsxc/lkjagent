mod support;

use lkjagent_tools::dispatch::{dispatch, AuthorityAdmissionView};
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn runtime_write_contract_refuses_other_scene_path() -> TestResult<()> {
    let workspace = temp_workspace("runtime-contract-path")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    seed_prompt_frame(&conn)?;
    let mut state = state();
    state.authority_view = Some(authority_view());

    let output = dispatch(
        &action(
            "fs.batch_write",
            &[(
                "files",
                "path: stories/aurora-ledger/manuscript/scenes/chapter-01/scene-02.md\ncontent:\nprose",
            )],
        ),
        &runtime,
        &mut conn,
        &mut state,
    )
    .content;

    assert!(
        output.contains("outside runtime write contract"),
        "{output}"
    );
    Ok(())
}

#[test]
fn runtime_write_contract_refuses_byte_overflow() -> TestResult<()> {
    let workspace = temp_workspace("runtime-contract-bytes")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    seed_prompt_frame(&conn)?;
    let mut state = state();
    state.authority_view = Some(authority_view());

    let output = dispatch(
        &action(
            "fs.batch_write",
            &[(
                "files",
                "path: stories/aurora-ledger/manuscript/scenes/chapter-01/scene-01.md\ncontent:\nabcdefghijklmnopqrstu",
            )],
        ),
        &runtime,
        &mut conn,
        &mut state,
    )
    .content;

    assert!(output.contains("max_file_bytes"), "{output}");
    Ok(())
}

fn authority_view() -> AuthorityAdmissionView {
    AuthorityAdmissionView {
        decision_id: "7".to_string(),
        case_id: "1".to_string(),
        authority_fingerprint: "fp".to_string(),
        active_mission: "owner_recovery".to_string(),
        active_node: "recover-repeat".to_string(),
        admitted_tools: vec!["fs.batch_write".to_string()],
        blocked_tools: Vec::new(),
        shell_allowed: false,
        completion_allowed: false,
        missing_evidence: vec!["artifact-readiness".to_string()],
        recovery_route: None,
        exact_valid_example: "none".to_string(),
    }
}

fn seed_prompt_frame(conn: &rusqlite::Connection) -> TestResult<()> {
    conn.execute(
        "INSERT INTO runtime_snapshots
         (id, case_scope, case_id, queue_pending_count, owner_objective, active_mode,
          active_node, missing_evidence, maintenance_state, staleness_fingerprint, created_at)
         VALUES (1, 'case', 1, 0, 'Aurora Ledger', 'recovery', 'recover-repeat',
          'artifact-readiness', 'none', 'stale', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO runtime_authority_events
         (id, snapshot_id, case_scope, case_id, event_kind, event_payload, created_at)
         VALUES (1, 1, 'case', 1, 'case_resumed', '', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO runtime_authority_decisions
         (id, snapshot_id, case_scope, case_id, event_id, mission, active_mode, active_node,
          admitted_tools, blocked_tools, missing_evidence, forced_next_action,
          completion_allowed, compaction_required, maintenance_allowed, authority_fingerprint,
          staleness_fingerprint, created_at)
         VALUES (7, 1, 'case', 1, 1, 'owner_recovery', 'recovery', 'recover-repeat',
          'fs.batch_write', '', 'artifact-readiness', 'fs.batch_write', 0, 0, 0,
          'fp', 'stale', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO runtime_prompt_frames
         (decision_id, case_scope, case_id, frame_kind, prompt_fingerprint,
          context_package_ids, rendered_summary, created_at)
         VALUES (7, 'case', 1, 'runtime', 'pf', '', ?1, 'now')",
        [write_contract()],
    )?;
    Ok(())
}

fn write_contract() -> String {
    "<write-contract>\n<tool>fs.batch_write</tool>\n<paths>\n- stories/aurora-ledger/manuscript/scenes/chapter-01/scene-01.md\n</paths>\n<limits>max_files=1 max_file_bytes=20 max_batch_bytes=20</limits>\n</write-contract>".to_string()
}
