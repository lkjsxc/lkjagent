use crate::error::{StoreError, StoreResult};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use rusqlite::{params, Connection, Transaction, TransactionBehavior};

pub const MAX_CONVERSATION_ROWS: usize = 100;
pub const MAX_ACTIVITY_ROWS: usize = 200;
pub const MAX_CONVERSATION_BODY_BYTES: usize = 16_384;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConversationRow {
    pub id: String, pub sequence: i64, pub role: String, pub body: Vec<u8>,
    pub body_truncated: bool, pub lifecycle: String, pub matter_id: String,
    pub replacement_id: Option<String>,
}

#[rustfmt::skip]
#[derive(Clone, PartialEq, Eq)]
pub struct ActivityCursor { monotonic_ms: i64, kind_rank: i64, raw_id: String }

#[rustfmt::skip]
#[derive(Clone, PartialEq, Eq)]
pub struct ActivityRow {
    pub id: String, pub kind: String, pub matter_id: String, pub status: String,
    pub monotonic_ms: i64, cursor: ActivityCursor,
}

impl ActivityRow {
    pub fn cursor(&self) -> ActivityCursor {
        self.cursor.clone()
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StatusCounts {
    pub open_matters: i64, pub blocked_matters: i64, pub closed_matters: i64,
    pub unfinished_decisions: i64, pub unfinished_exchanges: i64, pub unfinished_effects: i64,
    pub rejected_admissions: i64, pub failed_observations: i64,
    pub current_checks: i64, pub passing_checks: i64, pub active_cells: i64,
}

#[rustfmt::skip]
#[derive(Clone, PartialEq, Eq)]
pub struct SnapshotPage {
    pub conversation_before: Option<i64>, pub conversation_limit: usize,
    pub activity_before: Option<ActivityCursor>, pub activity_limit: usize,
}

#[rustfmt::skip]
#[derive(Clone, PartialEq, Eq)]
pub struct TuiSnapshot {
    pub conversation: Vec<ConversationRow>, pub activity: Vec<ActivityRow>, pub status: StatusCounts,
}

pub fn snapshot(connection: &mut Connection, page: &SnapshotPage) -> StoreResult<TuiSnapshot> {
    snapshot_with(connection, page, || Ok(()))
}

pub(crate) fn snapshot_with(
    connection: &mut Connection,
    page: &SnapshotPage,
    between_queries: impl FnOnce() -> StoreResult<()>,
) -> StoreResult<TuiSnapshot> {
    let conversation_limit = bounded(page.conversation_limit, MAX_CONVERSATION_ROWS)?;
    let activity_limit = bounded(page.activity_limit, MAX_ACTIVITY_ROWS)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Deferred)?;
    let conversation = conversation(&transaction, page.conversation_before, conversation_limit)?;
    between_queries()?;
    let activity = activity(&transaction, page.activity_before.as_ref(), activity_limit)?;
    let status = status(&transaction)?;
    transaction.commit()?;
    Ok(TuiSnapshot {
        conversation,
        activity,
        status,
    })
}

fn bounded(value: usize, maximum: usize) -> StoreResult<i64> {
    if value == 0 {
        Err(StoreError::InvalidState(
            "TUI page limit must be positive".into(),
        ))
    } else {
        Ok(value.min(maximum) as i64)
    }
}

fn conversation(
    tx: &Transaction<'_>,
    before: Option<i64>,
    limit: i64,
) -> StoreResult<Vec<ConversationRow>> {
    let mut query = tx.prepare(
        "SELECT id,sequence,role,substr(body,1,?3),lifecycle,matter_id,replacement_id
         FROM conversation_messages WHERE (?1 IS NULL OR sequence<?1)
         ORDER BY sequence DESC,id DESC LIMIT ?2",
    )?;
    let body_limit = (MAX_CONVERSATION_BODY_BYTES + 1) as i64;
    let rows = query.query_map(params![before, limit, body_limit], |row| {
        let mut body: Vec<u8> = row.get(3)?;
        let body_truncated = body.len() > MAX_CONVERSATION_BODY_BYTES;
        body.truncate(MAX_CONVERSATION_BODY_BYTES);
        Ok(ConversationRow {
            id: row.get(0)?,
            sequence: row.get(1)?,
            role: row.get(2)?,
            body,
            body_truncated,
            lifecycle: row.get(4)?,
            matter_id: row.get(5)?,
            replacement_id: row.get(6)?,
        })
    })?;
    let mut result = rows.collect::<Result<Vec<_>, _>>()?;
    result.reverse();
    Ok(result)
}

fn activity(
    tx: &Transaction<'_>,
    before: Option<&ActivityCursor>,
    limit: i64,
) -> StoreResult<Vec<ActivityRow>> {
    let (at, rank, id) = before.map_or((None, None, None), |cursor| {
        (
            Some(cursor.monotonic_ms),
            Some(cursor.kind_rank),
            Some(cursor.raw_id.as_str()),
        )
    });
    let mut query = tx.prepare(ACTIVITY_SQL)?;
    let rows = query.query_map(params![at, rank, id, limit], |row| {
        let raw_id: String = row.get(0)?;
        let kind_rank = row.get(1)?;
        let kind: String = row.get(2)?;
        let id = if kind == "state-cell" {
            stable_fingerprint(&(kind.as_str(), raw_id.as_str()))
                .map(|value| format!("state-cell/{value}"))
                .map_err(|error| {
                    rusqlite::Error::ToSqlConversionFailure(
                        std::io::Error::other(error.message).into(),
                    )
                })?
        } else {
            format!("{kind}/{raw_id}")
        };
        let monotonic_ms = row.get(5)?;
        Ok(ActivityRow {
            id,
            kind,
            matter_id: row.get(3)?,
            status: row.get(4)?,
            monotonic_ms,
            cursor: ActivityCursor {
                monotonic_ms,
                kind_rank,
                raw_id,
            },
        })
    })?;
    let mut result = rows.collect::<Result<Vec<_>, _>>()?;
    result.reverse();
    Ok(result)
}

#[rustfmt::skip]
fn status(tx: &Transaction<'_>) -> StoreResult<StatusCounts> {
    Ok(tx.query_row(STATUS_SQL, [], |row| Ok(StatusCounts {
        open_matters: row.get(0)?, blocked_matters: row.get(1)?, closed_matters: row.get(2)?,
        unfinished_decisions: row.get(3)?, unfinished_exchanges: row.get(4)?, unfinished_effects: row.get(5)?,
        rejected_admissions: row.get(6)?, failed_observations: row.get(7)?, current_checks: row.get(8)?,
        passing_checks: row.get(9)?, active_cells: row.get(10)?,
    }))?)
}

const ACTIVITY_SQL: &str = "WITH activity(raw_id,kind_rank,kind,matter_id,status,monotonic_ms) AS (
 SELECT d.id,0,'decision',d.matter_id,d.status,d.selected_monotonic_ms FROM runtime_decisions d UNION ALL
 SELECT p.id,1,'provider-exchange',d.matter_id,p.status,p.started_monotonic_ms FROM provider_exchanges p JOIN runtime_decisions d ON d.id=p.decision_id UNION ALL
 SELECT a.id,2,'tool-admission',d.matter_id,a.status,d.selected_monotonic_ms FROM tool_admissions a JOIN runtime_decisions d ON d.id=a.decision_id UNION ALL
 SELECT j.id,3,'effect',d.matter_id,j.status,d.selected_monotonic_ms FROM effect_journal j JOIN runtime_decisions d ON d.id=j.decision_id UNION ALL
 SELECT o.id,4,'observation',d.matter_id,o.status,e.monotonic_ms FROM observations o JOIN runtime_decisions d ON d.id=o.decision_id JOIN runtime_events e ON e.id=o.event_id UNION ALL
 SELECT c.id,5,'check',c.matter_id,CASE WHEN c.current=0 THEN 'superseded' WHEN c.passed=1 THEN 'passed' ELSE 'failed' END,e.monotonic_ms FROM checks c JOIN runtime_events e ON e.id=c.checked_event_id UNION ALL
 SELECT lower(hex(CAST(s.matter_id AS BLOB)))||'/'||lower(hex(s.namespace))||'/'||lower(hex(s.cell_key)),6,'state-cell',s.matter_id,s.status,e.monotonic_ms FROM state_cells s JOIN runtime_events e ON e.id=s.source_event_id)
 SELECT raw_id,kind_rank,kind,matter_id,status,monotonic_ms FROM activity
 WHERE (?1 IS NULL OR monotonic_ms<?1 OR (monotonic_ms=?1 AND (kind_rank<?2 OR (kind_rank=?2 AND raw_id<?3))))
 ORDER BY monotonic_ms DESC,kind_rank DESC,raw_id DESC LIMIT ?4";
const STATUS_SQL: &str = "SELECT
 (SELECT count(*) FROM matters WHERE lifecycle='open'),(SELECT count(*) FROM matters WHERE lifecycle='blocked'),
 (SELECT count(*) FROM matters WHERE lifecycle='closed'),(SELECT count(*) FROM runtime_decisions WHERE status NOT IN ('settled','failed')),
 (SELECT count(*) FROM provider_exchanges WHERE status NOT IN ('succeeded','failed')),(SELECT count(*) FROM effect_journal WHERE status NOT IN ('settled','failed','compensated')),
 (SELECT count(*) FROM tool_admissions WHERE status='rejected'),(SELECT count(*) FROM observations WHERE status='failed'),
 (SELECT count(*) FROM checks WHERE current=1),(SELECT count(*) FROM checks WHERE current=1 AND passed=1),
 (SELECT count(*) FROM state_cells WHERE status='active')";

#[cfg(test)]
#[path = "tui_snapshot_tests.rs"]
mod tests;
