use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_effects::workspace_edit::ObservedTarget;
use lkjagent_store::error::{StoreError, StoreResult};
use rusqlite::{params, Connection, TransactionBehavior};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq, Eq)]
#[rustfmt::skip]
pub struct CheckReduction { pub scheduled: usize, pub passed: usize }

#[rustfmt::skip]
struct EffectFact {
    matter: String, decision: String, path: String,
    intended: Vec<u8>, mode: u32, revision: Vec<u8>, targets: Vec<String>,
}

#[rustfmt::skip]
pub fn reduce_committed_edit(connection: &mut Connection, workspace: &OpenedWorkspace,
    journal: &str, monotonic_ms: i64, wall_time: &str) -> StoreResult<CheckReduction> {
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let fact = effect_fact(&tx, journal)?;
    let observed = workspace.observe_edit_target(&fact.path)
        .map_err(|error| StoreError::InvalidState(format!("check read failed: {error:?}")))?;
    let (bytes, mode) = match observed {
        ObservedTarget::Present(value) => (value.bytes, value.mode),
        ObservedTarget::Absent => (Vec::new(), 0),
    };
    let source = Sha256::digest(&bytes).to_vec();
    let pending = pending(&tx, obligations(&tx, &fact.matter)?, &source)?;
    if pending.is_empty() { tx.commit()?; return Ok(CheckReduction { scheduled: 0, passed: 0 }); }
    let event_sequence: i64 = tx.query_row("SELECT coalesce(max(causal_sequence),0)+1 FROM runtime_events WHERE matter_id=?1",
        [&fact.matter], |row| row.get(0))?;
    let event = identity("check-event", journal, &source);
    tx.execute(
        "INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id)
         VALUES(?1,?2,?3,'checks-computed',?4,?5,?6,'harness',?7)",
        params![event, fact.matter, event_sequence, monotonic_ms, wall_time,
            json!({"journal":journal,"source_revision":hex(&source)}).to_string().as_bytes(), journal],
    )?;
    let mut passed = 0;
    for (obligation, kind, parameters) in &pending {
        let success = evaluate(kind, parameters, &fact, &bytes, mode, &source);
        let measured = json!({"path":fact.path,"sha256":hex(&source),"mode":mode,
            "effect_targets":fact.targets,"passed":success})
        .to_string();
        let check = identity("check", &format!("{journal}:{obligation}:{kind}"), &source);
        let evidence = Sha256::digest([parameters.as_slice(), measured.as_bytes()].concat());
        tx.execute(
            "UPDATE checks SET current=0 WHERE obligation_id=?1 AND current=1",
            [obligation],
        )?;
        tx.execute(
            "INSERT INTO checks(id,matter_id,obligation_id,decision_id,kind,parameters,current,passed,measured,evidence_fingerprint,source_revision,checked_event_id)
             VALUES(?1,?2,?3,?4,?5,?6,1,?7,?8,?9,?10,?11)",
            params![check, fact.matter, obligation, fact.decision, kind, parameters,
                i64::from(success), measured.as_bytes(), evidence.as_slice(), source, event],
        )?;
        tx.execute(
            "UPDATE obligations SET status=?1,current_check_id=?2,invalidating_event_id=NULL WHERE id=?3",
            params![if success { "passed" } else { "open" }, check, obligation],
        )?;
        passed += usize::from(success);
    }
    let state = if passed == pending.len() { "current-passed" } else { "failed" };
    tx.execute("UPDATE state_cells SET status='suppressed' WHERE matter_id=?1 AND namespace='check' AND cell_key IN ('current-passed','failed')", [&fact.matter])?;
    tx.execute("INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,'check',?2,?3,'active',?4,?5) ON CONFLICT(matter_id,namespace,cell_key) DO UPDATE SET payload=excluded.payload,status='active',source_event_id=excluded.source_event_id,fingerprint=excluded.fingerprint",
        params![fact.matter,state,json!({"journal":journal,"source_revision":hex(&source)}).to_string().as_bytes(),event,source])?;
    tx.commit()?;
    Ok(CheckReduction { scheduled: pending.len(), passed })
}

