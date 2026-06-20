use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphStateTrackRow {
    pub track_id: String,
    pub label: String,
    pub posture: String,
    pub intensity: u8,
    pub confidence: u8,
    pub phase: String,
    pub active_node: String,
    pub evidence_gap: Vec<String>,
    pub next_affordances: Vec<String>,
    pub risk: Vec<String>,
    pub last_update_turn: Option<u64>,
    pub rank_score: u8,
}

pub fn replace_state_tracks(
    conn: &Connection,
    case_id: i64,
    rows: &[GraphStateTrackRow],
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "DELETE FROM graph_state_tracks WHERE case_id = ?1",
        params![case_id],
    )?;
    for row in rows {
        insert_state_track(conn, case_id, row, now)?;
    }
    Ok(())
}

pub fn state_tracks_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Vec<GraphStateTrackRow>> {
    let mut statement = conn.prepare(
        "SELECT track_id, label, posture, intensity, confidence, phase,
                active_node, evidence_gap, next_affordances, risk,
                last_update_turn, rank_score
         FROM graph_state_tracks
         WHERE case_id = ?1
         ORDER BY rank_score DESC, track_id",
    )?;
    let rows = statement.query_map(params![case_id], read_row)?;
    collect_rows(rows)
}

fn insert_state_track(
    conn: &Connection,
    case_id: i64,
    row: &GraphStateTrackRow,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_state_tracks
         (case_id, track_id, label, posture, intensity, confidence, phase,
          active_node, evidence_gap, next_affordances, risk, last_update_turn,
          rank_score, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            case_id,
            row.track_id,
            row.label,
            row.posture,
            i64::from(row.intensity),
            i64::from(row.confidence),
            row.phase,
            row.active_node,
            join(&row.evidence_gap),
            join(&row.next_affordances),
            join(&row.risk),
            row.last_update_turn.map(|turn| turn as i64),
            i64::from(row.rank_score),
            now
        ],
    )?;
    Ok(())
}

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<GraphStateTrackRow> {
    Ok(GraphStateTrackRow {
        track_id: row.get(0)?,
        label: row.get(1)?,
        posture: row.get(2)?,
        intensity: row.get::<_, i64>(3)?.clamp(0, 100) as u8,
        confidence: row.get::<_, i64>(4)?.clamp(0, 100) as u8,
        phase: row.get(5)?,
        active_node: row.get(6)?,
        evidence_gap: split(&row.get::<_, String>(7)?),
        next_affordances: split(&row.get::<_, String>(8)?),
        risk: split(&row.get::<_, String>(9)?),
        last_update_turn: row
            .get::<_, Option<i64>>(10)?
            .map(|turn| turn.max(0) as u64),
        rank_score: row.get::<_, i64>(11)?.clamp(0, 100) as u8,
    })
}

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
) -> StoreResult<Vec<T>> {
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn join(values: &[String]) -> String {
    values.join("\n")
}

fn split(value: &str) -> Vec<String> {
    value
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
