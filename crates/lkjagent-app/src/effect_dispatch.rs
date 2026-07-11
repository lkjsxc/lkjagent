use std::{fs, path::Path};

use lkjagent_core::engine::Command;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{mark_journal, PreparedEffect};
use rusqlite::Connection;

pub struct DispatchFailure {
    pub error: String,
    pub attempted: usize,
}

pub fn dispatch_effects(
    conn: &Connection,
    workspace: &Path,
    snapshot: &mut TaskSnapshot,
    commands: &[Command],
    effects: &[PreparedEffect],
    now: &str,
) -> Result<(), DispatchFailure> {
    let mut attempted = 0;
    for command in commands {
        if !external(command) {
            continue;
        }
        let effect = effects.get(attempted).ok_or_else(|| DispatchFailure {
            error: "prepared effect is missing".to_string(),
            attempted,
        })?;
        validate_prior(workspace, effect).map_err(|error| DispatchFailure { error, attempted })?;
        attempted += 1;
        apply(conn, workspace, snapshot, command, now)
            .map_err(|error| DispatchFailure { error, attempted })?;
    }
    Ok(())
}

pub fn mark_effects(
    conn: &Connection,
    effects: &[PreparedEffect],
    state: &str,
    now: &str,
) -> Result<(), String> {
    for effect in effects {
        mark_journal(conn, &effect.journal_id, state, now).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn external(command: &Command) -> bool {
    matches!(
        command,
        Command::WriteFile { .. } | Command::AppendFile { .. } | Command::RunExplore(_)
    )
}

fn validate_prior(workspace: &Path, effect: &PreparedEffect) -> Result<(), String> {
    let Some(path) = effect.target_path.as_deref() else {
        return Ok(());
    };
    let full =
        lkjagent_effects::workspace::resolve(workspace, path).map_err(|error| error.to_string())?;
    let bytes = if full.exists() {
        Some(fs::read(full).map_err(|error| error.to_string())?)
    } else {
        None
    };
    let fingerprint = stable_fingerprint(&bytes).map_err(|error| error.message)?;
    if fingerprint == effect.prior_fingerprint {
        Ok(())
    } else {
        Err(format!("prior fingerprint changed for {path}"))
    }
}

fn apply(
    conn: &Connection,
    workspace: &Path,
    snapshot: &mut TaskSnapshot,
    command: &Command,
    now: &str,
) -> Result<(), String> {
    match command {
        Command::WriteFile { path, content } => {
            persist_write(conn, workspace, snapshot, path, content, false, now)
        }
        Command::AppendFile { path, content } => {
            persist_write(conn, workspace, snapshot, path, content, true, now)
        }
        Command::RunExplore(action) => crate::explore::run(conn, workspace, snapshot, action),
        Command::RecordAttempt(_)
        | Command::RecordEvent(_)
        | Command::RecordMemory { .. }
        | Command::RecordChecks { .. }
        | Command::AddSteps(_) => Ok(()),
    }
}

fn persist_write(
    conn: &Connection,
    workspace: &Path,
    snapshot: &TaskSnapshot,
    path: &str,
    content: &str,
    append: bool,
    now: &str,
) -> Result<(), String> {
    let body = if append {
        let full = lkjagent_effects::workspace::resolve(workspace, path)
            .map_err(|error| error.to_string())?;
        let mut body = if full.exists() {
            fs::read_to_string(full).map_err(|error| error.to_string())?
        } else {
            String::new()
        };
        body.push_str(content);
        body
    } else {
        content.to_string()
    };
    let (body, units) = crate::artifact_effects::assemble_content(path, &body)?;
    crate::artifact_effects::sync_part_files(workspace, path, &units)?;
    lkjagent_effects::workspace::write(workspace, path, &body)
        .map_err(|error| error.to_string())?;
    crate::artifact_effects::persist_artifacts(conn, snapshot, path, &body, &units, now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lkjagent_core::classify::instantiate;

    #[test]
    fn changed_prior_blocks_workspace_write() -> Result<(), String> {
        let workspace = std::env::temp_dir().join(format!("lkjagent-prior-{}", std::process::id()));
        std::fs::create_dir_all(&workspace).map_err(|error| error.to_string())?;
        std::fs::write(workspace.join("note.md"), "changed").map_err(|error| error.to_string())?;
        let mut snapshot = instantiate(1, "write note");
        let conn = Connection::open_in_memory().map_err(|error| error.to_string())?;
        let effect = PreparedEffect {
            admission_id: "admission".to_string(),
            journal_id: "journal".to_string(),
            command_ordinal: 1,
            target_path: Some("note.md".to_string()),
            prior_fingerprint: stable_fingerprint(&Some(b"before".to_vec()))
                .map_err(|error| error.message)?,
            intended_fingerprint: String::new(),
            effect_name: "native.write_file".to_string(),
        };
        let failure = match dispatch_effects(
            &conn,
            &workspace,
            &mut snapshot,
            &[Command::WriteFile {
                path: "note.md".to_string(),
                content: "after".to_string(),
            }],
            &[effect],
            "now",
        ) {
            Err(failure) => failure,
            Ok(()) => return Err("changed prior was dispatched".to_string()),
        };
        assert_eq!(failure.attempted, 0);
        assert_eq!(
            std::fs::read_to_string(workspace.join("note.md")).map_err(|error| error.to_string())?,
            "changed"
        );
        Ok(())
    }
}
