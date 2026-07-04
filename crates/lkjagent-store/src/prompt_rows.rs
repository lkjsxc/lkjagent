use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_decision::RuntimeDecision;
use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptFrameRow {
    pub id: String,
    pub case_id: String,
    pub decision_id: String,
    pub prompt_fingerprint: String,
    pub context_frame_fingerprint: String,
    pub tool_view_fingerprint: String,
    pub body_ref: String,
    pub created_at: String,
}

pub fn insert_prompt_frame(
    conn: &Connection,
    id: &str,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    body_ref: &str,
    created_at: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO prompt_frames
         (id, case_id, decision_id, prompt_fingerprint,
          context_frame_fingerprint, tool_view_fingerprint, body_ref, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            id,
            decision.case_id,
            decision.id,
            prompt.fingerprint,
            decision.context_frame_fingerprint,
            decision.tool_view_fingerprint().unwrap_or_default(),
            body_ref,
            created_at,
        ],
    )?;
    Ok(())
}

pub fn prompt_frames(conn: &Connection, case_id: &str) -> StoreResult<Vec<PromptFrameRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, decision_id, prompt_fingerprint,
         context_frame_fingerprint, tool_view_fingerprint, body_ref, created_at
         FROM prompt_frames WHERE case_id = ?1 ORDER BY created_at, id",
    )?;
    let rows = statement.query_map([case_id], |row| {
        Ok(PromptFrameRow {
            id: row.get(0)?,
            case_id: row.get(1)?,
            decision_id: row.get(2)?,
            prompt_fingerprint: row.get(3)?,
            context_frame_fingerprint: row.get(4)?,
            tool_view_fingerprint: row.get(5)?,
            body_ref: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?;
    let mut frames = Vec::new();
    for row in rows {
        frames.push(row?);
    }
    Ok(frames)
}
