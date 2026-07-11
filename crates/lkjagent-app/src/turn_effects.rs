use std::path::Path;

use lkjagent_core::checks::{CommandFact, FileFact};
use lkjagent_core::engine::{Command, TurnOutcome};
use lkjagent_core::model::{CheckResult, CheckSpec, TaskSnapshot};
use lkjagent_core::runtime_admission::{AdmissionStatus, ToolAdmission};
use lkjagent_core::runtime_context::{
    contamination_for_observation, redact_sensitive_owner_data, ContaminationClass,
};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{
    insert_admission_and_prepare, mark_journal, EffectPreparation, PreparedEffect,
};
use lkjagent_store::observation_rows::{settle_effect_observation, ObservationRow};
use rusqlite::Connection;

#[rustfmt::skip]
pub fn gather_checks(conn: &mut Connection, workspace: &Path, snapshot: &TaskSnapshot, step_id: u64, decision: &RuntimeDecision, now: &str) -> Result<TurnOutcome, String> {
    let step = snapshot.steps.iter().find(|step| step.id == step_id).ok_or_else(|| "check step missing".to_string())?;
    let shell = step.checks.iter().filter_map(|spec| match spec { CheckSpec::Command { cmd } => Some(cmd.clone()), _ => None }).collect::<Vec<_>>();
    let effects = prepare_shell_checks(conn, decision, &shell, now)?;
    let (mut files, mut commands, mut shell_index): (Vec<FileFact>, Vec<CommandFact>, usize) = (Vec::new(), Vec::new(), 0);
    for spec in &step.checks {
        files.extend(lkjagent_effects::checks::gather_files(workspace, spec).map_err(|error| error.to_string())?);
        if let CheckSpec::Command { cmd } = spec {
            let effect = effects.get(shell_index).ok_or_else(|| "prepared shell check is missing".to_string())?;
            commands.push(CommandFact { cmd: cmd.clone(), success: run_shell_check(conn, workspace, decision, effect, cmd, now)? });
            shell_index += 1;
        }
    }
    dedupe_files(&mut files);
    Ok(TurnOutcome::Checks(files, commands))
}

#[rustfmt::skip]
fn prepare_shell_checks(conn: &Connection, decision: &RuntimeDecision, commands: &[String], now: &str) -> Result<Vec<PreparedEffect>, String> {
    let mut ordinal: i64 = conn.query_row("SELECT COUNT(*) FROM tool_admissions WHERE decision_id = ?1", [&decision.id], |row| row.get(0)).map_err(|error| error.to_string())?;
    let view = decision.tool_view_fingerprint().map_err(|error| error.message)?;
    let mut effects = Vec::new();
    for command in commands {
        ordinal += 1;
        let admission = ToolAdmission { decision_id: decision.id.clone(), tool_view_fingerprint: view.clone(), action_tool: "shell.run".to_string(), status: AdmissionStatus::Admitted, reason: "harness admitted command check".to_string() };
        let parsed = serde_json::json!({"command": command, "source": "check"}).to_string();
        let prior = stable_fingerprint(&(command, "prepared")).map_err(|error| error.message)?;
        let intended = stable_fingerprint(&(command, "attempted")).map_err(|error| error.message)?;
        let id = format!("{}-admission-{ordinal:04}", decision.id);
        effects.push(insert_admission_and_prepare(conn, &EffectPreparation { id: &id, case_id: &decision.case_id, admission: &admission, parsed_action_json: &parsed, journal_id: &format!("{id}-effect"), idempotency_key: &format!("{}:{ordinal}", decision.id), command_ordinal: ordinal, target_path: None, prior_fingerprint: &prior, intended_fingerprint: &intended, targets: &[], created_at: now }).map_err(|error| error.to_string())?);
    }
    Ok(effects)
}

