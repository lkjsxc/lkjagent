use std::path::Path;

use rusqlite::Connection;

use super::db_support::{compact, warn_if_empty};
use super::model::{
    CaseRow, ContractRow, CountRow, DecisionRow, EventRow, ProofBundle, QueueRow, ReadinessRow,
};

pub fn load_store(bundle: &mut ProofBundle, data_dir: &Path) {
    let store = data_dir.join("lkjagent.sqlite3");
    bundle.store_path = store.display().to_string();
    bundle.store_present = store.exists();
    if !bundle.store_present {
        bundle.warnings.push("store file missing".to_string());
        return;
    }
    let conn = match Connection::open(&store) {
        Ok(conn) => conn,
        Err(error) => {
            bundle.warnings.push(format!("open store failed: {error}"));
            return;
        }
    };
    load_rows(bundle, &conn);
}

fn load_rows(bundle: &mut ProofBundle, conn: &Connection) {
    bundle.cases = query_cases(conn, &mut bundle.warnings);
    bundle.queue_counts = query_queue_counts(conn, &mut bundle.warnings);
    bundle.queue_recent = query_queue_recent(conn, &mut bundle.warnings);
    bundle.readiness = query_readiness(conn, &mut bundle.warnings);
    bundle.active_contracts = query_contracts(conn, &mut bundle.warnings);
    bundle.decisions = query_decisions(conn, &mut bundle.warnings);
    bundle.transcript = query_events(conn, &mut bundle.warnings);
    warn_if_empty(bundle);
}

fn query_cases(conn: &Connection, warnings: &mut Vec<String>) -> Vec<CaseRow> {
    let sql = "SELECT id, family, phase, active_node, status, next_action_class,
        length(objective) FROM graph_cases ORDER BY id DESC LIMIT 8";
    query(conn, sql, warnings, "graph cases", |row| {
        Ok(CaseRow {
            id: row.get(0)?,
            family: row.get(1)?,
            phase: row.get(2)?,
            node: row.get(3)?,
            status: row.get(4)?,
            next_action: row.get(5)?,
            objective_chars: row.get(6)?,
        })
    })
}
fn query_queue_counts(conn: &Connection, warnings: &mut Vec<String>) -> Vec<CountRow> {
    query(
        conn,
        "SELECT status, COUNT(*) FROM queue GROUP BY status ORDER BY status",
        warnings,
        "queue counts",
        |row| {
            Ok(CountRow {
                name: row.get(0)?,
                count: row.get(1)?,
            })
        },
    )
}
fn query_queue_recent(conn: &Connection, warnings: &mut Vec<String>) -> Vec<QueueRow> {
    let sql = "SELECT id, status, delivered_turn, length(content), created_at
        FROM queue ORDER BY id DESC LIMIT 8";
    query(conn, sql, warnings, "queue recent", |row| {
        let delivered: Option<i64> = row.get(2)?;
        Ok(QueueRow {
            id: row.get(0)?,
            status: row.get(1)?,
            delivered_turn: delivered.map_or_else(|| "none".to_string(), |value| value.to_string()),
            content_chars: row.get(3)?,
            created_at: row.get(4)?,
        })
    })
}
fn query_readiness(conn: &Connection, warnings: &mut Vec<String>) -> Vec<ReadinessRow> {
    let sql = "SELECT p.id, p.root, p.profile, r.status, r.atom_total, r.atom_ready,
        r.atom_missing, r.measured_total, r.accepted_floor, r.active_contract_id,
        r.next_path, r.completion_blockers
        FROM artifact_readiness r JOIN artifact_plans p ON p.id = r.plan_id
        ORDER BY r.updated_at DESC, p.id DESC LIMIT 8";
    query(conn, sql, warnings, "artifact readiness", |row| {
        let total: i64 = row.get(4)?;
        let ready: i64 = row.get(5)?;
        let missing: i64 = row.get(6)?;
        Ok(ReadinessRow {
            plan_id: row.get(0)?,
            root: row.get(1)?,
            profile: row.get(2)?,
            status: row.get(3)?,
            atoms: format!("{ready}/{total} missing={missing}"),
            measured: row.get(7)?,
            floor: row.get(8)?,
            active_contract: row.get(9)?,
            next_path: row.get(10)?,
            blockers: compact(row.get::<_, String>(11)?),
        })
    })
}
fn query_contracts(conn: &Connection, warnings: &mut Vec<String>) -> Vec<ContractRow> {
    let sql = "SELECT c.contract_id, p.root, c.status, c.max_files, c.max_file_bytes,
        c.max_batch_bytes, c.exact_paths FROM artifact_write_contracts c
        JOIN artifact_plans p ON p.id = c.plan_id
        WHERE c.status = 'active' ORDER BY c.id DESC LIMIT 8";
    query(conn, sql, warnings, "active contracts", |row| {
        let max_files: i64 = row.get(3)?;
        let max_file_bytes: i64 = row.get(4)?;
        let max_batch_bytes: i64 = row.get(5)?;
        Ok(ContractRow {
            id: row.get(0)?,
            root: row.get(1)?,
            status: row.get(2)?,
            limits: format!(
                "files={max_files} file_bytes={max_file_bytes} batch_bytes={max_batch_bytes}"
            ),
            paths: compact(row.get::<_, String>(6)?),
        })
    })
}
fn query_decisions(conn: &Connection, warnings: &mut Vec<String>) -> Vec<DecisionRow> {
    let sql = "SELECT id, mission, active_mode, active_node, forced_next_action,
        completion_allowed, created_at FROM runtime_authority_decisions
        ORDER BY id DESC LIMIT 5";
    query(conn, sql, warnings, "runtime decisions", |row| {
        let allowed: i64 = row.get(5)?;
        Ok(DecisionRow {
            id: row.get(0)?,
            mission: row.get(1)?,
            mode: row.get(2)?,
            node: row.get(3)?,
            next_action: row.get(4)?,
            completion: if allowed == 1 { "allowed" } else { "refused" }.to_string(),
            created_at: row.get(6)?,
        })
    })
}
fn query_events(conn: &Connection, warnings: &mut Vec<String>) -> Vec<EventRow> {
    let sql = "SELECT id, turn, kind, tokens, created_at FROM events ORDER BY id DESC LIMIT 12";
    query(conn, sql, warnings, "transcript events", |row| {
        let turn: Option<i64> = row.get(1)?;
        Ok(EventRow {
            id: row.get(0)?,
            turn: turn.map_or_else(|| "none".to_string(), |value| value.to_string()),
            kind: row.get(2)?,
            tokens: row.get(3)?,
            created_at: row.get(4)?,
        })
    })
}
fn query<T, F>(
    conn: &Connection,
    sql: &str,
    warnings: &mut Vec<String>,
    label: &str,
    mut map: F,
) -> Vec<T>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
{
    let mut statement = match conn.prepare(sql) {
        Ok(statement) => statement,
        Err(error) => {
            warnings.push(format!("{label} query failed: {error}"));
            return Vec::new();
        }
    };
    let rows = match statement.query_map([], |row| map(row)) {
        Ok(rows) => rows,
        Err(error) => {
            warnings.push(format!("{label} read failed: {error}"));
            return Vec::new();
        }
    };
    rows.filter_map(|row| {
        row.map_err(|error| warnings.push(format!("{label} row failed: {error}")))
            .ok()
    })
    .collect()
}
