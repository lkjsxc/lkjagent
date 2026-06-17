use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_protocol::render_owner;
use rusqlite::Connection;

use crate::error::RuntimeResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveredOwner {
    pub id: i64,
    pub content: String,
    pub tokens: usize,
}

pub fn owner_frame(content: &str, tokens: usize) -> Frame {
    Frame::new(FrameKind::Owner, render_owner(content), tokens)
}

pub fn deliver_next(
    conn: &mut Connection,
    turn: i64,
    tokens: i64,
    now: &str,
) -> RuntimeResult<Option<DeliveredOwner>> {
    let delivered = lkjagent_store::queue::deliver_next(conn, turn, tokens, now)?;
    Ok(delivered.map(|row| DeliveredOwner {
        id: row.id,
        content: row.content,
        tokens: tokens as usize,
    }))
}
