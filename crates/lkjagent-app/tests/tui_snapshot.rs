use std::path::Path;

use lkjagent_app::tui_snapshot::load;
use lkjagent_app::workbench;
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
    assert!(!snapshot.transcript.contains("AI answered"));
    assert_order(&snapshot.transcript, &["owner: follow up", "agent: done"])
}

#[test]
fn transcript_hides_internal_step_duplicate_messages() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_duplicate_message_fixture(&conn)?;

    let snapshot = load(&conn, Path::new("data"))?;
    let lines = snapshot.transcript.lines().collect::<Vec<_>>();

    assert_eq!(lines, vec!["owner: hello", "agent: hello"]);
    Ok(())
}

#[test]
fn workbench_keeps_diagnostics_out_of_transcript_pane() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_duplicate_message_fixture(&conn)?;

    let text = workbench::render_once(&conn)?;
    let transcript = between(&text, "+-- transcript --+", "+-- right rail --+")?;

    assert!(transcript.contains("owner: hello"));
    assert!(transcript.contains("agent: hello"));
    assert!(!transcript.contains("stepdone"));
    assert!(!transcript.contains("taskclosed"));
    assert!(!transcript.contains("The user said hello"));
    assert!(text.contains("[status]"));
    Ok(())
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

fn insert_duplicate_message_fixture(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO queue (id, content, state, force_new, created_at, task_id)
         VALUES (1, 'hello', 'delivered', 0, '2026-07-09T06:20:43Z', 1)",
        [],
    )?;
    conn.execute(
        "INSERT INTO events (id, task_id, kind, content, created_at)
         VALUES (1, 1, 'stepdone', 'The user said hello.', '2026-07-09T06:20:50Z'),
                (2, 1, 'stepdone', 'hello', '2026-07-09T06:20:52Z'),
                (3, 1, 'taskclosed', 'hello', '2026-07-09T06:20:52Z')",
        [],
    )?;
    Ok(())
}

fn between<'a>(text: &'a str, start: &str, end: &str) -> TestResult<&'a str> {
    let start_at = text.find(start).ok_or("missing start")? + start.len();
    let rest = &text[start_at..];
    let end_at = rest.find(end).ok_or("missing end")?;
    Ok(&rest[..end_at])
}