fn run_shell_check(
    conn: &mut Connection,
    workspace: &Path,
    decision: &RuntimeDecision,
    effect: &PreparedEffect,
    command: &str,
    now: &str,
) -> Result<bool, String> {
    mark_journal(conn, &effect.journal_id, "applying", now).map_err(|error| error.to_string())?;
    let report = match lkjagent_effects::shell::run(workspace, command, 30) {
        Ok(report) => report,
        Err(error) => {
            let content = error.to_string();
            settle_shell(conn, decision, effect, "error", &content, "failed", now)?;
            return Ok(false);
        }
    };
    let content = format!(
        "command={command}\nexit_code={:?}\ntimed_out={}\nsuccess={}\noutput:\n{}",
        report.exit_code,
        report.timed_out,
        report.success(),
        report.output
    );
    settle_shell(conn, decision, effect, "ok", &content, "committed", now)?;
    Ok(report.success())
}

#[rustfmt::skip]
fn settle_shell(conn: &mut Connection, decision: &RuntimeDecision, effect: &PreparedEffect, status: &str, content: &str, state: &str, now: &str) -> Result<(), String> {
    let contamination = contamination_for_observation("shell.run", status, content);
    let stored = if contamination == ContaminationClass::SensitiveOwnerData { redact_sensitive_owner_data(content) } else { content.to_string() };
    let row = ObservationRow { id: format!("{}-check-observation-{:04}", decision.id, effect.command_ordinal), case_id: decision.case_id.clone(), decision_id: decision.id.clone(), admission_id: Some(effect.admission_id.clone()), effect_name: "shell.run".to_string(), status: status.to_string(), content: lkjagent_effects::observation::bound(&stored, lkjagent_effects::shell::SHELL_OUTPUT_BYTES), artifact_refs_json: "[]".to_string(), contamination_class: format!("{:?}", contamination), created_at: now.to_string() };
    settle_effect_observation(conn, &effect.journal_id, state, &row).map_err(|error| error.to_string())
}

fn dedupe_files(files: &mut Vec<FileFact>) {
    let mut seen = Vec::new();
    files.retain(|fact| {
        if seen.contains(&fact.path) {
            false
        } else {
            seen.push(fact.path.clone());
            true
        }
    });
}

pub fn tag_check_evidence(
    conn: &Connection,
    snapshot: &mut TaskSnapshot,
    commands: &mut [Command],
    decision_id: &str,
) -> Result<(), String> {
    for command in commands {
        let Command::RecordChecks {
            step_id,
            decision_id: slot,
            results,
        } = command
        else {
            continue;
        };
        *slot = Some(decision_id.to_string());
        tag_results(conn, snapshot, *step_id, results, decision_id)?;
    }
    for result in &mut snapshot.check_results {
        if result.decision_id.is_none() {
            result.decision_id = Some(decision_id.to_string());
        }
    }
    Ok(())
}

fn tag_results(
    conn: &Connection,
    snapshot: &mut TaskSnapshot,
    step_id: u64,
    results: &mut [CheckResult],
    decision_id: &str,
) -> Result<(), String> {
    let specs = snapshot
        .steps
        .iter()
        .find(|step| step.id == step_id)
        .map(|step| step.checks.clone())
        .unwrap_or_default();
    for (index, result) in results.iter_mut().enumerate() {
        result.decision_id = Some(decision_id.to_string());
        if result.params.is_none() {
            result.params = specs.get(index).cloned();
        }
        if result.artifact_refs.is_empty() {
            if let Some(spec) = result.params.as_ref() {
                result.artifact_refs = refs_for_spec(conn, snapshot.task.id, spec)?;
            }
        }
        tag_snapshot_result(snapshot, result);
    }
    Ok(())
}

fn refs_for_spec(conn: &Connection, task_id: u64, spec: &CheckSpec) -> Result<Vec<String>, String> {
    lkjagent_store::artifact_rows::refs_for_spec(conn, task_id as i64, spec)
        .map_err(|error| error.to_string())
}

fn tag_snapshot_result(snapshot: &mut TaskSnapshot, result: &CheckResult) {
    for item in snapshot.check_results.iter_mut().rev() {
        if same_row(item, result) {
            *item = result.clone();
            return;
        }
    }
}

fn same_row(left: &CheckResult, right: &CheckResult) -> bool {
    let same_params =
        left.params == right.params || left.params.is_none() || right.params.is_none();
    left.name == right.name && same_params && left.measured == right.measured
}
