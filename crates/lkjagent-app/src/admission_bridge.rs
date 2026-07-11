use std::{collections::BTreeSet, fs, path::Path};

use lkjagent_core::engine::Command;
use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction, ToolAdmission};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{
    insert_admission_and_prepare, insert_tool_admission, EffectPreparation, PreparedEffect,
};
use rusqlite::{params, Connection};

type Admission = (ToolAdmission, String, Option<ModelAction>);
type AdmissionResult = Result<Option<Admission>, String>;
type Fingerprints = Result<(String, String), String>;

#[rustfmt::skip]
pub fn persist_tool_admissions(conn: &Connection, workspace: &Path, decision: &RuntimeDecision,
    commands: &[Command], now: &str) -> Result<Vec<PreparedEffect>, String> {
    if !conn.is_autocommit() { return persist_admissions(conn, workspace, decision, commands, now); }
    conn.execute_batch("BEGIN IMMEDIATE").map_err(|error| error.to_string())?;
    let result = persist_admissions(conn, workspace, decision, commands, now);
    let commit = result.is_ok() || matches!(result.as_ref(), Err(error) if error.starts_with("admission rejected:"));
    let sql = if commit { "COMMIT" } else { "ROLLBACK" };
    conn.execute_batch(sql).map_err(|error| error.to_string())?;
    result
}

#[rustfmt::skip]
fn persist_admissions(conn: &Connection, workspace: &Path, decision: &RuntimeDecision,
    commands: &[Command], now: &str) -> Result<Vec<PreparedEffect>, String> {
    crate::artifact_effects::validate_bundle_commands(commands)?;
    let base = existing_admission_count(conn, &decision.id)?;
    let mut seen = BTreeSet::new();
    let mut staged = Vec::new();
    for command in commands {
        let Some((mut admission, parsed, model_action)) = admission_for_command(decision, command)?
        else {
            continue;
        };
        if model_action.is_some()
            && admission.status == AdmissionStatus::Admitted
            && (repeated_admitted_action(conn, &decision.id, &parsed)?
                || !seen.insert(parsed.clone()))
        {
            admission = repeated_admission(&admission);
        }
        let ordinal = base + i64::try_from(staged.len() + 1).map_err(|error| error.to_string())?;
        if admission.status == AdmissionStatus::Rejected {
            let id = format!("{}-admission-{ordinal:04}", decision.id);
            insert_tool_admission(conn, &id, &decision.case_id, &admission, &parsed, now)
                .map_err(|error| format!("admission persistence failed: {error}"))?;
            return Err(format!("admission rejected: {}", admission.reason));
        }
        let plan = effect_plan(conn, workspace, &decision.case_id, command, &parsed, now)?;
        staged.push((admission, parsed, plan));
    }
    let mut prepared = Vec::new();
    for (index, (admission, parsed, plan)) in staged.into_iter().enumerate() {
        let ordinal = base + i64::try_from(index + 1).map_err(|error| error.to_string())?;
        let id = format!("{}-admission-{ordinal:04}", decision.id);
        let journal_id = format!("{id}-effect");
        let idempotency_key = format!("{}:{ordinal}", decision.id);
        let preparation = EffectPreparation {
            id: &id, case_id: &decision.case_id, admission: &admission,
            parsed_action_json: &parsed, journal_id: &journal_id,
            idempotency_key: &idempotency_key, command_ordinal: ordinal,
            target_path: plan.target_path.as_deref(), prior_fingerprint: &plan.prior_fingerprint,
            intended_fingerprint: &plan.intended_fingerprint, targets: &plan.targets, created_at: now,
        };
        prepared.push(
            insert_admission_and_prepare(conn, &preparation)
                .map_err(|error| format!("admission persistence failed: {error}"))?,
        );
    }
    Ok(prepared)
}

fn existing_admission_count(conn: &Connection, decision_id: &str) -> Result<i64, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM tool_admissions WHERE decision_id = ?1",
        [decision_id],
        |row| row.get(0),
    )
    .map_err(|error| format!("admission persistence failed: {error}"))
}

