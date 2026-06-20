mod support;

use lkjagent_store::memory::{find, prune_exact_duplicates};
use support::{memory_store, TestResult};

#[test]
fn prune_merges_same_title_high_overlap_rows() -> TestResult<()> {
    let mut conn = memory_store()?;
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

    let report = prune_exact_duplicates(&mut conn)?;
    let found = find(&conn, "graph policy maintenance", 5)?;

    assert_eq!(report.merged, 1);
    assert_eq!(report.deleted, 1);
    assert_eq!(report.source_rows, vec![source]);
    assert_eq!(found.len(), 1);
    assert!(found[0].content.contains(&format!("source_row={source}")));
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
