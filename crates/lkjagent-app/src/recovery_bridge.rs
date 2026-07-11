use lkjagent_core::engine::Command;
use lkjagent_core::model::{AttemptOutcome, TaskSnapshot};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_recovery::{
    bounded_diagnostic, normalized_signature, plan, strategy_condition, tuple_fingerprint,
    FailureClass, RecoveryPlan, RecoveryStrategy,
};
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::{params, Connection, OptionalExtension};

pub fn recover_or_reuse(
    conn: &Connection,
    unfinished: &[RuntimeDecision],
    _now: &str,
) -> Result<Option<RuntimeDecision>, String> {
    if unfinished.len() > 1 {
        return Err(format!(
            "multiple pending decisions ({}) make replay authority ambiguous; automatic replay blocked",
            unfinished.len()
        ));
    }
    for decision in unfinished {
        let evidence_count = external_evidence_count(conn, &decision.id)?;
        if evidence_count > 0 {
            return Err(format!(
                "pending decision {} has {evidence_count} durable evidence rows; automatic replay blocked",
                decision.id
            ));
        }
    }
    Ok(unfinished.first().cloned())
}

#[rustfmt::skip]
pub fn record_command_recovery_facts(conn: &Connection, snapshot: &TaskSnapshot,
    commands: &[Command], decision_id: &str, now: &str) -> Result<(), String> {
    let case_id = snapshot.task.id.to_string();
    for (index, command) in commands.iter().enumerate() {
        let failure = match command {
            Command::RecordAttempt(attempt) => match attempt.outcome {
                AttemptOutcome::ParseFault => Some(("parse", attempt.diagnosis.as_str())),
                AttemptOutcome::EndpointError => Some(("endpoint", attempt.diagnosis.as_str())),
                AttemptOutcome::EffectError => Some(("effect", attempt.diagnosis.as_str())),
                AttemptOutcome::Ok | AttemptOutcome::CheckFail => None,
            },
            Command::RecordChecks { results, .. } if results.iter().any(|item| !item.passed) => Some(("check", "check failed")),
            _ => None,
        };
        if let Some((kind, detail)) = failure {
            record_recovery_fact(conn, &case_id, decision_id, kind, detail, index, now)?;
        }
    }
    Ok(())
}

