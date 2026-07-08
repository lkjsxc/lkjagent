use std::path::Path;

use lkjagent_app::tui_snapshot::load;
use lkjagent_store::plan_access::enqueue_with_force;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn snapshot_reads_durable_queue_proof_and_evidence_rows() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    enqueue_with_force(&conn, "hello", false, "001")?;
    enqueue_with_force(&conn, "follow up", false, "003")?;
    conn.execute(
        "INSERT INTO events (task_id, kind, content, created_at)
         VALUES (1, 'stepdone', 'AI answered', '002'),
                (1, 'taskclosed', 'done', '004')",
        [],
    )?;
    conn.execute_batch(
        "INSERT INTO tool_admissions
         (id, case_id, decision_id, tool_view_fingerprint, action_tool, status,
          parsed_action_json, result_json, created_at)
         VALUES ('adm-1', '1', 'dec-1', 'fp-tools', 'fs.read', 'Rejected', '{}', '{}', '005');
         INSERT INTO state_edges
         (id, scope, case_id, from_ref_kind, from_ref_id, to_ref_kind, to_ref_id,
          relation, reason, evidence_json, edge_json, status, created_at,
          source_event_id, suppression_reason)
         VALUES ('edge-1', 'case:1', '1', 'check', 'c1', 'artifact', 'a1',
          'verifies', 'stale', '[]', '{}', 'Suppressed', '006', 'event-1', 'stale');",
    )?;

    let snapshot = load(&conn, Path::new("data"))?;

    assert!(snapshot.status.contains("queue: 2 pending"));
    assert!(snapshot.queue.contains("queue 1"));
    assert!(snapshot.proof.contains("prompt_frames=0"));
    assert!(snapshot.workspace.contains("workspace: root="));
    assert!(snapshot.status.contains("refused=1 stale_edges=1"));
    assert!(snapshot.tools.contains("rejected=1 stale_edges=1"));
    assert_order(
        &snapshot.transcript,
        &["owner: hello", "agent: AI answered"],
    )?;
    assert_order(&snapshot.transcript, &["owner: follow up", "agent: done"])
}

fn assert_order(text: &str, expected: &[&str]) -> TestResult<()> {
    let mut cursor = 0;
    for needle in expected {
        let Some(offset) = text[cursor..].find(needle) else {
            return Err(format!("missing {needle} after byte {cursor} in {text}").into());
        };
        cursor += offset + needle.len();
    }
    Ok(())
}