fn repeated_admitted_action(
    conn: &Connection,
    decision_id: &str,
    parsed: &str,
) -> Result<bool, String> {
    let found: i64 = conn
        .query_row(
            "SELECT EXISTS(
             SELECT 1 FROM tool_admissions
             WHERE decision_id = ?1 AND status = 'Admitted' AND parsed_action_json = ?2)",
            params![decision_id, parsed],
            |row| row.get(0),
        )
        .map_err(|error| format!("admission persistence failed: {error}"))?;
    Ok(found != 0)
}

fn repeated_admission(admission: &ToolAdmission) -> ToolAdmission {
    let mut rejected = admission.clone();
    rejected.status = AdmissionStatus::Rejected;
    rejected.reason = "repeated tool call; state the next different tool call".to_string();
    rejected
}

fn admission_for_command(decision: &RuntimeDecision, command: &Command) -> AdmissionResult {
    match command {
        Command::RunExplore(action) => {
            let action = crate::explore::model_action(action);
            let admission = admit_action(decision, &action).map_err(|error| error.message)?;
            let parsed = serde_json::to_string(&action).map_err(|error| error.to_string())?;
            Ok(Some((admission, parsed, Some(action))))
        }
        Command::WriteFile { .. } => harness_admission(decision, command, "native.write_file"),
        Command::AppendFile { .. } => harness_admission(decision, command, "native.append_file"),
        Command::RecordAttempt(_)
        | Command::RecordEvent(_)
        | Command::RecordMemory { .. }
        | Command::RecordChecks { .. }
        | Command::AddSteps(_) => Ok(None),
    }
}

fn harness_admission(decision: &RuntimeDecision, command: &Command, tool: &str) -> AdmissionResult {
    let admission = ToolAdmission {
        decision_id: decision.id.clone(),
        tool_view_fingerprint: decision
            .tool_view_fingerprint()
            .map_err(|error| error.message)?,
        action_tool: tool.to_string(),
        status: AdmissionStatus::Admitted,
        reason: "harness admitted".to_string(),
    };
    let parsed =
        serde_json::to_string(&format!("{command:?}")).map_err(|error| error.to_string())?;
    Ok(Some((admission, parsed, None)))
}

fn effect_plan(
    conn: &Connection,
    workspace: &Path,
    case_id: &str,
    command: &Command,
    parsed: &str,
    now: &str,
) -> Result<crate::artifact_plan::WritePlan, String> {
    match command {
        Command::WriteFile { path, content } => {
            crate::artifact_plan::plan_write(conn, workspace, case_id, path, content, false, now)
        }
        Command::AppendFile { path, content } => {
            crate::artifact_plan::plan_write(conn, workspace, case_id, path, content, true, now)
        }
        Command::RunExplore(action) => match crate::explore::write_target(action) {
            Some((path, content)) => {
                simple_plan(Some(path), write_fingerprints(workspace, path, content))
            }
            None => simple_plan(None, crate::explore::semantic_fingerprints(parsed)),
        },
        _ => simple_plan(None, crate::explore::semantic_fingerprints(parsed)),
    }
}

fn simple_plan(
    target_path: Option<&str>,
    fingerprints: Fingerprints,
) -> Result<crate::artifact_plan::WritePlan, String> {
    let (prior_fingerprint, intended_fingerprint) = fingerprints?;
    Ok(crate::artifact_plan::WritePlan {
        target_path: target_path.map(str::to_string),
        prior_fingerprint,
        intended_fingerprint,
        targets: Vec::new(),
    })
}

fn write_fingerprints(workspace: &Path, path: &str, content: &str) -> Fingerprints {
    let full =
        lkjagent_effects::workspace::resolve(workspace, path).map_err(|error| error.to_string())?;
    let prior_fingerprint = if full.exists() {
        match fs::read(&full) {
            Ok(bytes) => stable_fingerprint(&Some(bytes)).map_err(|error| error.message)?,
            Err(error) => crate::explore::fingerprint_text(&format!("unreadable:{error}"))?,
        }
    } else {
        stable_fingerprint(&Option::<Vec<u8>>::None).map_err(|error| error.message)?
    };
    Ok((
        prior_fingerprint,
        crate::explore::fingerprint_text(content)?,
    ))
}
