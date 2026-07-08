use std::{fs, path::Path};

use lkjagent_core::checks::{CommandFact, FileFact};
use lkjagent_core::engine::{Command, TurnOutcome};
use lkjagent_core::model::{CheckResult, CheckSpec, TaskSnapshot};
use rusqlite::Connection;

pub fn gather_checks(
    workspace: &Path,
    snapshot: &TaskSnapshot,
    step_id: u64,
) -> Result<TurnOutcome, String> {
    let Some(step) = snapshot.steps.iter().find(|step| step.id == step_id) else {
        return Err("check step missing".to_string());
    };
    let mut files: Vec<FileFact> = Vec::new();
    let mut commands: Vec<CommandFact> = Vec::new();
    for spec in &step.checks {
        files.extend(
            lkjagent_effects::checks::gather_files(workspace, spec).map_err(|e| e.to_string())?,
        );
        if let CheckSpec::Command { cmd } = spec {
            commands.push(CommandFact {
                cmd: cmd.clone(),
                success: shell_ok(workspace, cmd)?,
            });
        }
    }
    dedupe_files(&mut files);
    Ok(TurnOutcome::Checks(files, commands))
}

pub fn dispatch_effects(
    conn: &Connection,
    workspace: &Path,
    snapshot: &mut TaskSnapshot,
    commands: &[Command],
    now: &str,
) -> Result<(), String> {
    for command in commands {
        match command {
            Command::WriteFile { path, content } => {
                let (body, units) = write_content(workspace, path, content)?;
                crate::artifact_effects::persist_artifacts(
                    conn, snapshot, path, &body, &units, now,
                )?;
            }
            Command::AppendFile { path, content } => {
                let (body, units) = append_content(workspace, path, content)?;
                crate::artifact_effects::persist_artifacts(
                    conn, snapshot, path, &body, &units, now,
                )?;
            }
            Command::RunExplore(action) => crate::explore::run(conn, workspace, snapshot, action),
            Command::RecordAttempt(_)
            | Command::RecordEvent(_)
            | Command::RecordMemory { .. }
            | Command::RecordChecks { .. }
            | Command::AddSteps(_) => {}
        }
    }
    Ok(())
}

fn shell_ok(workspace: &Path, cmd: &str) -> Result<bool, String> {
    Ok(lkjagent_effects::shell::run(workspace, cmd, 30)
        .map_err(|e| e.to_string())?
        .success())
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

fn write_content(
    workspace: &Path,
    path: &str,
    content: &str,
) -> Result<(String, Vec<lkjagent_core::runtime_artifact::ArtifactUnit>), String> {
    let body = content.to_string();
    write_assembled(workspace, path, &body)
}

fn append_content(
    workspace: &Path,
    path: &str,
    content: &str,
) -> Result<(String, Vec<lkjagent_core::runtime_artifact::ArtifactUnit>), String> {
    let full = lkjagent_effects::workspace::resolve(workspace, path).map_err(|e| e.to_string())?;
    let mut body = if full.exists() {
        fs::read_to_string(full).map_err(|e| e.to_string())?
    } else {
        String::new()
    };
    body.push_str(content);
    write_assembled(workspace, path, &body)
}

fn write_assembled(
    workspace: &Path,
    path: &str,
    body: &str,
) -> Result<(String, Vec<lkjagent_core::runtime_artifact::ArtifactUnit>), String> {
    let (assembled, units) = crate::artifact_effects::assemble_content(path, body)?;
    lkjagent_effects::workspace::write(workspace, path, &assembled)
        .map_err(|error| error.to_string())?;
    Ok((assembled, units))
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