fn effect_fact(tx: &rusqlite::Transaction<'_>, journal: &str) -> StoreResult<EffectFact> {
    let base = tx
        .query_row(
            "SELECT d.matter_id,d.id,t.normalized_path,t.intended_bytes,t.intended_mode,r.sha256
         FROM effect_journal j JOIN runtime_decisions d ON d.id=j.decision_id
         JOIN observations o ON o.id=j.observation_id
         JOIN effect_targets t ON t.journal_id=j.id JOIN workspace_revisions r ON r.effect_id=j.id
         WHERE j.id=?1 AND j.status='settled' AND o.status='succeeded'
         AND t.ordinal=0 AND t.operation IN ('create','replace')",
            [journal],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
                    row.get::<_, Vec<u8>>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, Vec<u8>>(5)?,
                ))
            },
        )
        .map_err(StoreError::from)?;
    let path =
        String::from_utf8(base.2).map_err(|error| StoreError::InvalidState(error.to_string()))?;
    let targets = query_bytes(
        tx,
        "SELECT normalized_path FROM effect_targets WHERE journal_id=?1 ORDER BY ordinal",
        journal,
    )?;
    let targets = targets
        .into_iter()
        .map(String::from_utf8)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| StoreError::InvalidState(error.to_string()))?;
    Ok(EffectFact {
        matter: base.0,
        decision: base.1,
        path,
        intended: base.3,
        mode: u32::try_from(base.4).unwrap_or(0),
        revision: base.5,
        targets,
    })
}

fn obligations(
    tx: &rusqlite::Transaction<'_>,
    matter: &str,
) -> StoreResult<Vec<(String, String, Vec<u8>)>> {
    let mut query = tx.prepare("SELECT id,predicate_kind,predicate_payload FROM obligations
        WHERE matter_id=?1 AND required=1 AND predicate_kind IN ('workspace-byte','workspace-content','workspace-collateral','managed-journal') ORDER BY id")?;
    let rows = query.query_map([matter], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn query_bytes(
    tx: &rusqlite::Transaction<'_>,
    sql: &str,
    value: &str,
) -> StoreResult<Vec<Vec<u8>>> {
    let mut query = tx.prepare(sql)?;
    let rows = query.query_map([value], |row| row.get(0))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

#[rustfmt::skip]
fn pending(tx: &rusqlite::Transaction<'_>, rows: Vec<(String, String, Vec<u8>)>, source: &[u8])
    -> StoreResult<Vec<(String, String, Vec<u8>)>> {
    let mut pending = Vec::new();
    for row in rows {
        if !already_current(tx, &row.0, source, &row.1)? { pending.push(row); }
    }
    Ok(pending)
}

fn already_current(
    tx: &rusqlite::Transaction<'_>,
    obligation: &str,
    source: &[u8],
    kind: &str,
) -> StoreResult<bool> {
    Ok(tx.query_row("SELECT count(*) FROM checks WHERE obligation_id=?1 AND source_revision=?2 AND kind=?3 AND current=1",
        params![obligation, source, kind], |row| row.get::<_, i64>(0))? == 1)
}

#[rustfmt::skip]
fn evaluate(kind: &str, parameters: &[u8], fact: &EffectFact, bytes: &[u8], mode: u32,
    source: &[u8]) -> bool {
    let Ok(value) = serde_json::from_slice::<Value>(parameters) else { return false; };
    match kind {
        "workspace-byte" => value["path"] == fact.path && value["sha256"] == hex(source)
            && bytes == fact.intended && mode == fact.mode && source == fact.revision,
        "workspace-content" => value["path"] == fact.path
            && std::str::from_utf8(bytes).is_ok_and(|text|
                count(text, value["old"].as_str().unwrap_or(""))
                    == value["old_count"].as_u64().unwrap_or(u64::MAX) as usize
                && count(text, value["new"].as_str().unwrap_or(""))
                    == value["new_count"].as_u64().unwrap_or(u64::MAX) as usize),
        "workspace-collateral" => value["allowed_paths"].as_array().is_some_and(|allowed|
            fact.targets.iter().all(|path|
                allowed.iter().any(|item| item.as_str() == Some(path)))),
        "managed-journal" => crate::journal_checks::evaluate(parameters, &fact.path, bytes),
        _ => false,
    }
}

#[rustfmt::skip]
fn count(text: &str, needle: &str) -> usize {
    if needle.is_empty() { 0 } else { text.match_indices(needle).count() }
}
fn identity(prefix: &str, seed: &str, source: &[u8]) -> String {
    let digest = Sha256::digest([seed.as_bytes(), source].concat());
    format!("{prefix}-{}", &hex(&digest)[..24])
}
#[rustfmt::skip]
fn hex(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
