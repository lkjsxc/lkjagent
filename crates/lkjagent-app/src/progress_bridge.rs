use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_eligibility::{no_progress_strategy, ProgressVector};
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey, StateStatus};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::{params, Connection, OptionalExtension};

pub fn record(conn: &Connection, decision: &RuntimeDecision, now: &str) -> Result<(), String> {
    let mut vector = ProgressVector {
        obligations: rows(conn, "SELECT key_label || ':' || payload_schema FROM state_cells
            WHERE case_id = ?1 AND status = 'Active' AND namespace NOT IN ('recovery','progress','runtime')", &decision.case_id)?,
        passed_checks: rows(conn, "SELECT checks.name || ':' || COALESCE(checks.evidence_fingerprint,'') || ':' || checks.artifact_refs_json
            FROM check_results AS checks JOIN runtime_decisions AS decisions ON decisions.id = checks.decision_id
            WHERE decisions.case_id = ?1 AND checks.passed = 1 AND checks.id = (SELECT MAX(latest.id)
                FROM check_results AS latest JOIN runtime_decisions AS latest_decision ON latest_decision.id = latest.decision_id WHERE latest_decision.case_id = decisions.case_id AND latest.step_id = checks.step_id AND latest.name = checks.name AND latest.params_json = checks.params_json)", &decision.case_id)?,
        artifacts: rows(conn, "SELECT kind || ':' || path || ':' || fingerprint FROM artifacts
            WHERE case_id = ?1 AND id = (SELECT id FROM artifacts AS latest WHERE latest.case_id = artifacts.case_id
                AND latest.kind = artifacts.kind AND latest.path = artifacts.path ORDER BY latest.created_at DESC, latest.id DESC LIMIT 1)", &decision.case_id)?,
        source_evidence: fingerprints(rows(conn, "SELECT json_array(effect_name, content, artifact_refs_json)
            FROM observations WHERE case_id = ?1 AND status = 'ok' AND contamination_class = 'Clean'", &decision.case_id)?)?,
        wake_conditions: rows(conn, "SELECT key_label || ':' || cooldown_until FROM state_cells
            WHERE case_id = ?1 AND status = 'Active' AND cooldown_until IS NOT NULL UNION
            SELECT 'edge:' || id || ':' || relation || ':' || from_ref_id || ':' || to_ref_id
            FROM state_edges WHERE case_id = ?1 AND status = 'Active'", &decision.case_id)?,
        recovery_strategy: decision.recovery_policy.clone(),
    };
    vector.canonicalize();
    let fingerprint = vector.fingerprint().map_err(|error| error.message)?;
    release_suspend(conn, decision, &fingerprint, now)?;
    append_progress(conn, decision, &vector, &fingerprint, now)?;
    let window = config_number(conn, "runtime.no_progress_window")?;
    if repeated(conn, &decision.case_id, &fingerprint, window)? {
        append_no_progress(conn, decision, &fingerprint, window, now)?;
    }
    Ok(())
}

#[rustfmt::skip]
fn release_suspend(conn: &Connection, decision: &RuntimeDecision,
    fingerprint: &str, now: &str) -> Result<(), String> {
    if !suspend_wake_operation(&decision.operation.0) { return Ok(()); }
    let label: Option<String> = conn.query_row("SELECT key_label FROM state_cells
        WHERE case_id = ?1 AND status = 'Active' AND payload_schema = 'recovery.no-progress'
        AND json_extract(payload_json, '$.next_strategy') = 'suspend'
        AND json_extract(payload_json, '$.progress_fingerprint') <> ?2 LIMIT 1",
        params![decision.case_id, fingerprint], |row| row.get(0)).optional()
        .map_err(|error| error.to_string())?;
    let Some(label) = label else { return Ok(()); };
    let key = StateKey::from_label(&label).map_err(|error| error.message)?;
    let event_id = next_event_id(conn, &decision.case_id, "progress-wake").map_err(|error| error.to_string())?;
    let event = RuntimeEvent { id: event_id, case_id: decision.case_id.clone(), kind: "state.cell.suppress".to_string(),
        payload: RuntimeEventPayload::SuppressCell { key, reason: "progress vector changed".to_string() },
        source: "progress-bridge".to_string(), created_at: now.to_string(), decision_id: Some(decision.id.clone()) };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn suspend_wake_operation(key: &str) -> bool {
    ["owner.intake", "owner.answer", "recovery.handle/", "effect.run/", "todo.review/", "calendar.review/", "routine.run/", "index.rebuild/", "proof.collect/", "dev.review/", "project.advance/", "finance.review/"].iter().any(|prefix| key.starts_with(prefix))
}

#[rustfmt::skip]
fn fingerprints(values: Vec<String>) -> Result<Vec<String>, String> {
    values.into_iter().map(|value| stable_fingerprint(&value).map_err(|error| error.message)).collect()
}

#[rustfmt::skip]
fn rows(conn: &Connection, sql: &str, case_id: &str) -> Result<Vec<String>, String> {
    let mut statement = conn.prepare(sql).map_err(|error| error.to_string())?;
    let mapped = statement.query_map([case_id], |row| row.get(0)).map_err(|error| error.to_string())?;
    mapped.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn append_progress(conn: &Connection, decision: &RuntimeDecision, vector: &ProgressVector,
    fingerprint: &str, now: &str) -> Result<(), String> {
    let event_id = next_event_id(conn, &decision.case_id, "progress").map_err(|error| error.to_string())?;
    let key = StateKey::new("progress", &decision.id).map_err(|error| error.message)?;
    let mut cell = StateCell::active(key, event_id.clone()); cell.status = StateStatus::Resolved;
    cell.payload_schema = "runtime.progress".to_string();
    cell.payload_json = serde_json::json!({ "fingerprint": fingerprint, "vector": vector }).to_string();
    cell.evidence_refs = vec![EvidenceRef { source_type: "runtime_decision".to_string(),
        source_id: decision.id.clone(), fingerprint: fingerprint.to_string() }];
    cell.created_at = now.to_string(); cell.updated_at = now.to_string();
    append(conn, decision, cell, event_id, now)
}

fn repeated(
    conn: &Connection,
    case_id: &str,
    fingerprint: &str,
    window: u64,
) -> Result<bool, String> {
    let limit = i64::try_from(window).map_err(|error| error.to_string())?;
    let mut statement = conn
        .prepare(
            "SELECT json_extract(payload_json, '$.fingerprint') FROM state_cells
        WHERE case_id = ?1 AND payload_schema = 'runtime.progress' ORDER BY rowid DESC LIMIT ?2",
        )
        .map_err(|error| error.to_string())?;
    let values = statement
        .query_map(params![case_id, limit], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(
        values.len() == usize::try_from(window).map_err(|error| error.to_string())?
            && values.iter().all(|value| value == fingerprint),
    )
}

fn append_no_progress(
    conn: &Connection,
    decision: &RuntimeDecision,
    fingerprint: &str,
    window: u64,
    now: &str,
) -> Result<(), String> {
    let prior: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM state_cells WHERE case_id = ?1
        AND payload_schema = 'recovery.no-progress'",
            [&decision.case_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let prior = usize::try_from(prior).map_err(|error| error.to_string())?;
    let strategy = no_progress_strategy(prior);
    let operation = if strategy == Some("suspend") {
        "runtime.wait"
    } else if decision.operation.0.starts_with("check.run/") {
        "recovery.handle/no-progress"
    } else {
        &decision.operation.0
    };
    let event_id =
        next_event_id(conn, &decision.case_id, "no-progress").map_err(|error| error.to_string())?;
    let key = if strategy.is_some() {
        StateKey::new("recovery", format!("no-progress/{}", decision.id))
    } else {
        StateKey::new("completion", "blocked")
    }
    .map_err(|error| error.message)?;
    let mut cell = StateCell::active(key, event_id.clone());
    cell.payload_schema = if strategy.is_some() {
        "recovery.no-progress"
    } else {
        "completion.blocked"
    }
    .to_string();
    cell.payload_json = serde_json::json!({
        "progress_fingerprint": fingerprint, "window": window, "repetition": prior + 1,
        "next_strategy": strategy, "operation_key": if strategy.is_some() { Some(operation) } else { None },
        "expected_envelope": format!("{:?}", decision.expected_envelope),
        "tool_view": decision.tool_view.entries, "model_budget_tokens": decision.model_budget_tokens,
        "recovery_policy": strategy.unwrap_or("owner-block"), "selector_tier": if strategy == Some("suspend") { 45 } else { 80 },
        "evidence_requirements": decision.evidence_requirements,
        "wake_condition": if strategy == Some("suspend") { "owner input or changed evidence" } else { "" },
        "owner_action": if strategy.is_none() { "change the goal, evidence, or operation strategy" } else { "" },
    }).to_string();
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "progress_vector".to_string(),
        source_id: decision.id.clone(),
        fingerprint: fingerprint.to_string(),
    }];
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    append(conn, decision, cell, event_id, now)
}

fn append(
    conn: &Connection,
    decision: &RuntimeDecision,
    cell: StateCell,
    event_id: String,
    now: &str,
) -> Result<(), String> {
    let event = RuntimeEvent {
        id: event_id,
        case_id: decision.case_id.clone(),
        kind: cell.payload_schema.clone(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "progress-bridge".to_string(),
        created_at: now.to_string(),
        decision_id: Some(decision.id.clone()),
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

fn config_number(conn: &Connection, key: &str) -> Result<u64, String> {
    let value: String = conn
        .query_row("SELECT value FROM config WHERE key = ?1", [key], |row| {
            row.get(0)
        })
        .map_err(|error| error.to_string())?;
    value
        .parse()
        .map_err(|error: std::num::ParseIntError| error.to_string())
}