#[rustfmt::skip]
pub fn record_recovery_fact(conn: &Connection, case_id: &str, decision_id: &str,
    kind: &str, detail: &str, _index: usize, now: &str) -> Result<(), String> {
    let decision = decision(conn, decision_id)?;
    let class = FailureClass::from_fault(kind, detail);
    let class_text = serde_json::to_value(class).map_err(|error| error.to_string())?
        .as_str().ok_or_else(|| "failure class is not text".to_string())?.to_string();
    let signature = normalized_signature(detail).map_err(|error| error.message)?;
    let prior = prior_failure_count(conn, case_id, &class_text, &decision.operation.0, &signature)?;
    let prior = usize::try_from(prior).map_err(|error| error.to_string())?;
    let mut plan = plan(class, prior);
    if class == FailureClass::Endpoint && prior >= config_number(conn, "runtime.endpoint_retry_limit")? as usize {
        plan = RecoveryPlan { next_strategy: Some(RecoveryStrategy::WaitExternal),
            changed_condition: "wait for external endpoint condition".to_string(), remaining_budget: 0,
            exhausted: false, wait_external: true };
    }
    let eligible_at = endpoint_eligibility(conn, class, &plan, prior, now)?;
    let endpoint_condition = (class == FailureClass::Endpoint).then(|| config_text(conn, "runtime.endpoint_condition_fingerprint")).transpose()?;
    let prompt = prompt_fingerprint(conn, &decision)?;
    let tool_view = decision.tool_view_fingerprint().map_err(|error| error.message)?;
    let budget = stable_fingerprint(&(decision.model_budget_tokens, &decision.evidence_requirements))
        .map_err(|error| error.message)?;
    let tuple = tuple_fingerprint(&decision.operation.0, &prompt, &tool_view, &budget, &signature)
        .map_err(|error| error.message)?;
    let strategy = plan.next_strategy.map(strategy_text).transpose()?.unwrap_or_else(|| "owner-block".to_string());
    let attempted = serde_json::from_value::<RecoveryStrategy>(serde_json::Value::String(decision.recovery_policy.clone())).ok();
    let changed_condition = attempted.map(strategy_condition).unwrap_or("initial failure conditions");
    let operation_key = if plan.wait_external { "runtime.wait".to_string() }
        else if class == FailureClass::Check { format!("recovery.handle/check/{}", tuple.replace(':', "-")) }
        else { decision.operation.0.clone() };
    let key = if plan.exhausted { StateKey::new("completion", "blocked") } else {
        StateKey::new("recovery", format!("{class_text}/{}/{}", tuple.replace(':', "-"), prior + 1))
    }.map_err(|error| error.message)?;
    let event_id = next_event_id(conn, case_id, "recovery").map_err(|error| error.to_string())?;
    let mut cell = StateCell::active(key, event_id.clone());
    cell.payload_schema = if plan.exhausted { "completion.blocked" } else { "recovery.failure" }.to_string();
    cell.payload_json = serde_json::json!({
        "fault_class": class, "normalized_signature": signature, "decision_id": decision_id,
        "operation": decision.operation.0, "prompt_fingerprint": prompt,
        "state_vector_fingerprint": decision.state_vector_fingerprint,
        "context_fingerprint": decision.context_frame_fingerprint, "tool_view_fingerprint": tool_view,
        "budget_fingerprint": budget, "attempted_strategy": decision.recovery_policy,
        "changed_condition": changed_condition, "next_changed_condition": plan.changed_condition,
        "diagnostic": bounded_diagnostic(detail),
        "retry_count": prior + 1, "next_strategy": plan.next_strategy,
        "eligible_at": eligible_at, "remaining_budget": plan.remaining_budget,
        "tuple_fingerprint": tuple, "operation_key": if plan.exhausted { serde_json::Value::Null } else { serde_json::Value::String(operation_key) },
        "expected_envelope": format!("{:?}", decision.expected_envelope),
        "tool_view": decision.tool_view.entries, "model_budget_tokens": recovery_budget(decision.model_budget_tokens, plan.next_strategy),
        "recovery_policy": strategy, "evidence_requirements": decision.evidence_requirements,
        "owner_action": if plan.exhausted { "inspect preserved failure evidence and choose a changed condition" } else { "" },
        "timed_retry_remaining": eligible_at.is_some(), "endpoint_condition_fingerprint": endpoint_condition,
        "wake_condition": if plan.wait_external { "endpoint configuration fingerprint change" } else { "" },
    }).to_string();
    cell.evidence_refs = vec![EvidenceRef { source_type: "runtime_decision".to_string(),
        source_id: decision_id.to_string(), fingerprint: tuple }];
    cell.cooldown_until = eligible_at; cell.created_at = now.to_string(); cell.updated_at = now.to_string();
    let event = RuntimeEvent { id: event_id, case_id: case_id.to_string(), kind: cell.payload_schema.clone(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)), source: "recovery-bridge".to_string(),
        created_at: now.to_string(), decision_id: Some(decision_id.to_string()) };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn prior_failure_count(conn: &Connection, case_id: &str, class: &str, operation: &str,
    signature: &str) -> Result<i64, String> {
    conn.query_row("SELECT COUNT(*) FROM state_cells WHERE case_id = ?1
        AND payload_schema = 'recovery.failure' AND json_extract(payload_json, '$.fault_class') = ?2
        AND json_extract(payload_json, '$.operation') = ?3
        AND json_extract(payload_json, '$.normalized_signature') = ?4",
        params![case_id, class, operation, signature], |row| row.get(0)).map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn decision(conn: &Connection, id: &str) -> Result<RuntimeDecision, String> {
    let json: String = conn.query_row("SELECT decision_json FROM runtime_decisions WHERE id = ?1",
        [id], |row| row.get(0)).map_err(|error| error.to_string())?;
    serde_json::from_str(&json).map_err(|error| error.to_string())
}

fn prompt_fingerprint(conn: &Connection, decision: &RuntimeDecision) -> Result<String, String> {
    conn.query_row("SELECT prompt_fingerprint FROM prompt_frames WHERE decision_id = ?1 ORDER BY created_at DESC, id DESC LIMIT 1",
        [&decision.id], |row| row.get(0)).optional().map_err(|error| error.to_string())?
        .map_or_else(|| Ok(format!("native:{}", decision.context_frame_fingerprint)), Ok)
}

#[rustfmt::skip]
fn strategy_text(strategy: RecoveryStrategy) -> Result<String, String> {
    serde_json::to_value(strategy).map_err(|error| error.to_string())?.as_str()
        .map(str::to_string).ok_or_else(|| "recovery strategy is not text".to_string())
}

#[rustfmt::skip]
fn recovery_budget(current: Option<u32>, strategy: Option<RecoveryStrategy>) -> Option<u32> {
    if matches!(strategy, Some(RecoveryStrategy::ReduceUnit | RecoveryStrategy::NarrowOutput |
        RecoveryStrategy::ContinueBoundary | RecoveryStrategy::SplitSection |
        RecoveryStrategy::AlternateSampling | RecoveryStrategy::SmallerPrompt))
        { current.map(|value| (value / 2).max(128)) } else { current }
}

#[rustfmt::skip]
fn config_text(conn: &Connection, key: &str) -> Result<String, String> {
    conn.query_row("SELECT value FROM config WHERE key = ?1", [key], |row| row.get(0)).map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn config_number(conn: &Connection, key: &str) -> Result<u64, String> {
    config_text(conn, key)?.parse::<u64>().map_err(|error| error.to_string())
}

fn endpoint_eligibility(
    conn: &Connection,
    class: FailureClass,
    plan: &RecoveryPlan,
    prior: usize,
    now: &str,
) -> Result<Option<String>, String> {
    if class != FailureClass::Endpoint || plan.exhausted || plan.wait_external {
        return Ok(None);
    }
    let base = config_number(conn, "runtime.endpoint_backoff_milliseconds")?;
    let factor = 1_u64
        .checked_shl(u32::try_from(prior.min(16)).map_err(|error| error.to_string())?)
        .unwrap_or(u64::MAX);
    Ok(crate::clock::add_milliseconds(
        now,
        base.saturating_mul(factor),
    ))
}

#[rustfmt::skip]
fn external_evidence_count(conn: &Connection, decision_id: &str) -> Result<i64, String> {
    let sql = "SELECT
        (SELECT COUNT(*) FROM provider_exchanges WHERE decision_id = ?1) +
        (SELECT COUNT(*) FROM observations WHERE decision_id = ?1) +
        (SELECT COUNT(*) FROM tool_admissions WHERE decision_id = ?1)";
    conn.query_row(sql, [decision_id], |row| row.get(0)).map_err(|error| error.to_string())
}
