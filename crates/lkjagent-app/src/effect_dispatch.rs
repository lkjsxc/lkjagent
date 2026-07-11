use std::path::Path;

use lkjagent_core::engine::Command;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{mark_journal, EffectTargetRevision, PreparedEffect};
use rusqlite::Connection;

pub struct DispatchFailure {
    pub error: String,
    pub completed: usize,
    pub failed_current: bool,
    pub recovery_required: bool,
}

struct ApplyFailure {
    error: String,
    recovery_required: bool,
}

pub fn dispatch_effects(
    conn: &Connection,
    workspace: &Path,
    snapshot: &mut TaskSnapshot,
    commands: &[Command],
    effects: &[PreparedEffect],
) -> Result<(), DispatchFailure> {
    let mut attempted = 0;
    for command in commands {
        if !external(command) {
            continue;
        }
        let effect = effects.get(attempted).ok_or_else(|| DispatchFailure {
            error: "prepared effect is missing".to_string(),
            completed: attempted,
            failed_current: false,
            recovery_required: false,
        })?;
        validate_prior(workspace, effect).map_err(|error| DispatchFailure {
            error,
            completed: attempted,
            failed_current: true,
            recovery_required: false,
        })?;
        let completed = attempted;
        attempted += 1;
        apply(conn, workspace, snapshot, effect, command).map_err(|failure| DispatchFailure {
            error: failure.error,
            completed,
            failed_current: true,
            recovery_required: failure.recovery_required,
        })?;
    }
    Ok(())
}

pub fn mark_effects(
    conn: &Connection,
    effects: &[PreparedEffect],
    state: &str,
    now: &str,
) -> Result<(), String> {
    let tx = conn
        .unchecked_transaction()
        .map_err(|error| error.to_string())?;
    for effect in effects {
        mark_journal(&tx, &effect.journal_id, state, now).map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())
}

fn external(command: &Command) -> bool {
    matches!(
        command,
        Command::WriteFile { .. } | Command::AppendFile { .. } | Command::RunExplore(_)
    )
}

fn validate_prior(workspace: &Path, effect: &PreparedEffect) -> Result<(), String> {
    if !effect.targets.is_empty() {
        for target in &effect.targets {
            validate_target(workspace, target, &target.prior_fingerprint)?;
        }
        return Ok(());
    }
    let Some(path) = effect.target_path.as_deref() else {
        return Ok(());
    };
    let fingerprint = stable_fingerprint(&crate::artifact_effects::read_optional(workspace, path)?)
        .map_err(|error| error.message)?;
    if fingerprint == effect.prior_fingerprint {
        Ok(())
    } else {
        Err(format!("prior fingerprint changed for {path}"))
    }
}

#[rustfmt::skip]
fn apply(conn: &Connection, workspace: &Path, snapshot: &mut TaskSnapshot,
    effect: &PreparedEffect, command: &Command) -> Result<(), ApplyFailure> {
    match command {
        Command::WriteFile { .. } | Command::AppendFile { .. } => apply_targets(workspace, &effect.targets),
        Command::RunExplore(action) => crate::explore::run(conn, workspace, snapshot, action)
            .map_err(|error| ApplyFailure { error, recovery_required: false }),
        Command::RecordAttempt(_) | Command::RecordEvent(_) | Command::RecordMemory { .. }
        | Command::RecordChecks { .. } | Command::AddSteps(_) => Ok(()),
    }
}

fn apply_targets(workspace: &Path, targets: &[EffectTargetRevision]) -> Result<(), ApplyFailure> {
    if targets.is_empty() {
        return Err(ApplyFailure {
            error: "prepared write targets are missing".to_string(),
            recovery_required: false,
        });
    }
    let mut applied = Vec::new();
    for target in targets {
        let result = if target.role == "parts-membership" {
            validate_target(workspace, target, &target.prior_fingerprint)
        } else {
            crate::effect_files::apply_revision(
                workspace,
                &target.path,
                &target.prior_bytes,
                &target.intended_bytes,
            )
        };
        if let Err(error) = result {
            let current = restore_uncertain(workspace, target);
            let previous = rollback_targets(workspace, &applied);
            let recovery_required = current.is_err() || previous.is_err();
            let detail = current
                .err()
                .into_iter()
                .chain(previous.err())
                .collect::<Vec<_>>()
                .join("; ");
            return Err(ApplyFailure {
                error: if detail.is_empty() {
                    error
                } else {
                    format!("{error}; bundle rollback failed: {detail}")
                },
                recovery_required,
            });
        }
        if target.role != "parts-membership" {
            applied.push(target);
        }
    }
    Ok(())
}

fn restore_uncertain(workspace: &Path, target: &EffectTargetRevision) -> Result<(), String> {
    if target.role == "parts-membership" {
        return Ok(());
    }
    let actual = lkjagent_store::row_support::target_fingerprint(workspace, target)
        .map_err(|error| error.to_string())?;
    if actual == target.prior_fingerprint {
        Ok(())
    } else if actual == target.intended_fingerprint {
        crate::effect_files::apply_revision(
            workspace,
            &target.path,
            &target.intended_bytes,
            &target.prior_bytes,
        )
    } else {
        Err(format!("uncertain target bytes at {}", target.path))
    }
}

fn rollback_targets(workspace: &Path, targets: &[&EffectTargetRevision]) -> Result<(), String> {
    for target in targets.iter().rev() {
        crate::effect_files::apply_revision(
            workspace,
            &target.path,
            &target.intended_bytes,
            &target.prior_bytes,
        )?;
    }
    Ok(())
}

fn validate_target(
    workspace: &Path,
    target: &EffectTargetRevision,
    expected: &str,
) -> Result<(), String> {
    let fingerprint = lkjagent_store::row_support::target_fingerprint(workspace, target)
        .map_err(|error| error.to_string())?;
    if fingerprint == expected {
        Ok(())
    } else {
        Err(format!("target fingerprint changed for {}", target.path))
    }
}
