use std::path::Path;

use lkjagent_core::checks::{CommandFact, FileFact};
use lkjagent_core::engine::{Command, TurnOutcome};
use lkjagent_core::model::{CheckSpec, TaskSnapshot};
use lkjagent_core::runtime_artifact::{
    artifact_fingerprint, assemble_checked_units, ArtifactUnit, DEFAULT_UNIT_TARGET_TOKENS,
};
use lkjagent_store::artifact_rows::{insert_artifact, ArtifactRow};
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
) -> Result<(), String> {
    for command in commands {
        match command {
            Command::WriteFile { path, content } => {
                let body = write_content(workspace, path, content)?;
                persist_artifacts(conn, snapshot, path, content, &body)?;
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

fn persist_artifacts(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    path: &str,
    unit: &str,
    body: &str,
) -> Result<(), String> {
    let file_id = format!("task-{}-file-{}", snapshot.task.id, safe_id(path));
    let unit_id = format!("{file_id}-unit-{:04}", snapshot.task.budget_used + 1);
    insert_artifact(
        conn,
        &artifact_row(snapshot, &file_id, "file", path, body, None, "{}")?,
    )
    .map_err(|error| error.to_string())?;
    let metadata = serde_json::json!({"target_tokens": DEFAULT_UNIT_TARGET_TOKENS}).to_string();
    insert_artifact(
        conn,
        &artifact_row(
            snapshot,
            &unit_id,
            "unit",
            path,
            unit,
            Some(file_id),
            &metadata,
        )?,
    )
    .map_err(|error| error.to_string())
}

fn artifact_row(
    snapshot: &TaskSnapshot,
    id: &str,
    kind: &str,
    path: &str,
    content: &str,
    parent_artifact_id: Option<String>,
    metadata_json: &str,
) -> Result<ArtifactRow, String> {
    Ok(ArtifactRow {
        id: id.to_string(),
        case_id: snapshot.task.id.to_string(),
        kind: kind.to_string(),
        path: path.to_string(),
        fingerprint: artifact_fingerprint(path, content).map_err(|error| error.message)?,
        parent_artifact_id,
        metadata_json: metadata_json.to_string(),
        created_at: "turn".to_string(),
    })
}

fn safe_id(path: &str) -> String {
    path.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

fn write_content(workspace: &Path, path: &str, content: &str) -> Result<String, String> {
    let full = workspace.join(path);
    let body = if path.contains("/manuscript/chapter-") && full.exists() {
        let current = std::fs::read_to_string(&full).map_err(|error| error.to_string())?;
        format!("{current}\n\n{content}")
    } else {
        content.to_string()
    };
    let assembled = assembled_body(path, &body)?;
    lkjagent_effects::workspace::write(workspace, path, &assembled)
        .map_err(|error| error.to_string())?;
    Ok(assembled)
}

fn assembled_body(path: &str, body: &str) -> Result<String, String> {
    let mut unit = ArtifactUnit::new("effect-unit-1", path, 1);
    unit.content = body.to_string();
    unit.check_passed = true;
    assemble_checked_units(path, &[unit])
        .map(|artifact| artifact.content)
        .map_err(|error| error.message)
}
