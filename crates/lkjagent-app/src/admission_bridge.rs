use std::{fs, path::Path};

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

pub fn persist_tool_admissions(
    conn: &Connection,
    workspace: &Path,
    decision: &RuntimeDecision,
    commands: &[Command],
    now: &str,
) -> Result<Vec<PreparedEffect>, String> {
    let mut ordinal = existing_admission_count(conn, &decision.id)?;
    let mut prepared = Vec::new();
    for command in commands {
        let Some((mut admission, parsed, model_action)) = admission_for_command(decision, command)?
        else {
            continue;
        };
        if model_action.is_some()
            && admission.status == AdmissionStatus::Admitted
            && repeated_admitted_action(conn, &decision.id, &parsed)?
        {
            admission = repeated_admission(&admission);
        }
        ordinal += 1;
        let id = format!("{}-admission-{ordinal:04}", decision.id);
        if admission.status == AdmissionStatus::Rejected {
            insert_tool_admission(conn, &id, &decision.case_id, &admission, &parsed, now)
                .map_err(|error| error.to_string())?;
            return Err(format!("admission rejected: {}", admission.reason));
        }
        let journal_id = format!("{id}-effect");
        let idempotency_key = format!("{}:{ordinal}", decision.id);
        let (prior_fingerprint, intended_fingerprint) =
            effect_fingerprints(workspace, command, &parsed)?;
        let preparation = EffectPreparation {
            id: &id,
            case_id: &decision.case_id,
            admission: &admission,
            parsed_action_json: &parsed,
            journal_id: &journal_id,
            idempotency_key: &idempotency_key,
            command_ordinal: ordinal,
            target_path: match command {
                Command::WriteFile { path, .. } | Command::AppendFile { path, .. } => Some(path),
                Command::RunExplore(action) => {
                    crate::explore::write_target(action).map(|(path, _)| path)
                }
                _ => None,
            },
            prior_fingerprint: &prior_fingerprint,
            intended_fingerprint: &intended_fingerprint,
            created_at: now,
        };
        prepared.push(
            insert_admission_and_prepare(conn, &preparation).map_err(|error| error.to_string())?,
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
    .map_err(|error| error.to_string())
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
        .map_err(|error| error.to_string())?;
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

fn effect_fingerprints(workspace: &Path, command: &Command, parsed: &str) -> Fingerprints {
    match command {
        Command::WriteFile { path, content } => {
            write_fingerprints(workspace, path, content, false, true)
        }
        Command::AppendFile { path, content } => {
            write_fingerprints(workspace, path, content, true, true)
        }
        Command::RunExplore(action) => match crate::explore::write_target(action) {
            Some((path, content)) => write_fingerprints(workspace, path, content, false, false),
            None => crate::explore::semantic_fingerprints(parsed),
        },
        _ => crate::explore::semantic_fingerprints(parsed),
    }
}

fn write_fingerprints(
    workspace: &Path,
    path: &str,
    content: &str,
    append: bool,
    assemble: bool,
) -> Fingerprints {
    let full =
        lkjagent_effects::workspace::resolve(workspace, path).map_err(|error| error.to_string())?;
    let (prior, prior_fingerprint) = if full.exists() {
        match fs::read(&full) {
            Ok(bytes) => {
                let fingerprint =
                    stable_fingerprint(&Some(bytes.clone())).map_err(|error| error.message)?;
                (Some(bytes), fingerprint)
            }
            Err(error) => (
                None,
                crate::explore::fingerprint_text(&format!("unreadable:{error}"))?,
            ),
        }
    } else {
        (
            None,
            stable_fingerprint(&Option::<Vec<u8>>::None).map_err(|error| error.message)?,
        )
    };
    let body = if append {
        let prior = prior
            .as_deref()
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .unwrap_or_default();
        format!("{prior}{content}")
    } else {
        content.to_string()
    };
    let intended = if assemble {
        crate::artifact_effects::assemble_content(path, &body)?.0
    } else {
        body
    };
    Ok((
        prior_fingerprint,
        crate::explore::fingerprint_text(&intended)?,
    ))
}
