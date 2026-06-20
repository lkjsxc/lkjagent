mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn memory_prune_reports_semantic_merge_source_rows() -> TestResult<()> {
    let workspace = temp_workspace("memory-prune-merge")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    insert_memory(
        &mut conn,
        "Graph Policy Lesson",
        "graph,policy",
        "Record graph policy candidates once and skip duplicate maintenance lessons safely.",
    )?;
    let source = insert_memory(
        &mut conn,
        "graph policy lesson",
        "maintenance",
        "Record graph policy candidates once and skip duplicate maintenance lessons safely now.",
    )?;

    let output = dispatch(
        &action("memory.prune", &[]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("merged_rows=1"));
    assert!(output.content.contains(&format!("source_rows={source}")));
    Ok(())
}

fn insert_memory(
    conn: &mut rusqlite::Connection,
    title: &str,
    tags: &str,
    content: &str,
) -> TestResult<i64> {
    conn.execute(
        "INSERT INTO memory (kind, title, tags, content, tokens, created_at, updated_at)
         VALUES ('lesson', ?1, ?2, ?3, 10, '2026', '2026')",
        (title, tags, content),
    )?;
    let id = conn.last_insert_rowid();
    conn.execute(
        "INSERT INTO memory_fts (rowid, title, tags, content) VALUES (?1, ?2, ?3, ?4)",
        (id, title, tags, content),
    )?;
    Ok(id)
}
