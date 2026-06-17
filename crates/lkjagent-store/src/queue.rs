mod row;

use rusqlite::{params, Connection};

pub use row::QueueRow;

use crate::error::StoreResult;
use crate::events::{append_event_tx, EventKind};
use row::{next_pending, pending_content, queue_content, rows_from_statement};

pub fn enqueue(conn: &mut Connection, content: &str, reason: &str, now: &str) -> StoreResult<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO queue (created_at, updated_at, source_queue_id, content, status, delivered_turn)
         VALUES (?1, ?1, NULL, ?2, 'pending', NULL)",
        params![now, content],
    )?;
    let id = tx.last_insert_rowid();
    append_event_tx(
        &tx,
        None,
        EventKind::QueueMutation,
        &mutation_content("enqueue", reason, id, None, None, Some(content)),
        0,
        now,
    )?;
    tx.commit()?;
    Ok(id)
}

pub fn edit(
    conn: &mut Connection,
    id: i64,
    content: &str,
    reason: &str,
    now: &str,
) -> StoreResult<()> {
    let tx = conn.transaction()?;
    let before = pending_content(&tx, id)?;
    tx.execute(
        "UPDATE queue SET content = ?1, updated_at = ?2 WHERE id = ?3 AND status = 'pending'",
        params![content, now, id],
    )?;
    append_event_tx(
        &tx,
        None,
        EventKind::QueueMutation,
        &mutation_content("edit", reason, id, None, Some(&before), Some(content)),
        0,
        now,
    )?;
    tx.commit()?;
    Ok(())
}

pub fn delete(conn: &mut Connection, id: i64, reason: &str, now: &str) -> StoreResult<()> {
    let tx = conn.transaction()?;
    let before = pending_content(&tx, id)?;
    tx.execute(
        "UPDATE queue SET status = 'deleted', updated_at = ?1 WHERE id = ?2 AND status = 'pending'",
        params![now, id],
    )?;
    append_event_tx(
        &tx,
        None,
        EventKind::QueueMutation,
        &mutation_content("delete", reason, id, None, Some(&before), None),
        0,
        now,
    )?;
    tx.commit()?;
    Ok(())
}

pub fn redeliver(
    conn: &mut Connection,
    source_id: i64,
    content: Option<&str>,
    reason: &str,
    now: &str,
) -> StoreResult<i64> {
    let tx = conn.transaction()?;
    let source = queue_content(&tx, source_id)?;
    let next_content = content.unwrap_or(source.as_str());
    tx.execute(
        "INSERT INTO queue (created_at, updated_at, source_queue_id, content, status, delivered_turn)
         VALUES (?1, ?1, ?2, ?3, 'pending', NULL)",
        params![now, source_id, next_content],
    )?;
    let id = tx.last_insert_rowid();
    append_event_tx(
        &tx,
        None,
        EventKind::QueueMutation,
        &mutation_content(
            "redeliver",
            reason,
            id,
            Some(source_id),
            Some(&source),
            Some(next_content),
        ),
        0,
        now,
    )?;
    tx.commit()?;
    Ok(id)
}

pub fn deliver_next(
    conn: &mut Connection,
    turn: i64,
    tokens: i64,
    now: &str,
) -> StoreResult<Option<QueueRow>> {
    let tx = conn.transaction()?;
    let row = next_pending(&tx)?;
    let Some(row) = row else {
        tx.commit()?;
        return Ok(None);
    };
    tx.execute(
        "UPDATE queue SET status = 'delivered', delivered_turn = ?1, updated_at = ?2
         WHERE id = ?3 AND status = 'pending'",
        params![turn, now, row.id],
    )?;
    append_event_tx(&tx, Some(turn), EventKind::Owner, &row.content, tokens, now)?;
    tx.commit()?;
    Ok(Some(QueueRow {
        updated_at: now.to_string(),
        status: "delivered".to_string(),
        delivered_turn: Some(turn),
        ..row
    }))
}

pub fn list(conn: &Connection) -> StoreResult<Vec<QueueRow>> {
    let mut statement = conn.prepare(
        "SELECT id, created_at, updated_at, source_queue_id, content, status, delivered_turn
         FROM queue ORDER BY id",
    )?;
    rows_from_statement(&mut statement, [])
}

fn mutation_content(
    operation: &str,
    reason: &str,
    target_id: i64,
    source_id: Option<i64>,
    before: Option<&str>,
    after: Option<&str>,
) -> String {
    format!(
        "operation={operation}\nreason={reason}\ntarget_id={target_id}\nsource_queue_id={}\nbefore={}\nafter={}",
        source_id.map_or_else(|| "null".to_string(), |id| id.to_string()),
        before.unwrap_or(""),
        after.unwrap_or("")
    )
}
