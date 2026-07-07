use std::path::Path;

use lkjagent_core::checks::{CommandFact, FileFact};
use lkjagent_core::engine::{Command, TurnOutcome};
use lkjagent_core::model::{CheckSpec, TaskSnapshot};
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
                let (body, units) = write_content(workspace, path, content)?;
                crate::artifact_effects::persist_artifacts(conn, snapshot, path, &body, &units)?;
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
    let (assembled, units) = crate::artifact_effects::assemble_content(path, &body)?;
    lkjagent_effects::workspace::write(workspace, path, &assembled)
        .map_err(|error| error.to_string())?;
    Ok((assembled, units))
}
